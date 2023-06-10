BetterASM
=================
The goal is obvius, but this will be very complicated.

*I will be happy about any pull requests since I will not be able to do this alone.*

My wish is to turn this:

```asm
section	.data
msg	db	'Hello, world!'	;our string
len	equ	$ - msg	        ;length of our string

section	.text
	global _start       ;must be declared for using gcc
_start:                     ;tell linker entry point
	mov	edx, len    ;message length
	mov	ecx, msg    ;message to write
	mov	ebx, 1	    ;file descriptor (stdout)
	mov	eax, 4	    ;system call number (sys_write)
	int	0x80        ;call kernel
	mov	eax, 1	    ;system call number (sys_exit)
	int	0x80        ;call kernel
```
into something like this:
```
DATA
    msg: 'Hello, world!'
    len: msg.len

CODE
    _start:
        edx = len
        ecx = msg
        ebx = 1
        eax = 4
        kernel_call
        eax = 1
        kernel_call
```

Please note that I am not good at assembly, so I might do some mistakes, especially with multiple platforms, so you can either create a pull request or an issue.
----------------------------------------

