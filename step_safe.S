example::AsciiStr::step_by:
        push    rax
        mov     rax, qword ptr [rdi + 8]
        sub     rax, rsi
        jb      .LBB0_1
        add     qword ptr [rdi], rsi
        mov     qword ptr [rdi + 8], rax
        mov     rax, rdi
        pop     rcx
        ret
.LBB0_1:
        lea     rdi, [rip + .L__unnamed_1]
        lea     rdx, [rip + .L__unnamed_2]
        mov     esi, 43
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

example::try_parse_digits:
        push    rax
        cmp     qword ptr [rdi + 8], 0
        je      .LBB1_5
        mov     rax, qword ptr [rdi]
.LBB1_2:
        movzx   eax, byte ptr [rax]
        add     al, -48
        cmp     al, 9
        ja      .LBB1_5
        mov     rcx, qword ptr [rsi]
        lea     rcx, [rcx + 4*rcx]
        movzx   eax, al
        lea     rax, [rax + 2*rcx]
        mov     qword ptr [rsi], rax
        mov     rcx, qword ptr [rdi + 8]
        test    rcx, rcx
        je      .LBB1_6
        mov     rax, qword ptr [rdi]
        add     rax, 1
        mov     qword ptr [rdi], rax
        add     rcx, -1
        mov     qword ptr [rdi + 8], rcx
        jne     .LBB1_2
.LBB1_5:
        pop     rax
        ret
.LBB1_6:
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
        .asciz  "\017\000\000\000\000\000\000\000u\000\000\000&\000\000"
