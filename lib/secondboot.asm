;;; For x86 Architecture
;;; @author k-masatany

	org	0xc200

INITOS	EQU	0x00280000	; OS本体部分の転送アドレス
DSKCAC	EQU	0x00100000	; キャッシュのアドレス
DSKCAC0	EQU	0x00008000	; キャッシュのアドレス(iplが転送したもの)
VRAM	EQU	0x000B8000	; グラフィックバッファ（テキストモード）の開始番地

;;; BOOTINFO関連
CYLS	EQU	0x0ff0		; 読み込んだシリンダ数を記録(iplが書き込んだ)
LEDS	EQU	0x0ff1

; テキストモード固定
	mov	al, 0x03
	mov	ah, 0x00
	int	0x10

keystatus:
	;; キーボードの状態を取得
	mov	ah,0x02
	int	0x16
	mov	[LEDS],al	; LED点灯状態をメモ

	;; PICが一切の割り込みを受け付けないようにする
	mov	al,0xff
	out	0x21,al		; io出力
	nop			; out命令を連続させるとうまくいかない機種があるため
	cli			; CPUレベルでも割り込み禁止

	;; A20GATE信号線をONにする(1MB以上のメモリにアクセスするため)
	call	waitkbdout	; 制御命令を受けられるまで待つ
	mov	al,0xd1		; 0xd1 = キーボードコントローラのおまけ出力ポートに出力
	out	0x64,al		; 0x64 = PORT_KEYCMD

	call	waitkbdout	; 制御命令を受けられるまで待つ
	mov	al,0x1f		; 0x1f : A20信号線をONにするための設定
	out	0x60,al		; 0x60 = PORT_KEYDAT

	call	waitkbdout	; A20GATEの設定が終わるまで待つ

	;; プロテクトモードに移行

	lgdt	[GDTR0]		; 暫定GDTを設定

	mov	eax,cr0
	and	eax,0x7fffffff	; bit31を0にする(ページングを禁止するため)
	or	eax,0x00000001	; bit0を1にする(プロテクトモードへ移行)
	mov	cr0,eax

	jmp	pipelineflush	; パイプラインに先読みした命令をリセット

pipelineflush:
	;; セグメントレジスタの値を設定しなおす(0x0008に設定)
	mov	ax,1*8		; 2つ目のセグメント(gdt + 1) = 0x0000 + 1 * 8
	mov	ds,ax
	mov	es,ax
	mov	fs,ax
	mov	gs,ax
	mov	ss,ax

	;; os本体の転送
	mov	esi,initOS	; 転送元
	mov	edi,INITOS	; 転送先
	mov	ecx,512*1024/4	; 512キロバイト(512*1024)を4で割る(ダブルワード単位なので)
	call	memcpy

	;; ディスクデータを転送
	;; まずはipl部分の転送
	mov	esi,0x7c00	; 転送元
	mov	edi,DSKCAC	; 転送先
	mov	ecx,512/4	; 512バイト転送
	call	memcpy

	;; 残りのディスクキャッシュを転送
	mov	esi,DSKCAC0+512	; 転送元
	mov	edi,DSKCAC+512	; 転送先
	mov	ecx,0
	mov	cl,BYTE [CYLS]	; シリンダ数を読み込み
	IMUL	ecx,512*18*2/4	; 1シリンダ分の大きさ(512バイト*18セクタ*2ヘッド分)*10シリンダ(ecxに格納)
	sub	ecx,512/4	; iplの分だけ引く
	call	memcpy

	;; .dataの領域を確保
	mov	ebx,INITOS
	mov	ecx,[ebx+16]	; .dataセクションのサイズを代入
	add	ecx,3		; ecx += 3
	shr	ecx,2		; ecx /= 4(右に2桁シフト->1桁シフトで半分になる)=>転送するサイズ
	jz	skip		; .dataセクションの有無を確認(セクションがあれば.data領域を確保)
	mov	esi,[ebx+20]	; .dataセクションのファイル内の相対アドレス
	add	esi,ebx		; 転送元(ファイルから転送されたructiss.sysの.dataセクション)
	mov	edi,[ebx+12]	; 転送先(.data転送先とスタック領域の境目)
	call	memcpy

skip:
	;; initOSに制御を移す
	mov	esp,[ebx+12]	; スタックポインタを設定(.data転送先とスタック領域の境目)
	jmp	DWORD 2*8:0x0000001b ; .sysのヘッダのjmp命令のある場所へジャンプ(2番目のセグメントの0x001bへジャンプ)

fin:
	hlt
	jmp	fin

;;; サブルーチン関係
waitkbdout:
	in	al,0x64		; 装置番号0x64から読み込み
	and	al,0x02		; 下から2ビット目の以外はマスク
	in	al,0x60		; から読みして受信バッファからキーボードに溜ってるデータを出す
	jnz	waitkbdout	; 下から2ビット目が0
	ret

memcpy:
	mov	eax,[esi]
	add	esi,4

	mov	[edi],eax
	add	edi,4

	sub	ecx,1
	jnz	memcpy

	ret

	ALIGNB	16,DB 0		; GDT0のラベルのアドレスが8の倍数になるように、キリのよいところまでDB 0で埋める

;;; 定数
GDT0:
	times	8 DB 0		; ヌルセレクタ(gdtの0番)
	DW	0xffff,0x0000,0x9200,0x00cf ; 1番(asmhead用,読み書き可能セグメント)
	DW	0xffff,0x0000,0x9a28,0x0047 ; 2番(osmain用,実行可能セグメント)

	DW	0

GDTR0:
	DW	8*3-1		; lgdt命令でレジスタに転送されるGDTのテーブルサイズ
	DD	GDT0		; lgdt命令でレジスタに転送されるGDTの先頭アドレス

	ALIGNB	16,DB 0

;;; os本体へのラベル
initOS:
