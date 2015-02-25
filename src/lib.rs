#![feature(core, test)]
extern crate test;

/// Swap bytes for `u8` slices on all targets.
pub mod u8 {
    use std::mem;
    use std::num::Int;
    use std::ptr;
    pub const BYTES: usize = 1;

    /// TODO
    #[inline]
    pub unsafe fn align_of_ptr(src: *const u8) -> usize {
        let off: usize = mem::transmute(src);
        2.pow(off.trailing_zeros())
    }

    /// TODO
    #[inline]
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
    #[inline]
    pub unsafe fn reverse_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        s = s.offset((len - 1) as isize);
        for _ in 0..len {
            *d = *s;
            d = d.offset(1);
            s = s.offset(-1);
        }
    }
}

/// Swap bytes for `u16` objects on all targets.
pub mod u16 {
    use std::ptr;
    pub const BYTES: usize = 2;

    /// TODO
    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        let mut b = buf;
        for _ in 0..len {
            ptr::swap(b.offset(0), b.offset(1));
            b = b.offset(2);
        }
    }

    /// TODO
    #[inline]
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        for _ in 0..len {
            *(d.offset(0)) = *(s.offset(1));
            *(d.offset(1)) = *(s.offset(0));
            d = d.offset(2);
            s = s.offset(2);
        }
    }
}

/// Swap bytes for `[u8; 3]` objects on all targets.
pub mod u24 {
    use std::ptr;
    pub const BYTES: usize = 3;
    
    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        let mut b = buf;
        for _ in 0..len {
            ptr::swap(b.offset(0), b.offset(2));
            // we don't need to swap 1 and 1
            b = b.offset(3);
        }
    }
    
    #[inline]
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        for _ in 0..len {
            *(d.offset(0)) = *(s.offset(2));
            *(d.offset(1)) = *(s.offset(1));
            *(d.offset(2)) = *(s.offset(0));
            d = d.offset(3);
            s = s.offset(3);
        }
    }
}

/// Swap bytes for `[u8; 5]` objects on all targets.
pub mod u40 {
    use std::ptr;
    pub const BYTES: usize = 5;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        let mut b = buf;
        for _ in 0..len {
            ptr::swap(b.offset(0), b.offset(4));
            ptr::swap(b.offset(1), b.offset(3));
            // we don't need to swap 2 and 2
            b = b.offset(5);
        }
    }
    
    #[inline]
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        for _ in 0..len {
            *(d.offset(0)) = *(s.offset(4));
            *(d.offset(1)) = *(s.offset(3));
            *(d.offset(2)) = *(s.offset(2));
            *(d.offset(3)) = *(s.offset(1));
            *(d.offset(4)) = *(s.offset(0));
            d = d.offset(5);
            s = s.offset(5);
        }
    }
}

/// Swap bytes for `[u8; 6]` objects on all targets.
pub mod u48 {
    use std::ptr;
    pub const BYTES: usize = 6;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        let mut b = buf;
        for _ in 0..len {
            ptr::swap(b.offset(0), b.offset(5));
            ptr::swap(b.offset(1), b.offset(4));
            ptr::swap(b.offset(2), b.offset(3));
            b = b.offset(6);
        }
    }

    #[inline]
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        for _ in 0..len {
            *(d.offset(0)) = *(s.offset(5));
            *(d.offset(1)) = *(s.offset(4));
            *(d.offset(2)) = *(s.offset(3));
            *(d.offset(3)) = *(s.offset(2));
            *(d.offset(4)) = *(s.offset(1));
            *(d.offset(5)) = *(s.offset(0));
            d = d.offset(6);
            s = s.offset(6);
        }
    }
}

/// Swap bytes for `[u8; 7]` objects on all targets.
pub mod u56 {
    use std::ptr;
    pub const BYTES: usize = 7;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        let mut b = buf;
        for _ in 0..len {
            ptr::swap(b.offset(0), b.offset(6));
            ptr::swap(b.offset(1), b.offset(5));
            ptr::swap(b.offset(2), b.offset(4));
            // we don't need to swap 3 and 3
            b = b.offset(7);
        }
    }

    #[inline]
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
        let (mut d, mut s) = (dst, src);
        for _ in 0..len {
            *(d.offset(0)) = *(s.offset(6));
            *(d.offset(1)) = *(s.offset(5));
            *(d.offset(2)) = *(s.offset(4));
            *(d.offset(3)) = *(s.offset(3));
            *(d.offset(4)) = *(s.offset(2));
            *(d.offset(5)) = *(s.offset(1));
            *(d.offset(6)) = *(s.offset(0));
            d = d.offset(7);
            s = s.offset(7);
        }
    }
}

/// Swap bytes for `u32` objects on all targets.
pub mod u32 {
    use std::cmp;
    use std::num::Int;
    use std::ptr;
    pub const BYTES: usize = 4;


    /// Swaps `len*4` bytes for `u32` objects inplace in `buf`.
    ///
    ///
    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {

        unsafe fn u32_swap_memory_inplace_u8(buf: *mut u8, len: usize) {
            let mut b = buf;
            for _ in 0..len {
                ptr::swap(b.offset(0), b.offset(3));
                ptr::swap(b.offset(1), b.offset(2));
                b = b.offset(4);
            }
        }

        unsafe fn u32_swap_memory_inplace_u32(buf: *mut u8, len: usize) {
            let mut b: *mut u32 = buf as *mut u32;
            for _ in 0..len {
                (*b) = (*b).swap_bytes();
                b = b.offset(1);
            }
        }

        match super::u8::align_of_ptr(buf) {
            4 => u32_swap_memory_inplace_u32(buf, len),
            _ => u32_swap_memory_inplace_u8(buf, len),
        }
    }


    /// Swaps `len*4` bytes for `u32` objects from `src` to `dst`. The source and destination may not overlap.
    ///
    ///
    #[inline]
    pub unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {

        unsafe fn u32_swap_nonoverlapping_memory_u8(dst: *mut u8, src: *const u8, len: usize) {
            let (mut d, mut s) = (dst, src);
            for _ in 0..len {
                *(d.offset(0)) = *(s.offset(3));
                *(d.offset(1)) = *(s.offset(2));
                *(d.offset(2)) = *(s.offset(1));
                *(d.offset(3)) = *(s.offset(0));
                d = d.offset(4);
                s = s.offset(4);
            }
        }

        unsafe fn u32_swap_nonoverlapping_memory_u32(dst: *mut u8, src: *const u8, len: usize) {
            let (mut d, mut s) = (dst as *mut u32, src as *const u32);
            for _ in 0..len {
                (*d) = (*s).swap_bytes();
                d = d.offset(1);
                s = s.offset(1);
            }
        }

        let dalign = super::u8::align_of_ptr(dst);
        let salign = super::u8::align_of_ptr(src);
        match cmp::min(dalign, salign) {
            4 => u32_swap_nonoverlapping_memory_u32(dst, src, len),
            _ => u32_swap_nonoverlapping_memory_u8(dst, src, len),
        }
    }
}

/// Swap bytes for `u64` objects on all targets.
pub mod u64 {
    use std::cmp;
    use std::num::Int;
    use std::ptr;
    pub const BYTES: usize = 7;

    /// Swaps `len*8` bytes for `u64` objects inplace in `buf`.
    ///
    ///
    #[inline]
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
            let mut b: *mut u64 = buf as *mut u64;
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
    #[inline]
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
            let (mut d, mut s) = (dst as *mut u64, src as *const u64);
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

pub mod beunknown {
    use std::mem;
    use std::num::Int;
    use std::ptr;

    #[inline]
    pub fn decode(src: &[u8], nbytes: usize) -> u64 {
        assert!(src.len() >= nbytes);
        assert!(0 < nbytes && nbytes < 9);
        let mut dst = [0u8; 8];
        let ptr_out = dst.as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping_memory(
                ptr_out.offset((8 - nbytes) as isize),
                src.as_ptr(), nbytes);
            (*(ptr_out as *const u64)).to_be()
        }
    }

    #[inline]
    pub fn encode(dst: &mut [u8], src: u64, nbytes: usize) {
        assert!(dst.len() >= nbytes);
        assert!(0 < nbytes && nbytes < 9);
        unsafe {
            let bytes: &[u8; 8] = &mem::transmute::<_, [u8; 8]>(src.to_be());
            ptr::copy_nonoverlapping_memory(
                dst.as_mut_ptr(), (&bytes[8 - nbytes..]).as_ptr(), nbytes);
        }
    }
}

pub mod leunknown {
    use std::mem;
    use std::num::Int;
    use std::ptr;

    #[inline]
    pub fn decode(src: &[u8], nbytes: usize) -> u64 {
        assert!(src.len() >= nbytes);
        assert!(0 < nbytes && nbytes < 9);
        let mut dst = [0u8; 8];
        let ptr_out = dst.as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping_memory(
                ptr_out, src.as_ptr(), nbytes);
            (*(ptr_out as *const u64)).to_le()
        }
    }

    #[inline]
    pub fn encode(dst: &mut [u8], src: u64, nbytes: usize) {
        assert!(dst.len() >= nbytes);
        assert!(0 < nbytes && nbytes < 9);
        unsafe {
            let bytes: &[u8; 8] = &mem::transmute::<_, [u8; 8]>(src.to_le());
            ptr::copy_nonoverlapping_memory(dst.as_mut_ptr(), (&bytes[..nbytes]).as_ptr(), nbytes);
        }
    }
}


macro_rules! mod_odd_impls {
    ($I:ident, $T:ident, $S:ident, $Bytes:expr, $DFunc:ident, $EMeth:ident, $E:expr, $NotE:expr) => {
        use std::mem;
        use std::num::Int;
        use std::ptr;

        #[inline]
        unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
            if cfg!(target_endian = $NotE) {
                super::$T::swap_memory(dst, src, len);
            } else {
                ptr::copy_nonoverlapping_memory(dst, src, len*$Bytes);
            }
        }

        /// Decodes $E-endian bytes to a native-endian $T object.
        #[inline]
        pub fn decode(buf: &[u8]) -> $S {
            assert!(buf.len() >= $Bytes);
            unsafe {
                let mut tmp: $S = mem::uninitialized();
                ptr::copy_nonoverlapping_memory(&mut tmp as *mut _ as *mut u8, buf.as_ptr(), $Bytes);
                Int::$DFunc(tmp)
            }
        }

        /// Decodes $E-endian bytes to a slice of native-endian $T objects.
        #[inline]
        pub fn decode_slice(dst: &mut [[u8; $Bytes]], src: &[u8]) {
            assert!(dst.len()*$Bytes >= src.len());
            unsafe {
                swap_memory(dst.as_mut_ptr() as *mut u8, src.as_ptr(), dst.len());
            }
        }

        /// Encodes a native-endian $T object to $E-endian bytes.
        #[inline]
        pub fn encode(dst: &mut [u8], src: $S) {
            assert!(dst.len() >= $Bytes);
            unsafe {
                let tmp: $S = src.$EMeth();
                ptr::copy_nonoverlapping_memory(dst.as_mut_ptr(), &tmp as *const _ as *const u8, $Bytes);
            }
        }

        /// Encodes a slice of native-endian $T objects to $E-endian bytes.
        #[inline]
        pub fn encode_slice(dst: &mut [u8], src: &[[u8; $Bytes]]) {
            assert!(dst.len() >= src.len()*$Bytes);
            unsafe {
                swap_memory(dst.as_mut_ptr(), src.as_ptr() as *const u8, src.len());
            }
        }

    }
}

macro_rules! mod_std_impls {
    ($I:ident, $T:ident, $DFunc:ident, $EMeth:ident, $E:expr, $NotE:expr) => {
        use std::mem;
        use std::num::Int;
        use std::ptr;

        #[inline]
        unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
            if cfg!(target_endian = $NotE) {
                super::$T::swap_memory(dst, src, len);
            } else {
                ptr::copy_nonoverlapping_memory(dst, src, len*::std::$T::BYTES);
            }
        }

        /// Decodes $E-endian bytes to a native-endian $T object.
        #[inline]
        pub fn decode(buf: &[u8]) -> $T {
            assert!(buf.len() >= ::std::$T::BYTES);
            unsafe {
                let mut tmp: $T = mem::uninitialized();
                ptr::copy_nonoverlapping_memory(&mut tmp as *mut _ as *mut u8, buf.as_ptr(), ::std::$T::BYTES);
                Int::$DFunc(tmp)
            }
        }

        /// Decodes $E-endian bytes to a slice of native-endian $T objects.
        #[inline]
        pub fn decode_slice(dst: &mut [$T], src: &[u8]) {
            assert!(dst.len()*::std::$T::BYTES >= src.len());
            unsafe {
                swap_memory(dst.as_mut_ptr() as *mut u8, src.as_ptr(), dst.len());
            }
        }

        /// Encodes a native-endian $T object to $E-endian bytes.
        #[inline]
        pub fn encode(dst: &mut [u8], src: $T) {
            assert!(dst.len() >= ::std::$T::BYTES);
            unsafe {
                let tmp: $T = src.$EMeth();
                ptr::copy_nonoverlapping_memory(dst.as_mut_ptr(), &tmp as *const _ as *const u8, ::std::$T::BYTES);
            }
        }

        /// Encodes a slice of native-endian $T objects to $E-endian bytes.
        #[inline]
        pub fn encode_slice(dst: &mut [u8], src: &[$T]) {
            assert!(dst.len() >= src.len()*::std::$T::BYTES);
            unsafe {
                swap_memory(dst.as_mut_ptr(), src.as_ptr() as *const u8, src.len());
            }
        }

    }
}

/// Swap bytes for `[u8; 3]` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu24 { mod_odd_impls!(be, u24, u32, 3, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 5]` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu40 { mod_odd_impls!(be, u40, u64, 5, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 6]` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu48 { mod_odd_impls!(be, u48, u64, 6, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 7]` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu56 { mod_odd_impls!(be, u56, u64, 7, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 3]` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu24 { mod_odd_impls!(le, u24, u32, 3, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 5]` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu40 { mod_odd_impls!(le, u40, u64, 5, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 6]` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu48 { mod_odd_impls!(le, u48, u64, 6, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 7]` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu56 { mod_odd_impls!(le, u56, u64, 7, from_le, to_le, "little", "big"); }
/// Swap bytes for `u16` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu16 { mod_std_impls!(be, u16, from_be, to_be, "big", "little"); }
/// Swap bytes for `u16` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu16 { mod_std_impls!(le, u16, from_le, to_le, "little", "big"); }
/// Swap bytes for `u32` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu32 { mod_std_impls!(be, u32, from_be, to_be, "big", "little"); }
/// Swap bytes for `u32` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu32 { mod_std_impls!(le, u32, from_le, to_le, "little", "big"); }
/// Swap bytes for `u64` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu64 { mod_std_impls!(be, u64, from_be, to_be, "big", "little"); }
/// Swap bytes for `u64` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu64 { mod_std_impls!(le, u64, from_le, to_le, "little", "big"); }

mod tests;
