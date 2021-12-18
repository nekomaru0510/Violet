# TODOリスト

## 優先度高め
* README.mdの整理

* スクリプトの整理
* デバッグ用スクリプトの整理 ... 
* コンテナの切り替えやすさ向上(コンテナ内も)

* QEMU virt用のコンテナ作成
* 動作モードによってアクセスするCSRを変更(特にブート周り)
* 動作環境ごとにビルドするものをcargoで切り替えられるように(今のところ、ちょっと手間が多い)

## 優先度低め

1こめ
b *0xffffffff806f6ed0
set $medeleg=0xb0b109

// opensbi mtvec
b *0x800004ec
<- mepcどうなってる？ ... 変わってる
b sbi_trap_handler
// opensbi mret
b *0x800005f2

./arch/riscv/include/asm/sbi.h:58
SBI_CALL_1(SBI_SET_TIMER, stime_value);
```
#0  sbi_set_timer (stime_value=<optimized out>) at ./arch/riscv/include/asm/sbi.h:58
#1  riscv_clock_next_event (delta=<optimized out>, ce=<optimized out>) at drivers/clocksource/timer-riscv.c:23
#2  0xffffffff800c8564 in clockevents_program_event (dev=0xffffffff86dec100, expires=<optimized out>, force=false)
    at kernel/time/clockevents.c:334
#3  0xffffffff800c8d9c in tick_setup_periodic (broadcast=<optimized out>, dev=<optimized out>)
    at kernel/time/tick-common.c:171
#4  tick_setup_periodic (dev=0xffffffff86dec100, broadcast=<optimized out>) at kernel/time/tick-common.c:148
```
多分、下記の理由からsbicallがhypervisorに投げてる
* CAUSE_VIRTUAL_SUPERVISOR_ECALL(0xa)を解釈していない
* medelegでHypervisorに譲渡している。
2こめ
sepc           0xffffffff806f6ed0
状況
scause         0xa      10
medeleg        0xb0b109 11579657


OpenSBIにSBICallを横流しする
OpenSBIの割込み取得処理を真似する

hyp
```
_trap_handler_rv32_hyp:
	TRAP_SAVE_AND_SETUP_SP_T0

	TRAP_SAVE_MEPC_MSTATUS 1

	TRAP_SAVE_GENERAL_REGS_EXCEPT_SP_T0

	TRAP_CALL_C_ROUTINE

_trap_exit_rv32_hyp:
	TRAP_RESTORE_GENERAL_REGS_EXCEPT_A0_T0

	TRAP_RESTORE_MEPC_MSTATUS 1

	TRAP_RESTORE_A0_T0

	mret
```



```
_trap_handler:
	TRAP_SAVE_AND_SETUP_SP_T0

	TRAP_SAVE_MEPC_MSTATUS 0

	TRAP_SAVE_GENERAL_REGS_EXCEPT_SP_T0

	TRAP_CALL_C_ROUTINE

_trap_exit:
	TRAP_RESTORE_GENERAL_REGS_EXCEPT_A0_T0

	TRAP_RESTORE_MEPC_MSTATUS 0

	TRAP_RESTORE_A0_T0

	mret
```

	/* Swap TP and MSCRATCH */
	csrrw	tp, CSR_MSCRATCH, tp
	/* Save T0 in scratch space */
	REG_S	t0, SBI_SCRATCH_TMP0_OFFSET(tp)

tp ... スレッドポインタ

そもそも割込みでレジスタにどんな変化があるか
pcがジャンプ先に

2つめ
```
Breakpoint 1, sbi_remote_sfence_vma (size=<optimized out>, start=<optimized out>, hart_mask=<optimized out>) at ./arch/riscv/include/asm/sbi.h:86
86              SBI_CALL_3(SBI_REMOTE_SFENCE_VMA, hart_mask, start, size);
```
b *0xffffffff8004a3d4

3つめ
```
Breakpoint 3, flush_tlb_mm (mm=<optimized out>) at ./arch/riscv/include/asm/sbi.h:86
86              SBI_CALL_3(SBI_REMOTE_SFENCE_VMA, hart_mask, start, size);
```
b *0xffffffff8004a328


4
0xffffffff80049f50


5
$3 = 0xffffffff80049f50

SBI callの呼び出し規約
Linuxのシステムコールとあわせてるらしい
a7 ... EID(RV32Eはt0)
a6 ... FID

戻り値 ... a0とa1のペア
a0 ... error
a1 ... value

```
void sbi_remote_sfence_vma(const unsigned long *hart_mask,
                           unsigned long start,
                           unsigned long size)
```



sbi_ecall_legacy_handler関数での処理は失敗？
sbi_load_hart_mask_unprivでret=-1007(SBI_ETRAP)が返される
causeレジスタを見てる？
sbi_trap_redirect(regs, &trap);


シェルが起動しない
引数は渡されてる。上書きも多分されてない。
カーネルの起動メッセージも時間以外はまったく一緒
しいていうなら、最後の`/bin/sh: can't access tty; job control turned off`くらい
violetのメモリがLinuxに上書きされていないかは気になる

デバッグしてみると、0x800004e8で停止している。
(gdb) p/x $mepc
$2 = 0x80104b10


なぜかスーパーバイザタイマ割込みが発生
正常系でもタイマ割込みが発生している。
タイマ割込みはどう対処してる？
sbi_timer_process();

```
void sbi_timer_process(void)
{
	csr_clear(CSR_MIE, MIP_MTIP);
	csr_set(CSR_MIP, MIP_STIP);
}
```

ユーザアプリのキックがこちらに飛んできていると良そう
(レジスタが0ばかりできれいすぎるので、)
hidelegが0x80に。。。？


sfence.vmaの依頼をopensbiに投げるとハイパーバイザにリダイレクトされる
sfence.vmaは、ハイパーバイザでやる必要あり？
xvisorだとSBI_EXT_RFENCE(0x52464E43)を指定してSBIに投げてる

_sbi_rfence(SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA,
		     hart_mask, start, size, 0, 0);
__sbi_rfence_v02_real(fid, hmask, hbase,
						start, size, arg4);
ret = sbi_ecall(SBI_EXT_RFENCE, fid, hmask, hbase,
				start, size, 0, 0);

a7 ... 0x52464E43
a6 ... 0x1
a0 ... 0
a1 ... 0
a2 ... start
a3 ... size


```
void sbi_remote_sfence_vma(const unsigned long *hart_mask,
                           unsigned long start,
                           unsigned long size)
```

```
a0             0xffffffe00764ba88       -137314911608
a1             0x3ffffff000     274877902848
a2             0x1000   4096
a3             0x0      0
a4             0x8      8
a5             0xffffffe00080ddd8       -137430508072
a6             0xffffffe000894f98       -137429954664
a7             0x6      6
```

fence.i
sfence.vma rs1 rs2 rd=0
命令実行前のメモリアクセスが完了することを保障する。
rs1 ... vaddr 仮想アドレス
rs2 ... asid アドレス空間の指定？



hfence.vvma


hfence.gvma


#define MIP_MTIP			(_UL(1) << IRQ_M_TIMER)
#define IRQ_M_TIMER			7

initプロセス起動処理手順
run_init_process関数に失敗してパニックが起きてる。
パニックしてる？例外飛んでるのはOKっぽい



デバッグできてないのではなく、アドレスが変わっている。。。？
途中まではアドレスも正しいはず
Linux単体でもデバッグできてないので、、、
CONFIG見直したらいけた

```
1047            return do_execve(getname_kernel(init_filename),
(gdb) p init_filename
$1 = 0xffffffe007c3a955 "/bin/sh"
(gdb) p argv
argv          argv_free     argv_init     argv_split    argv_split.c  
(gdb) p argv_init 
$2 = {0xffffffe007c3a955 "/bin/sh", 0x0 <repeats 33 times>}
(gdb) p envp
envp       envp_idx   envp_init  
(gdb) p envp_init 
$3 = {0xffffffe000733c48 "HOME=/", 0xffffffe000733c50 "TERM=linux", 0x0 <repeats 32 times>}
(gdb)  
```

```
(gdb) s
__do_execve_file (fd=-100, filename=0xffffffe007680000, argv=..., envp=..., flags=0, file=0x0) at fs/exec.c:1735
1735            if ((current->flags & PF_NPROC_EXCEEDED) &&
(gdb) bt
#0  __do_execve_file (fd=-100, filename=0xffffffe007680000, argv=..., envp=..., flags=0, file=0x0) at fs/exec.c:1735
#1  0xffffffe00011a50e in do_execveat_common (flags=<optimized out>, envp=..., argv=..., filename=<optimized out>, fd=<optimized out>)
    at fs/exec.c:1885
#2  do_execve (filename=<optimized out>, __argv=<optimized out>, __envp=<optimized out>) at fs/exec.c:1885
#3  0xffffffe0000359ba in run_init_process (init_filename=0xffffffe007c3a955 "/bin/sh") at init/main.c:1047
#4  0xffffffe00060e9fc in kernel_init (unused=<optimized out>) at init/main.c:1128
#5  0xffffffe000036814 in handle_exception () at arch/riscv/kernel/entry.S:244
Backtrace stopped: frame did not save the PC
(gdb) 
```


ユーザ空間にはいってそう
ユーザ空間入った後、　即座に例外発生
scause 12 ... 命令フェッチ まあ、ページフォルトだろう。
あれ、initに入ってる。。。



sepc           0xffffffe000038338       -137438723272
scause         0xa      10

3回目
sscratch       0xffffffe00764bab0       -137314911568
sepc           0x10328  66344
scause         0x8000000000000005       -9223372036854775803

initの開始アドレス？
スーパーバイザタイマ割込み

hidelegがおかしい

mstatusにある？mppとmpv

うまくhidelegにかけてない。

hideleg ... 1を書いた例外がフックされる？