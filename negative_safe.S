example::is_negative:
        mov     rcx, qword ptr [rdi + 8]
        test    rcx, rcx
        je      .LBB0_1
        mov     rdx, qword ptr [rdi]
        mov     sil, byte ptr [rdx]
        cmp     sil, 45
        sete    al
        je      .LBB0_4
        cmp     sil, 43
        jne     .LBB0_5
.LBB0_4:
        add     rdx, 1
        add     rcx, -1
        mov     qword ptr [rdi], rdx
        mov     qword ptr [rdi + 8], rcx
.LBB0_5:
        ret
.LBB0_1:
        xor     eax, eax
        ret
