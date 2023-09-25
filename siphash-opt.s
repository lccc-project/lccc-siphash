	.text
	.file	"siphash_opt.279c6ca2b954e9d1-cgu.0"
	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb10c6415c951d1d1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb10c6415c951d1d1E,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb10c6415c951d1d1E:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rsi, %rbx
	movq	(%rdi), %r14
	movq	%rsi, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_lower_hex17h7fdbc8ef66bd11a7E@GOTPCREL(%rip)
	testb	%al, %al
	je	.LBB0_1
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u64$GT$3fmt17h01a2b4b48ce68b75E@GOTPCREL(%rip)
.LBB0_1:
	.cfi_def_cfa_offset 32
	movq	%rbx, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_upper_hex17h9e038a705a29e777E@GOTPCREL(%rip)
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	testb	%al, %al
	je	.LBB0_4
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u64$GT$3fmt17hc3bcd5f9578ca1fbE@GOTPCREL(%rip)
.LBB0_4:
	.cfi_def_cfa_offset 32
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h7c8758aea31e7638E@GOTPCREL(%rip)
.Lfunc_end0:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb10c6415c951d1d1E, .Lfunc_end0-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb10c6415c951d1d1E
	.cfi_endproc

	.section	".text._ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h93671e49cf0d1c4fE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h93671e49cf0d1c4fE,@function
_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h93671e49cf0d1c4fE:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movq	%rsi, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_lower_hex17h7fdbc8ef66bd11a7E@GOTPCREL(%rip)
	testb	%al, %al
	je	.LBB1_1
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u64$GT$3fmt17h01a2b4b48ce68b75E@GOTPCREL(%rip)
.LBB1_1:
	.cfi_def_cfa_offset 32
	movq	%rbx, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_upper_hex17h9e038a705a29e777E@GOTPCREL(%rip)
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	testb	%al, %al
	je	.LBB1_4
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u64$GT$3fmt17hc3bcd5f9578ca1fbE@GOTPCREL(%rip)
.LBB1_4:
	.cfi_def_cfa_offset 32
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h7c8758aea31e7638E@GOTPCREL(%rip)
.Lfunc_end1:
	.size	_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h93671e49cf0d1c4fE, .Lfunc_end1-_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h93671e49cf0d1c4fE
	.cfi_endproc

	.section	".text._ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE,@function
_ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE:
	.cfi_startproc
	retq
.Lfunc_end2:
	.size	_ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE, .Lfunc_end2-_ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE
	.cfi_endproc

	.section	".text._ZN76_$LT$siphash_opt..siphash..sys..SipHashState$u20$as$u20$core..fmt..Debug$GT$3fmt17h85ce43ed0d4788c0E","ax",@progbits
	.globl	_ZN76_$LT$siphash_opt..siphash..sys..SipHashState$u20$as$u20$core..fmt..Debug$GT$3fmt17h85ce43ed0d4788c0E
	.p2align	4, 0x90
	.type	_ZN76_$LT$siphash_opt..siphash..sys..SipHashState$u20$as$u20$core..fmt..Debug$GT$3fmt17h85ce43ed0d4788c0E,@function
_ZN76_$LT$siphash_opt..siphash..sys..SipHashState$u20$as$u20$core..fmt..Debug$GT$3fmt17h85ce43ed0d4788c0E:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	subq	$16, %rsp
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -16
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	8(%rdi), %r9
	leaq	16(%rdi), %r10
	leaq	24(%rdi), %rdx
	movq	%rdx, 8(%rsp)
	subq	$8, %rsp
	.cfi_adjust_cfa_offset 8
	leaq	.L__unnamed_1(%rip), %r11
	leaq	16(%rsp), %rbx
	leaq	.L__unnamed_2(%rip), %r8
	leaq	.L__unnamed_3(%rip), %rsi
	movl	$12, %edx
	movq	%rax, %rdi
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	%rbx
	.cfi_adjust_cfa_offset 8
	pushq	%r8
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	%r8
	.cfi_adjust_cfa_offset 8
	callq	*_ZN4core3fmt9Formatter25debug_tuple_field4_finish17hb5689f45cb7a16feE@GOTPCREL(%rip)
	addq	$64, %rsp
	.cfi_adjust_cfa_offset -64
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_ZN76_$LT$siphash_opt..siphash..sys..SipHashState$u20$as$u20$core..fmt..Debug$GT$3fmt17h85ce43ed0d4788c0E, .Lfunc_end3-_ZN76_$LT$siphash_opt..siphash..sys..SipHashState$u20$as$u20$core..fmt..Debug$GT$3fmt17h85ce43ed0d4788c0E
	.cfi_endproc

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.ascii	"SipHashState"
	.size	.L__unnamed_3, 12

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_2:
	.quad	_ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h93671e49cf0d1c4fE
	.size	.L__unnamed_2, 32

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_1:
	.quad	_ZN4core3ptr24drop_in_place$LT$u64$GT$17h95420b9da68bcb2bE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb10c6415c951d1d1E
	.size	.L__unnamed_1, 32

	.ident	"rustc version 1.74.0-nightly (203c57dbe 2023-09-17)"
	.section	".note.GNU-stack","",@progbits
