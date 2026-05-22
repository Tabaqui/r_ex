.text

.globl main
.globl add

main:
    pushq %rbp
    movq %rsp, %rsp

    call add

    movq $res, %rdi
    movq %rax, %rsi
    call printf
    
    movq $0, %rax

    movq %rsp, %rbp
    popq %rbp
    
    ret

add:
    movq $100, %rax
    movq $200, %rbx
    addq %rbx, %rax
    ret

.section .rodata
    res:
        .string "%d cats\n"
