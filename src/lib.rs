#![feature(core, test)]
extern crate test;

pub mod u8 {
    
    /// TODO
    pub unsafe fn reverse_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let b = buf;
        for i in 0..(len/2) {
            swap(b.offset(i as isize), b.offset((len - 1 - i) as isize));
        }
    }
    
    /// TODO
    pub unsafe fn reverse_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        for i in 0..len {
            *(d.offset(i as isize)) = *(s.offset((len - 1 - i) as isize));
            //d = d.offset(1);
            //s = s.offset(1);
        }
    }
}

//mod u16 {
//    ///// Swap bytes inplace in a buffer of u16 objects (unaligned).
//    //pub unsafe fn u16_swap_inplace_unaligned(buf: *mut u8, len: usize) {
//    //    use std::ptr::swap;
//    //    let mut d = buf;
//    //    for _ in 0..len {
//    //        swap(d.offset(0), d.offset(1));
//    //        d = d.offset(2);
//    //    }
//    //}
//    ///// Swap bytes inplace in a buffer of u16 objects (aligned to 2 bytes).
//    //pub unsafe fn u16_swap_inplace_aligned(buf: *mut u16, len: usize) {
//    //    u16_swap_inplace_unaligned(buf as *mut u8, len);
//    //}
//}
//
//mod u32 {
//    ///// Swap bytes inplace in a buffer of u32 objects (unaligned).
//    //pub unsafe fn u32_swap_inplace_unaligned(buf: *mut u8, len: usize) {
//    //    use std::ptr::swap;
//    //    let mut d = buf;
//    //    for _ in 0..len {
//    //        swap(d.offset(0), d.offset(3));
//    //        swap(d.offset(1), d.offset(2));
//    //        d = d.offset(4);
//    //    }
//    //}
//    ///// Swap bytes inplace in a buffer of u32 objects (aligned to 4 bytes).
//    //pub unsafe fn u32_swap_inplace_aligned(buf: *mut u32, len: usize) {
//    //    u32_swap_inplace_unaligned(buf as *mut u8, len);
//    //}
//}

/// Swap bytes for `u64`s on all targets.
pub mod u64 {
    use std::num::Int;
    use std::mem::align_of;
    //use std::ptr::copy_nonoverlapping_memory;

    /// Swaps `len*8` bytes for `u64` objects inplace in `buf`.
    pub unsafe fn swap_inplace(buf: *mut u8, len: usize) {

        unsafe fn u64_swap_inplace_u8(buf: *mut u8, len: usize) {
            use std::ptr::swap;
            let mut b = buf;
            for _ in 0..len {
                swap(b.offset(0), b.offset(7));
                swap(b.offset(1), b.offset(6));
                swap(b.offset(2), b.offset(5));
                swap(b.offset(3), b.offset(4));
                b = b.offset(8);
            }
        }

        unsafe fn u64_swap_inplace_u64(buf: *mut u8, len: usize) {
            let mut b = buf;
            for _ in 0..len {
                (*b) = (*b).swap_bytes();
                b = b.offset(1);
            }
        }

        unsafe fn u64_swap_inplace_u64x2(buf: *mut u8, len: usize) {
            u64_swap_inplace_u64(buf, len);
        }
        
        match align_of::<u64>() {
            16 => u64_swap_inplace_u64x2(buf, len),
            8 => u64_swap_inplace_u64(buf, len),
            _ => u64_swap_inplace_u8(buf, len),
        }
    }
    

    /// Swaps `len*8` bytes for `u64` objects from `src` to `dst`. The source and destination may not overlap.
    ///
    ///
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {

        unsafe fn u64_swap_nonoverlapping_memory_u8(dst: *mut u8, src: *const u8, len: usize) {
            let (mut d, mut s) = (dst, src);
            for _ in 0..len {
                *(d.offset(0)) = *(s.offset(7));
                *(d.offset(1)) = *(s.offset(6));
                *(d.offset(2)) = *(s.offset(5));
                *(d.offset(3)) = *(s.offset(4));
                *(d.offset(4)) = *(s.offset(3));
                *(d.offset(5)) = *(s.offset(2));
                *(d.offset(6)) = *(s.offset(1));
                *(d.offset(7)) = *(s.offset(0));
                d = d.offset(8);
                s = s.offset(8);
            }
        }

        unsafe fn u64_swap_nonoverlapping_memory_u64(dst: *mut u8, src: *const u8, len: usize) {
            let (mut d, mut s) = (dst, src);
            for _ in 0..len {
                (*d) = (*s).swap_bytes();
                d = d.offset(1);
                s = s.offset(1);
            }
        }
        
        unsafe fn u64_swap_nonoverlapping_memory_u64x2(dst: *mut u8, src: *const u8, len: usize) {
            u64_swap_nonoverlapping_memory_u64(dst, src, len);
        }

        match align_of::<u64>() {
            16 => u64_swap_nonoverlapping_memory_u64x2(dst, src, len),
            8 => u64_swap_nonoverlapping_memory_u64(dst, src, len),
            _ => u64_swap_nonoverlapping_memory_u8(dst, src, len),
        }
    }
}

//
// unsafe derived
//


//
// safe core
//


/// Swap bytes for `u64` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu64 {
    use std::num::Int;
    use std::ptr::copy_nonoverlapping_memory;
    use std::mem::uninitialized;

    /// Swap bytes for `u64` objects.
    ///
    /// ```rust
    /// fn swap(n: u64) -> u64 {
    ///     use std::num::Int;
    ///     n.to_be()
    /// }
    /// ```
    ///
    pub fn swap(n: u64) -> u64 {
        n.to_be()
    }

    /// Swap bytes for `u64` objects, inplace in a buffer.
    pub fn swap_slice(buf: &mut [u64]) {
        unsafe {
            swap_inplace(buf.get_unchecked_mut(0) as *mut _ as *mut u8, buf.len());
        }
    }

    /// Decodes big-endian bytes to a native-endian `u64` object.
    pub fn decode(buf: &[u8]) -> u64 {
        assert_eq!(buf.len(), 8);
        unsafe {
            let mut tmp: u64 = uninitialized();
            copy_nonoverlapping_memory(
                &mut tmp as *mut _ as *mut u8,
                buf.get_unchecked(0), 8);
            tmp.to_be()
        }
    }
    
    /// Decodes big-endian bytes to native-endian `u64` objects.
    pub fn decode_slice(dst: &mut [u64], src: &[u8]) {
        assert_eq!(dst.len()*8, src.len());
        unsafe {
            swap_memory(
                dst.get_unchecked_mut(0) as *mut _ as *mut u8,
                src.get_unchecked(0), src.len());
        }
    }
    
    /// Encodes a native-endian `u64` object to big-endian bytes.
    pub fn encode(dst: &mut [u8], src: u64) {
        assert_eq!(dst.len(), 8);
        unsafe {
            let tmp: u64 = src.to_be();
            copy_nonoverlapping_memory(
                dst.get_unchecked_mut(0),
                &tmp as *const _ as *const u8, 4);
        }

    }
    
    /// Encodes native-endian `u64` objects to big-endian bytes.
    pub fn encode_slice(dst: &mut [u8], src: &[u64]) {
        assert_eq!(dst.len(), src.len()*8);
        unsafe {
            swap_memory(
                dst.get_unchecked_mut(0),
                src.get_unchecked(0) as *const _ as *const u8, src.len());
        }
    }

    /// **[UNSAFE]** TODO
    pub unsafe fn swap_inplace(buf: *mut u8, len: usize) {
        if cfg!(target_endian = "little") {
            super::u64::swap_inplace(buf, len);
        }
    }
    
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        if cfg!(target_endian = "little") {
            super::u64::swap_memory(dst, src, len);
        } else {
            copy_nonoverlapping_memory(dst, src, len*8);
        }
    }
        
}
    //fn beu16_swap_inplace_aligned(buf: *mut u16, len: usize);
    //fn leu16_swap_inplace_aligned(buf: *mut u16, len: usize);
    //fn beu32_swap_inplace_aligned(buf: *mut u32, len: usize);
    //fn leu32_swap_inplace_aligned(buf: *mut u32, len: usize);
    //fn leu64_swap_inplace_aligned(buf: *mut u64, len: usize);
    //fn beu16_swap_inplace_unaligned(buf: *mut u8, len: usize);
    //fn leu16_swap_inplace_unaligned(buf: *mut u8, len: usize);
    //fn beu32_swap_inplace_unaligned(buf: *mut u8, len: usize);
    //fn leu32_swap_inplace_unaligned(buf: *mut u8, len: usize);
    //fn leu64_swap_inplace_unaligned(buf: *mut u8, len: usize);

/// Swap bytes for `u64`s only on big-endian targets, does nothing on little-endian targets.
pub mod leu64 {
}

mod tests;
