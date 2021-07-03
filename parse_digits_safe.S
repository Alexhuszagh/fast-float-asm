example::parse_decimal_digits:
        test    rsi, rsi
        je      .LBB0_6
        xor     r8d, r8d
        jmp     .LBB0_2
.LBB0_5:
        add     rcx, 1
        mov     qword ptr [rdx], rcx
        add     r8, 1
        cmp     rsi, r8
        je      .LBB0_6
.LBB0_2:
        movzx   eax, byte ptr [rdi + r8]
        add     al, -48
        cmp     al, 9
        ja      .LBB0_6
        mov     rcx, qword ptr [rdx]
        cmp     rcx, 767
        ja      .LBB0_5
        mov     byte ptr [rdx + rcx + 14], al
        mov     rcx, qword ptr [rdx]
        jmp     .LBB0_5
.LBB0_6:
        ret
