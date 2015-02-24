#![feature(core, test)]
//#![feature(intrinsics, simd, simd_ffi, link_llvm_intrinsics)]
extern crate test;

pub mod u8 {
    use std::mem;
    use std::num::Int;
    use std::ptr;
    
    pub unsafe fn align_of_ptr(src: *const u8) -> usize {
        let off: usize = mem::transmute(src);
        2.pow(off.trailing_zeros())
    }

    /// TODO
    pub unsafe fn reverse_memory_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let n: usize = len >> 1;
        let b = buf;
        for i in 0..n {
            ptr::swap(b.offset(i as isize),
                      b.offset((len - 1 - i) as isize));
        }
    }
    
    /// TODO
    pub unsafe fn reverse_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        s = s.offset((len - 1) as isize);
        for _ in 0..len {
            *d = *s;
            d = d.offset(1);
            s = s.offset(-1);
        }

        //use std::simd::u8x16;
        //use std::cmp;
        //#[inline]
        //unsafe fn u8_reverse_memory_u8(dst: *mut u8, src: *const u8, len: usize) {
        //    let (mut d, mut s) = (dst, src);
        //    s = s.offset((len - 1) as isize);
        //    for _ in 0..len {
        //        *d = *s;
        //        d = d.offset(1);
        //        s = s.offset(-1);
        //    }
        //}
        //#[inline]
        //#[cfg(target_arch = "x86_64")]
        //unsafe fn u8_reverse_memory_u8x16(dst: *mut u8, src: *const u8, len: usize) {
        //    extern {
        //        #[link_name = "llvm.x86.ssse3.pshuf.b.128"]
        //        fn u8x16_shuffle(a: u8x16, b: u8x16) -> u8x16;
        //    }
        //    let len16: usize = len >> 4;
        //    let (mut d, mut s) = (dst as *mut u8x16, src as *const u8x16);
        //    s = s.offset((len16 - 1) as isize);
        //    for _ in 0..len16 {
        //        *d = u8x16_shuffle(*s, u8x16(15, 14, 13, 12, 11, 10, 9, 8,
        //                                      7,  6,  5,  4,  3,  2, 1, 0));
        //        d = d.offset(1);
        //        s = s.offset(-1);
        //    }
        //}
        //#[inline]
        //#[cfg(not(target_arch = "x86_64"))]
        //unsafe fn u8_reverse_memory_u8x16(dst: *mut u8, src: *const u8, len: usize) {
        //    u8_reverse_memory_u8(dst, src, len);
        //}        
        //let dalign = super::u8::align_of_ptr(dst);
        //let salign = super::u8::align_of_ptr(src);
        //match (cmp::min(dalign, salign), len % 16) {
        //    (16, 0) => u8_reverse_memory_u8x16(dst, src, len),
        //    _ => u8_reverse_memory_u8(dst, src, len),
        //}
        //u8_reverse_memory_u8(dst, src, len)
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
    use std::ptr;
    use std::cmp;
    
    /// Swaps `len*8` bytes for `u64` objects inplace in `buf`.
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {

        unsafe fn u64_swap_memory_inplace_u8(buf: *mut u8, len: usize) {
            let mut b = buf;
            for _ in 0..len {
                ptr::swap(b.offset(0), b.offset(7));
                ptr::swap(b.offset(1), b.offset(6));
                ptr::swap(b.offset(2), b.offset(5));
                ptr::swap(b.offset(3), b.offset(4));
                b = b.offset(8);
            }
        }

        unsafe fn u64_swap_memory_inplace_u64(buf: *mut u8, len: usize) {
            let mut b = buf;
            for _ in 0..len {
                (*b) = (*b).swap_bytes();
                b = b.offset(1);
            }
        }
        
        match super::u8::align_of_ptr(buf) {
            8 => u64_swap_memory_inplace_u64(buf, len),
            _ => u64_swap_memory_inplace_u8(buf, len),
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

        let dalign = super::u8::align_of_ptr(dst);
        let salign = super::u8::align_of_ptr(src);
        match cmp::min(dalign, salign) {
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
    use std::mem;
    use std::num::Int;
    use std::ptr;

    /// Decodes big-endian bytes to a native-endian `u64` object.
    pub fn decode(buf: &[u8]) -> u64 {
        assert_eq!(buf.len(), 8);
        unsafe {
            let mut tmp: u64 = mem::uninitialized();
            ptr::copy_nonoverlapping_memory(
                &mut tmp as *mut _ as *mut u8,
                buf.as_ptr(), 8);
            Int::from_be(tmp)
        }
    }
    
    /// Decodes big-endian bytes to a slice of native-endian `u64` objects.
    pub fn decode_slice(dst: &mut [u64], src: &[u8]) {
        assert_eq!(dst.len()*8, src.len());
        unsafe {
            swap_memory(
                dst.as_mut_ptr() as *mut u8,
                src.as_ptr(), src.len());
        }
    }
    
    /// Encodes a native-endian `u64` object to big-endian bytes.
    pub fn encode(dst: &mut [u8], src: u64) {
        assert_eq!(dst.len(), 8);
        unsafe {
            let tmp: u64 = src.to_be();
            ptr::copy_nonoverlapping_memory(
                dst.as_mut_ptr(),
                &tmp as *const _ as *const u8, 4);
        }

    }
    
    /// Encodes a slice of native-endian `u64` objects to big-endian bytes.
    pub fn encode_slice(dst: &mut [u8], src: &[u64]) {
        assert_eq!(dst.len(), src.len()*8);
        unsafe {
            swap_memory(
                dst.as_mut_ptr(),
                src.as_ptr() as *const u8, src.len());
        }
    }

    unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        if cfg!(target_endian = "little") {
            super::u64::swap_memory(dst, src, len);
        } else {
            ptr::copy_nonoverlapping_memory(dst, src, len*8);
        }
    }
        
}

/// Swap bytes for `u64`s only on big-endian targets, does nothing on little-endian targets.
pub mod leu64 {
}

mod tests;
