example::Decimal::try_add_digit:
        mov     rax, qword ptr [rdi]
        cmp     rax, 767
        ja      .LBB0_2
        mov     byte ptr [rdi + rax + 14], sil
        mov     rax, qword ptr [rdi]
.LBB0_2:
        add     rax, 1
        mov     qword ptr [rdi], rax
        ret
