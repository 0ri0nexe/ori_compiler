bits 64

; return 6
extern ExitProcess

global main
main:
	mov rcx, 6
	call ExitProcess