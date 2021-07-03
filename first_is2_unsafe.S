example::AsciiStr::first_is2:
        cmp     qword ptr [rdi + 8], 0
        je      .LBB0_1
        mov     rax, qword ptr [rdi]
        mov     al, byte ptr [rax]
        cmp     al, sil
        sete    cl
        cmp     al, dl
        sete    al
        or      al, cl
        ret
.LBB0_1:
        xor     eax, eax
        ret
