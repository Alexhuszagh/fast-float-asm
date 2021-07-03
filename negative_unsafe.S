example::is_negative:
        mov     rcx, qword ptr [rdi + 8]
        test    rcx, rcx
        je      .LBB0_5
        mov     rdx, qword ptr [rdi]
        mov     al, byte ptr [rdx]
        cmp     al, 45
        je      .LBB0_3
        cmp     al, 43
        jne     .LBB0_5
.LBB0_3:
        cmp     al, 45
        sete    al
        add     rdx, 1
        add     rcx, -1
        mov     qword ptr [rdi], rdx
        mov     qword ptr [rdi + 8], rcx
        ret
.LBB0_5:
        xor     eax, eax
        ret
