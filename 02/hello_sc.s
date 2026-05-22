.text

.globl main

main:
    pushq %rbp
    movq %rsp, %rsp

    movq $1, %rax
    movq $1, %rdi
#    movq $msg, %rsi
    leaq msg(%rip), %rsi
    movq 6, %rdx
    syscall

    mov $0, %rax

    movq %rsp, %rbp
    popq %rbp
    
    ret

.rodata
    msg: 
        .string "hello\n"
    