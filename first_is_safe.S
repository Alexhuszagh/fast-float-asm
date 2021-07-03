example::AsciiStr::first_is:
        cmp     qword ptr [rdi + 8], 0
        je      .LBB0_1
        mov     rax, qword ptr [rdi]
        cmp     byte ptr [rax], sil
        sete    al
        ret
.LBB0_1:
        xor     eax, eax
        ret
