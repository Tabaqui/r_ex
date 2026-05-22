.text

.globl main

main:
  pushq %rbp
  movq %rsp, %rbp
  
  movq $47, %rdx
  movq $0, %rax
  movq $1, %rbx

  xorq %rcx, %rcx
  
  cmpq $1, %rdx
  je L0

  LOOP:
  movq %rbx, %rcx
  
  cmpq $2, %rdx
  je L0

  addq %rax, %rcx

  movq %rbx, %rax
  movq %rcx, %rbx

  decq %rdx
  cmpq $1, %rdx
  jne LOOP

  L0:
  leaq out(%rip), %rdi
  movq %rcx, %rsi

  call printf

  movq %rbp, %rsp
  popq %rbp

  ret

.section .rodata
  out:
    .string "Out: %d\n"
