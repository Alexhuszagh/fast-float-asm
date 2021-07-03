example::offset_from:
        mov     rax, rdi
        test    rsi, rsi
        je      .LBB0_5
        lea     rcx, [rdi + rsi]
        mov     rax, rdi
.LBB0_2:
        movzx   edx, byte ptr [rax]
        add     dl, -48
        cmp     dl, 9
        ja      .LBB0_5
        add     rax, 1
        add     rsi, -1
        jne     .LBB0_2
        mov     rax, rcx
.LBB0_5:
        sub     rax, rdi
        ret
