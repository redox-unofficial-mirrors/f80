pub unsafe fn load_f80_into_f64(input: *const u8, output: *mut f64) {
    core::arch::asm!("
      fld TBYTE PTR [{}]
      fstp QWORD PTR [{}]
    ", in(reg) input, in(reg) output);
}

pub unsafe fn load_f64_into_f80(input: *const f64, output: *mut u8) {
    core::arch::asm!("
      fld QWORD PTR [{}]
      fstp TBYTE PTR [{}]
    ", in(reg) input, in(reg) output);
}
