# ASM Checks

Used to check ASM generation, filtered via Compiler Explorer. The only major differences are:

**eq_ignore_case**

This is insignificant, because it is always called with literals for `u`, and this produces the same ASM in that case, and otherwise the initial implementation is logically unsound.

```diff
2c2,5
<         test    rdx, rdx
---
>         mov     r9, qword ptr [rdi + 8]
>         cmp     r9, rdx
>         cmova   r9, rdx
>         test    r9, r9
```

When we use it with a literal `u`, we get the same ASM.

**step_by**

This can't be trivially fixed, since the compiler can't decipher the bounds checks are redundant.

```diff
0a1,17
> example::AsciiStr::step_by:
>         push    rax
>         mov     rax, qword ptr [rdi + 8]
>         sub     rax, rsi
>         jb      .LBB0_1
>         add     qword ptr [rdi], rsi
>         mov     qword ptr [rdi + 8], rax
>         mov     rax, rdi
>         pop     rcx
>         ret
> .LBB0_1:
>         lea     rdi, [rip + .L__unnamed_1]
>         lea     rdx, [rip + .L__unnamed_2]
>         mov     esi, 43
>         call    qword ptr [rip + core::panicking::panic@GOTPCREL]
>         ud2
> 
1a19
>         push    rax
3c21
<         je      .LBB0_4
---
>         je      .LBB1_5
5c23
< .LBB0_2:
---
> .LBB1_2:
9c27
<         ja      .LBB0_4
---
>         ja      .LBB1_5
14a33,35
>         mov     rcx, qword ptr [rdi + 8]
>         test    rcx, rcx
>         je      .LBB1_6
18,20c39,43
<         add     qword ptr [rdi + 8], -1
<         jne     .LBB0_2
< .LBB0_4:
---
>         add     rcx, -1
>         mov     qword ptr [rdi + 8], rcx
>         jne     .LBB1_2
> .LBB1_5:
>         pop     rax
21a45,50
> .LBB1_6:
>         lea     rdi, [rip + .L__unnamed_1]
>         lea     rdx, [rip + .L__unnamed_2]
>         mov     esi, 43
>         call    qword ptr [rip + core::panicking::panic@GOTPCREL]
>         ud2
24a54,63
> 
> .L__unnamed_1:
>         .ascii  "called `Option::unwrap()` on a `None` value"
> 
> .L__unnamed_3:
>         .ascii  "/app/example.rs"
> 
> .L__unnamed_2:
>         .quad   .L__unnamed_3
>         .asciz  "\017\000\000\000\000\000\000\000u\000\000\000&\000\000"
```

# Summary

Safety go brrr.
