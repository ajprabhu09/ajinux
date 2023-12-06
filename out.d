
./user/tests/progs/simple.o:	file format elf64-x86-64

Disassembly of section .strtab:

0000000000000000 <.strtab>:
       0: 00 2e                        	addb	%ch, (%rsi)
       2: 74 65                        	je	0x69 <.strtab+0x69>
       4: 78 74                        	js	0x7a <.strtab+0x7a>
       6: 00 2e                        	addb	%ch, (%rsi)
       8: 63 6f 6d                     	movslq	109(%rdi), %ebp
       b: 6d                           	insl	%dx, %es:(%rdi)
       c: 65 6e                        	outsb	%gs:(%rsi), %dx
       e: 74 00                        	je	0x10 <.strtab+0x10>
      10: 6d                           	insl	%dx, %es:(%rdi)
      11: 61                           	<unknown>
      12: 69 6e 00 2e 6e 6f 74         	imull	$1953459758, (%rsi), %ebp # imm = 0x746F6E2E
      19: 65 2e 47 4e                  	<unknown>
      1d: 55                           	pushq	%rbp
      1e: 2d 73 74 61 63               	subl	$1667331187, %eax       # imm = 0x63617473
      23: 6b 00 2e                     	imull	$46, (%rax), %eax
      26: 6c                           	insb	%dx, %es:(%rdi)
      27: 6c                           	insb	%dx, %es:(%rdi)
      28: 76 6d                        	jbe	0x97 <.strtab+0x97>
      2a: 5f                           	popq	%rdi
      2b: 61                           	<unknown>
      2c: 64 64 72 73                  	jb	0xa3 <.strtab+0xa3>
      30: 69 67 00 2e 72 65 6c         	imull	$1818587694, (%rdi), %esp # imm = 0x6C65722E
      37: 61                           	<unknown>
      38: 2e 65 68 5f 66 72 61         	pushq	$1634887263             # imm = 0x6172665F
      3f: 6d                           	insl	%dx, %es:(%rdi)
      40: 65 00 73 69                  	addb	%dh, %gs:105(%rbx)
      44: 6d                           	insl	%dx, %es:(%rdi)
      45: 70 6c                        	jo	0xb3 <.strtab+0xb3>
      47: 65 2e 63 00                  	movslq	%cs:(%rax), %eax
      4b: 2e 73 74                     	jae	0xc2 <.strtab+0xc2>
      4e: 72 74                        	jb	0xc4 <.strtab+0xc4>
      50: 61                           	<unknown>
      51: 62 00 2e 73                  	<unknown>
      55: 79 6d                        	jns	0xc4 <.strtab+0xc4>
      57: 74 61                        	je	0xba <.strtab+0xba>
      59: 62 00 2e 64                  	<unknown>
      5d: 61                           	<unknown>
      5e: 74 61                        	je	0xc1 <.strtab+0xc1>
      60: 00                           	<unknown>

Disassembly of section .text:

0000000000000000 <main>:
       0: 55                           	pushq	%rbp
       1: 48 89 e5                     	movq	%rsp, %rbp
       4: c7 45 fc 00 00 00 00         	movl	$0, -4(%rbp)
       b: 31 c0                        	xorl	%eax, %eax
       d: 5d                           	popq	%rbp
       e: c3                           	retq

Disassembly of section .data:

0000000000000000 <a>:
       0: 01 00                        	addl	%eax, (%rax)
       2: 00 00                        	addb	%al, (%rax)

Disassembly of section .comment:

0000000000000000 <.comment>:
       0: 00 41 70                     	addb	%al, 112(%rcx)
       3: 70 6c                        	jo	0x71 <.comment+0x71>
       5: 65 20 63 6c                  	andb	%ah, %gs:108(%rbx)
       9: 61                           	<unknown>
       a: 6e                           	outsb	(%rsi), %dx
       b: 67 20 76 65                  	andb	%dh, 101(%esi)
       f: 72 73                        	jb	0x84 <.comment+0x84>
      11: 69 6f 6e 20 31 35 2e         	imull	$775237920, 110(%rdi), %ebp # imm = 0x2E353120
      18: 30 2e                        	xorb	%ch, (%rsi)
      1a: 30 20                        	xorb	%ah, (%rax)
      1c: 28 63 6c                     	subb	%ah, 108(%rbx)
      1f: 61                           	<unknown>
      20: 6e                           	outsb	(%rsi), %dx
      21: 67 2d 31 35 30 30            	addr32		subl	$808465713, %eax # imm = 0x30303531
      27: 2e 30 2e                     	xorb	%ch, %cs:(%rsi)
      2a: 34 30                        	xorb	$48, %al
      2c: 2e 31 29                     	xorl	%ebp, %cs:(%rcx)
      2f: 00                           	<unknown>

Disassembly of section .eh_frame:

0000000000000000 <.eh_frame>:
       0: 14 00                        	adcb	$0, %al
       2: 00 00                        	addb	%al, (%rax)
       4: 00 00                        	addb	%al, (%rax)
       6: 00 00                        	addb	%al, (%rax)
       8: 01 7a 52                     	addl	%edi, 82(%rdx)
       b: 00 01                        	addb	%al, (%rcx)
       d: 78 10                        	js	0x1f <.eh_frame+0x1f>
       f: 01 1b                        	addl	%ebx, (%rbx)
      11: 0c 07                        	orb	$7, %al
      13: 08 90 01 00 00 1c            	orb	%dl, 469762049(%rax)
      19: 00 00                        	addb	%al, (%rax)
      1b: 00 1c 00                     	addb	%bl, (%rax,%rax)
      1e: 00 00                        	addb	%al, (%rax)
      20: 00 00                        	addb	%al, (%rax)
      22: 00 00                        	addb	%al, (%rax)
      24: 0f 00 00                     	sldtw	(%rax)
      27: 00 00                        	addb	%al, (%rax)
      29: 41 0e                        	<unknown>
      2b: 10 86 02 43 0d 06            	adcb	%al, 101532418(%rsi)
      31: 4a 0c 07                     	orb	$7, %al
      34: 08 00                        	orb	%al, (%rax)
      36: 00 00                        	addb	%al, (%rax)

Disassembly of section .rela.eh_frame:

0000000000000000 <.rela.eh_frame>:
       0: 20 00                        	andb	%al, (%rax)
       2: 00 00                        	addb	%al, (%rax)
       4: 00 00                        	addb	%al, (%rax)
       6: 00 00                        	addb	%al, (%rax)
       8: 02 00                        	addb	(%rax), %al
       a: 00 00                        	addb	%al, (%rax)
       c: 02 00                        	addb	(%rax), %al
		...
      16: 00 00                        	addb	%al, (%rax)

Disassembly of section .symtab:

0000000000000000 <.symtab>:
		...
      18: 42 00 00                     	addb	%al, (%rax)
      1b: 00 04 00                     	addb	%al, (%rax,%rax)
      1e: f1                           	<unknown>
      1f: ff 00                        	incl	(%rax)
		...
      31: 00 00                        	addb	%al, (%rax)
      33: 00 03                        	addb	%al, (%rbx)
      35: 00 02                        	addb	%al, (%rdx)
		...
      47: 00 10                        	addb	%dl, (%rax)
      49: 00 00                        	addb	%al, (%rax)
      4b: 00 12                        	addb	%dl, (%rdx)
      4d: 00 02                        	addb	%al, (%rdx)
		...
      57: 00 0f                        	addb	%cl, (%rdi)
      59: 00 00                        	addb	%al, (%rax)
      5b: 00 00                        	addb	%al, (%rax)
      5d: 00 00                        	addb	%al, (%rax)
      5f: 00 5f 00                     	addb	%bl, (%rdi)
      62: 00 00                        	addb	%al, (%rax)
      64: 11 00                        	adcl	%eax, (%rax)
      66: 03 00                        	addl	(%rax), %eax
		...
      70: 04 00                        	addb	$0, %al
      72: 00 00                        	addb	%al, (%rax)
      74: 00 00                        	addb	%al, (%rax)
      76: 00 00                        	addb	%al, (%rax)
