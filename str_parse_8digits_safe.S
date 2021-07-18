example::try_parse_8digits:
        push    rax
        cmp     qword ptr [rdi + 8], 8
        jb      .LBB0_4
        mov     rax, qword ptr [rdi]
        mov     rcx, qword ptr [rax]
        movabs  rdx, 5063812098665367110
        add     rdx, rcx
        movabs  rax, -3472328296227680304
        add     rax, rcx
        or      rdx, rax
        movabs  rcx, -9187201950435737472
        test    rdx, rcx
        je      .LBB0_2
.LBB0_4:
        mov     rax, rdi
        pop     rcx
        ret
.LBB0_2:
        imul    r8, qword ptr [rsi], 100000000
        lea     rdx, [rax + 4*rax]
        shr     rax, 8
        lea     rax, [rax + 2*rdx]
        movabs  r9, 1095216660735
        mov     rcx, rax
        and     rcx, r9
        movabs  rdx, 4294967296000100
        imul    rdx, rcx
        shr     rax, 16
        and     rax, r9
        movabs  rcx, 42949672960001
        imul    rcx, rax
        add     rcx, rdx
        shr     rcx, 32
        add     rcx, r8
        mov     qword ptr [rsi], rcx
        mov     rax, qword ptr [rdi + 8]
        cmp     rax, 7
        jbe     .LBB0_5
        add     rax, -8
        add     qword ptr [rdi], 8
        mov     qword ptr [rdi + 8], rax
        mov     rax, rdi
        pop     rcx
        ret
.LBB0_5:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 43
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::main:
        ret

.L__unnamed_1:
        .ascii  "called `Option::unwrap()` on a `None` value"

.L__unnamed_3:
        .ascii  "/app/example.rs"

.L__unnamed_2:
        .quad   .L__unnamed_3
        .asciz  "\017\000\000\000\000\000\000\000\272\000\000\000\"\000\000"
