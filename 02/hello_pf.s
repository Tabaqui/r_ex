.text

.globl main

main:
    pushq %rbp
    movq %rsp, %rbp

    leaq msg(%rip), %rdi
    call printf

#    преимущества или недостатки вызова syscall непонятны/неизвестны (карманность?)
#    movq $1, %rax
#    movq $1, %rdi
##    movq $msg, %rsi
#    leaq msg(%rip), %rsi
#    movq $14, %rdx
#    syscall

    movq $0, %rax

    movq %rbp, %rsp
    popq %rbp

    ret

.section .rodata
msg:
    .string "hello peoplez\n"