example::AsciiStr::skip_chars:
        mov     rax, qword ptr [rdi]
        mov     rdx, qword ptr [rdi + 8]
        test    rdx, rdx
        je      .LBB0_1
        lea     rcx, [rax + rdx]
.LBB0_3:
        cmp     byte ptr [rax], sil
        jne     .LBB0_6
        add     rax, 1
        add     rdx, -1
        jne     .LBB0_3
        xor     edx, edx
        mov     rax, rcx
.LBB0_6:
        ret
.LBB0_1:
        xor     edx, edx
        ret
