example::offset_from:
        mov     rax, rsi
        test    rsi, rsi
        je      .LBB0_4
        mov     rcx, rax
.LBB0_2:
        movzx   edx, byte ptr [rdi]
        add     dl, -48
        cmp     dl, 9
        ja      .LBB0_5
        add     rdi, 1
        add     rcx, -1
        jne     .LBB0_2
.LBB0_4:
        xor     ecx, ecx
.LBB0_5:
        sub     rax, rcx
        ret
