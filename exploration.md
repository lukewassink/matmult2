This code:

```rust
pub fn naive_raw_slice(m1_vals: &Vec<f64>, m2_vals: &Vec<f64>, rows: usize, cols: usize, inner: usize) -> Vec<f64> {
    let mut prod: Vec<f64> = vec![0.0; rows * cols];

    for i in 0..rows {
        for j in 0..cols {
            assert!(m1_vals.len() >= (rows - 1) * inner + inner);
            assert!(m2_vals.len() >= (inner - 1) * cols + cols);
            let mut acc = 0.0;
            for k in 0..inner {
                acc += m1_vals[i * inner + k] * m2_vals[k * cols + j];
            }
            prod[i * cols + j] = acc;
        }
    }

    prod
}
```

produced this core loop:

```
LBB0_18:
  lea rdi, [rdx + 1]
  xorpd xmm0, xmm0
  mov rax, rdx
  xor r12d, r12d
.LBB0_19:
  lea rcx, [r11 + r12]
  cmp rcx, rsi
  jae .LBB0_40 // Panics
  cmp rax, r10
  jae .LBB0_21 // Panics
  movsd xmm1, qword ptr [r14 + 8*r12]
  mulsd xmm1, qword ptr [r15 + 8*rax]
  inc r12
  addsd xmm0, xmm1
  add rax, r8
  cmp r9, r12
  jne .LBB0_19
  add rdx, rbp
  cmp rdx, r13
  jae .LBB0_24 // Panics
  movsd qword ptr [rbx + 8*rdx], xmm0
  mov rdx, rdi
  cmp rdi, r8
  jne .LBB0_18
```

and

```
.LBB0_21:
  mov rsi, r10
  lea rdx, [rip + .Lanon.4a0354c8099b7d52d5a6d288cc38fbee.7]
  jmp .LBB0_42
.LBB0_40:
  cmp rsi, r11
  cmova r11, rsi
  lea rdx, [rip + .Lanon.4a0354c8099b7d52d5a6d288cc38fbee.6]
  mov rax, r11
.LBB0_42:
  mov rdi, rax
  call qword ptr [rip + core[a9090a07a7fa7517]::panicking::panic_bounds_check@GOTPCREL]
  jmp .LBB0_12
.LBB0_24:
  mov rax, rdx
  mov rsi, r13
  lea rdx, [rip + .Lanon.4a0354c8099b7d52d5a6d288cc38fbee.5]
  jmp .LBB0_42
```

So we see there are 3 places where the code can panic. Bounds checks?

## Next step

Use unsafe code.
