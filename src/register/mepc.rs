//! mepc register

read_csr_as_usize!(0x341);
write_csr_as_usize!(0x341);
set!(0x341);
clear!(0x341);

#[inline]
pub unsafe fn set(val: usize){
    let bits = val as usize;
    _set(bits);
}
#[inline]
pub unsafe fn clear(val: usize){
    let bits = val as usize;
    _clear(bits);
}
