example::try_parse_19digits:
        movabs  rax, 1000000000000000000
        mov     rcx, qword ptr [rsi]
        cmp     rcx, rax
        jae     .LBB0_5
        mov     rdx, qword ptr [rdi + 8]
.LBB0_2:
        test    rdx, rdx
        je      .LBB0_5
        mov     rdx, qword ptr [rdi]
        movzx   edx, byte ptr [rdx]
        add     dl, -48
        cmp     dl, 9
        ja      .LBB0_5
        lea     rcx, [rcx + 4*rcx]
        movzx   edx, dl
        lea     rcx, [rdx + 2*rcx]
        mov     qword ptr [rsi], rcx
        mov     rdx, qword ptr [rdi + 8]
        add     qword ptr [rdi], 1
        add     rdx, -1
        mov     qword ptr [rdi + 8], rdx
        mov     rcx, qword ptr [rsi]
        cmp     rcx, rax
        jb      .LBB0_2
.LBB0_5:
        ret
