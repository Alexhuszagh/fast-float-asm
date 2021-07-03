example::AsciiStr::eq_ignore_case:
        test    rdx, rdx
        je      .LBB0_12
        mov     r9, qword ptr [rdi]
        cmp     rdx, 32
        jae     .LBB0_3
        xor     edi, edi
        xor     r10d, r10d
        jmp     .LBB0_10
.LBB0_3:
        mov     r10, rdx
        and     r10, -32
        lea     rax, [r10 - 32]
        mov     r8, rax
        shr     r8, 5
        add     r8, 1
        test    rax, rax
        je      .LBB0_4
        mov     rax, r8
        and     rax, -2
        neg     rax
        pxor    xmm0, xmm0
        xor     edi, edi
        pxor    xmm1, xmm1
.LBB0_6:
        movdqu  xmm2, xmmword ptr [r9 + rdi]
        movdqu  xmm3, xmmword ptr [r9 + rdi + 16]
        movdqu  xmm4, xmmword ptr [r9 + rdi + 32]
        movdqu  xmm5, xmmword ptr [r9 + rdi + 48]
        movdqu  xmm6, xmmword ptr [rsi + rdi]
        pxor    xmm6, xmm2
        por     xmm6, xmm0
        movdqu  xmm2, xmmword ptr [rsi + rdi + 16]
        pxor    xmm2, xmm3
        por     xmm2, xmm1
        movdqu  xmm0, xmmword ptr [rsi + rdi + 32]
        pxor    xmm0, xmm4
        por     xmm0, xmm6
        movdqu  xmm1, xmmword ptr [rsi + rdi + 48]
        pxor    xmm1, xmm5
        por     xmm1, xmm2
        add     rdi, 64
        add     rax, 2
        jne     .LBB0_6
        test    r8b, 1
        je      .LBB0_9
.LBB0_8:
        movdqu  xmm2, xmmword ptr [rsi + rdi]
        movdqu  xmm3, xmmword ptr [rsi + rdi + 16]
        movdqu  xmm4, xmmword ptr [r9 + rdi]
        pxor    xmm4, xmm2
        por     xmm0, xmm4
        movdqu  xmm2, xmmword ptr [r9 + rdi + 16]
        pxor    xmm2, xmm3
        por     xmm1, xmm2
.LBB0_9:
        por     xmm0, xmm1
        pshufd  xmm1, xmm0, 238
        por     xmm1, xmm0
        pshufd  xmm0, xmm1, 85
        por     xmm0, xmm1
        movdqa  xmm1, xmm0
        psrld   xmm1, 16
        por     xmm1, xmm0
        movdqa  xmm0, xmm1
        psrlw   xmm0, 8
        por     xmm0, xmm1
        movd    edi, xmm0
        cmp     r10, rdx
        je      .LBB0_11
.LBB0_10:
        movzx   eax, byte ptr [rsi + r10]
        xor     al, byte ptr [r9 + r10]
        lea     rcx, [r10 + 1]
        or      dil, al
        mov     r10, rcx
        cmp     rdx, rcx
        jne     .LBB0_10
.LBB0_11:
        cmp     dil, 32
        sete    al
        test    dil, dil
        je      .LBB0_12
        ret
.LBB0_12:
        mov     al, 1
        ret
.LBB0_4:
        pxor    xmm0, xmm0
        xor     edi, edi
        pxor    xmm1, xmm1
        test    r8b, 1
        jne     .LBB0_8
        jmp     .LBB0_9

example::AsciiStr::is_nan:
        cmp     qword ptr [rdi + 8], 3
        jb      .LBB1_2
        mov     rax, qword ptr [rdi]
        mov     cl, byte ptr [rax]
        mov     dl, byte ptr [rax + 1]
        xor     cl, 78
        xor     dl, 65
        or      dl, cl
        mov     al, byte ptr [rax + 2]
        xor     al, 78
        or      al, dl
        test    al, -33
        sete    al
        ret
.LBB1_2:
        xor     eax, eax
        ret
