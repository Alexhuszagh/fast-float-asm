example::AsciiStr::first_isdigit:
        cmp     qword ptr [rdi + 8], 0
        je      .LBB0_1
        mov     rax, qword ptr [rdi]
        mov     al, byte ptr [rax]
        add     al, -48
        cmp     al, 10
        setb    al
        ret
.LBB0_1:
        xor     eax, eax
        ret
