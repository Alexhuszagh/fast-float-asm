example::try_parse_digits:
        cmp     qword ptr [rdi + 8], 0
        je      .LBB0_4
        mov     rax, qword ptr [rdi]
.LBB0_2:
        movzx   eax, byte ptr [rax]
        add     al, -48
        cmp     al, 9
        ja      .LBB0_4
        mov     rcx, qword ptr [rsi]
        lea     rcx, [rcx + 4*rcx]
        movzx   eax, al
        lea     rax, [rax + 2*rcx]
        mov     qword ptr [rsi], rax
        mov     rax, qword ptr [rdi]
        add     rax, 1
        mov     qword ptr [rdi], rax
        add     qword ptr [rdi + 8], -1
        jne     .LBB0_2
.LBB0_4:
        ret
