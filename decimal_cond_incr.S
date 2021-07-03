example::Decimal::try_add_digit:
        mov     rax, qword ptr [rdi]
        cmp     rax, 767
        ja      .LBB0_2
        mov     byte ptr [rdi + rax + 14], sil
        add     qword ptr [rdi], 1
.LBB0_2:
        ret
