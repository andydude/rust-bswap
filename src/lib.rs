
/// Swap bytes for `u8` slices on all targets.
pub mod u8 {
    use std::ptr;
    use std::mem;
    use std::error;
    use std::fmt;
    pub const BYTES: usize = 1;

    /// TODO
    #[inline]
    pub unsafe fn align_of_ptr(src: *const u8) -> usize {
        let off: usize = mem::transmute(src);
        (2 as usize).pow(off.trailing_zeros() as u32)
    }

    /// TODO
    #[inline]
    pub fn reverse_slice_inplace(buf: &mut [u8]) {
        buf.reverse();
    }

    /// TODO
    #[inline]
    pub fn reverse_slice(dst: &mut [u8], src: &[u8]) {
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(),
			             dst.as_mut_ptr(),
                                            src.len());
        }
        dst.reverse();
    }

    /// TODO
    #[inline]
    pub unsafe fn reverse_memory_inplace(buf: *mut u8, count: usize) {
        use std::ptr::swap;
        let n: usize = count >> 1;
        let b = buf;
        for i in 0..n {
            swap(b.offset(i as isize),
                      b.offset((count - 1 - i) as isize));
        }
    }

    /// TODO
    #[inline]
    pub unsafe fn reverse_memory(dst: *mut u8, src: *const u8, count: usize) {
        let (mut d, mut s) = (dst, src);
        s = s.offset((count - 1) as isize);
        for _ in 0..count {
            *d = *s;
            d = d.offset(1);
            s = s.offset(-1);
        }
    }

    pub unsafe fn reverse_memory_array_inplace(buf: *mut u8, count: usize, size: usize) {
        use std::ptr::swap;
        let mut b = buf;
        for _ in 0..count {
            for i in 0..(size/2) {
                swap(b.offset(i as isize),
                          b.offset((size - 1 - i) as isize));
            }
            b = b.offset(size as isize);
        }
    }

    pub unsafe fn reverse_memory_array(dst: *mut u8, src: *const u8, count: usize, size: usize) {
        let (mut d, mut s) = (dst, src);
        for _ in 0..count {
            for i in 0..size {
                *(d.offset(i as isize)) = *(s.offset((size - 1 - i) as isize));
            }
            d = d.offset(size as isize);
            s = s.offset(size as isize);
        }
    }

    /// Errors that can occur when decoding a hex encoded string
    #[derive(Copy, Clone)]
    pub enum FromHexError {
        /// The input contained a character not part of the hex format
        InvalidHexCharacter(char, usize),
        /// The input had an invalid length
        InvalidHexLength,
    }
    impl error::Error for FromHexError {
        fn description(&self) -> &str {
            match *self {
                FromHexError::InvalidHexCharacter(_, _) => "Invalid character '{}' at position {}",
                FromHexError::InvalidHexLength => "Invalid length",
            }
        }
    }
    impl fmt::Debug for FromHexError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use std::error::Error;
            write!(f, "{}", self.description())
        }
    }
    impl fmt::Display for FromHexError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self, f)
        }
    }

    pub fn decode_hex(src: &str) -> Result<Vec<u8>, FromHexError> {
        // This may be an overestimate if there is any whitespace
        let mut b = Vec::with_capacity(src.len() / 2);
        let mut modulus = 0;
        let mut buf = 08;

        for (idx, byte) in src.bytes().enumerate() {
            buf <<= 4;

            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                b' '|b'\r'|b'\n'|b'\t' => {
                    buf >>= 4;
                    continue
                }
                _ => return Err(FromHexError::InvalidHexCharacter('?' /*src[idx]*/, idx)),
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(FromHexError::InvalidHexLength),
        }
    }

    pub fn encode_hex(src: &[u8]) -> String {
        static CHARS: &'static[u8] = b"0123456789abcdef";
        let mut v = Vec::with_capacity(src.len() * 2);
        for &byte in src.iter() {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        }

        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}

/// Swap bytes for `u16` objects on all targets.
pub mod u16 {
    pub const BYTES: usize = 2;

    /// TODO
    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let mut b = buf;
        for _ in 0..len {
            swap(b.offset(0), b.offset(1));
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
    pub const BYTES: usize = 3;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let mut b = buf;
        for _ in 0..len {
            swap(b.offset(0), b.offset(2));
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
    pub const BYTES: usize = 5;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let mut b = buf;
        for _ in 0..len {
            swap(b.offset(0), b.offset(4));
            swap(b.offset(1), b.offset(3));
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
    pub const BYTES: usize = 6;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let mut b = buf;
        for _ in 0..len {
            swap(b.offset(0), b.offset(5));
            swap(b.offset(1), b.offset(4));
            swap(b.offset(2), b.offset(3));
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
    pub const BYTES: usize = 7;

    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {
        use std::ptr::swap;
        let mut b = buf;
        for _ in 0..len {
            swap(b.offset(0), b.offset(6));
            swap(b.offset(1), b.offset(5));
            swap(b.offset(2), b.offset(4));
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
    pub const BYTES: usize = 4;


    /// Swaps `len*4` bytes for `u32` objects inplace in `buf`.
    ///
    ///
    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {

        unsafe fn u32_swap_memory_inplace_u8(buf: *mut u8, len: usize) {
            use std::ptr::swap;
            let mut b = buf;
            for _ in 0..len {
                swap(b.offset(0), b.offset(3));
                swap(b.offset(1), b.offset(2));
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
    pub const BYTES: usize = 7;

    /// Swaps `len*8` bytes for `u64` objects inplace in `buf`.
    ///
    ///
    #[inline]
    pub unsafe fn swap_memory_inplace(buf: *mut u8, len: usize) {

        unsafe fn u64_swap_memory_inplace_u8(buf: *mut u8, len: usize) {
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

pub mod beusize {
    use std::ptr;
    use std::mem;

    #[inline]
    pub fn decode(src: &[u8], nbytes: usize) -> u64 {
        assert_eq!(src.len(), nbytes);
        assert!(0 < nbytes && nbytes <= 8);
        let mut dst = [0u8; 8];
        let ptr_out = dst.as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping(
                src.as_ptr(), ptr_out.offset((8 - nbytes) as isize), nbytes);
            (*(ptr_out as *const u64)).to_be()
        }
    }

    #[inline]
    pub fn encode(dst: &mut [u8], src: u64, nbytes: usize) {
        assert_eq!(dst.len(), nbytes);
        assert!(0 < nbytes && nbytes <= 8);
        unsafe {
            // n.b. https://github.com/rust-lang/rust/issues/22776
            let bytes: [u8; 8] = mem::transmute::<_, [u8; 8]>(src.to_be());
            ptr::copy_nonoverlapping(
                (&bytes[8 - nbytes..]).as_ptr(), dst.as_mut_ptr(), nbytes);
        }
    }
}

pub mod leusize {
    use std::ptr;
    use std::mem;

    #[inline]
    pub fn decode(src: &[u8], nbytes: usize) -> u64 {
        assert_eq!(src.len(), nbytes);
        assert!(0 < nbytes && nbytes <= 8);
        let mut dst = [0u8; 8];
        let ptr_out = dst.as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping(
                src.as_ptr(), ptr_out, nbytes);
            (*(ptr_out as *const u64)).to_le()
        }
    }

    #[inline]
    pub fn encode(dst: &mut [u8], src: u64, nbytes: usize) {
        assert_eq!(dst.len(), nbytes);
        assert!(0 < nbytes && nbytes <= 8);
        unsafe {
            // n.b. https://github.com/rust-lang/rust/issues/22776
            let bytes: [u8; 8] = mem::transmute::<_, [u8; 8]>(src.to_le());
            ptr::copy_nonoverlapping(
	        (&bytes[..nbytes]).as_ptr(), dst.as_mut_ptr(), nbytes);
        }
    }
}


macro_rules! mod_odd_impls {
    ($I:ident, $T:ident, $S:ident, $Bytes:expr, $DFunc:ident, $EMeth:ident, $E:expr, $NotE:expr) => {
        use std::ptr;
        use std::mem;

        #[inline]
        unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
            if cfg!(target_endian = $NotE) {
                super::$T::swap_memory(dst, src, len);
            } else {
                ptr::copy_nonoverlapping(dst, src, len*$Bytes);
            }
        }

        /// Decodes $E-endian bytes to a native-endian $T object.
        #[inline]
        pub fn decode(buf: &[u8]) -> $S {
            assert_eq!(buf.len(), $Bytes);
            unsafe {
                let mut tmp: $S = mem::uninitialized();
                ptr::copy_nonoverlapping(&mut tmp as *mut _ as *mut u8, buf.as_ptr(), $Bytes);
                $T::$DFunc(tmp)
            }
        }

        /// Decodes $E-endian bytes to a slice of native-endian $T objects.
        #[inline]
        pub fn decode_slice(dst: &mut [[u8; $Bytes]], src: &[u8]) {
            assert_eq!(dst.len()*$Bytes, src.len());
            unsafe {
                swap_memory(dst.as_mut_ptr() as *mut u8, src.as_ptr(), dst.len());
            }
        }

        /// Encodes a native-endian $T object to $E-endian bytes.
        #[inline]
        pub fn encode(dst: &mut [u8], src: $S) {
            assert_eq!(dst.len(), $Bytes);
            unsafe {
                let tmp: $S = src.$EMeth();
                ptr::copy_nonoverlapping(&tmp as *const _ as *const u8, dst.as_mut_ptr(), $Bytes);
            }
        }

        /// Encodes a slice of native-endian $T objects to $E-endian bytes.
        #[inline]
        pub fn encode_slice(dst: &mut [u8], src: &[[u8; $Bytes]]) {
            assert_eq!(dst.len(), src.len()*$Bytes);
            unsafe {
                swap_memory(dst.as_mut_ptr(), src.as_ptr() as *const u8, src.len());
            }
        }

    }
}

macro_rules! mod_std_impls {
    ($I:ident, $T:ident, $Bytes:expr, $DFunc:ident, $EMeth:ident, $E:expr, $NotE:expr) => {
        use std::ptr;
        use std::mem;

        #[inline]
        unsafe fn swap_memory(dst: *mut u8, src: *const u8, len: usize) {
            if cfg!(target_endian = $NotE) {
                super::$T::swap_memory(dst, src, len);
            } else {
                ptr::copy_nonoverlapping(src, dst, len*$Bytes);
            }
        }

        /// Decodes $E-endian bytes to a native-endian $T object.
        #[inline]
        pub fn decode(buf: &[u8]) -> $T {
            assert_eq!(buf.len(), $Bytes);
            unsafe {
                let mut tmp: $T = mem::uninitialized();
                ptr::copy_nonoverlapping(buf.as_ptr(), &mut tmp as *mut _ as *mut u8, $Bytes);
                $T::$DFunc(tmp)
            }
        }

        /// Decodes $E-endian bytes to a slice of native-endian $T objects.
        #[inline]
        pub fn decode_slice(dst: &mut [$T], src: &[u8]) {
            assert_eq!(dst.len()*$Bytes, src.len());
            unsafe {
                swap_memory(dst.as_mut_ptr() as *mut u8, src.as_ptr(), dst.len());
            }
        }

        /// Encodes a native-endian $T object to $E-endian bytes.
        #[inline]
        pub fn encode(dst: &mut [u8], src: $T) {
            assert_eq!(dst.len(), $Bytes);
            unsafe {
                let tmp: $T = src.$EMeth();
                ptr::copy_nonoverlapping(&tmp as *const _ as *const u8, dst.as_mut_ptr(), $Bytes);
            }
        }

        /// Encodes a slice of native-endian $T objects to $E-endian bytes.
        #[inline]
        pub fn encode_slice(dst: &mut [u8], src: &[$T]) {
            assert_eq!(dst.len(), src.len()*$Bytes);
            unsafe {
                swap_memory(dst.as_mut_ptr(), src.as_ptr() as *const u8, src.len());
            }
        }

    }
}

/// Swap bytes for `u16` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu16 { mod_std_impls!(be, u16, 2, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 3]` objects only on little-endian targets, does nothing on big-endian targets.
//pub mod beu24 { mod_odd_impls!(be, u24, u32, 3, from_be, to_be, "big", "little"); }
/// Swap bytes for `u32` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu32 { mod_std_impls!(be, u32, 4, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 5]` objects only on little-endian targets, does nothing on big-endian targets.
//pub mod beu40 { mod_odd_impls!(be, u40, u64, 5, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 6]` objects only on little-endian targets, does nothing on big-endian targets.
//pub mod beu48 { mod_odd_impls!(be, u48, u64, 6, from_be, to_be, "big", "little"); }
/// Swap bytes for `[u8; 7]` objects only on little-endian targets, does nothing on big-endian targets.
//pub mod beu56 { mod_odd_impls!(be, u56, u64, 7, from_be, to_be, "big", "little"); }
/// Swap bytes for `u64` objects only on little-endian targets, does nothing on big-endian targets.
pub mod beu64 { mod_std_impls!(be, u64, 8, from_be, to_be, "big", "little"); }
/// Swap bytes for `u16` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu16 { mod_std_impls!(le, u16, 2, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 3]` objects only on big-endian targets, does nothing on little-endian targets.
//pub mod leu24 { mod_odd_impls!(le, u24, u32, 3, from_le, to_le, "little", "big"); }
/// Swap bytes for `u32` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu32 { mod_std_impls!(le, u32, 4, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 5]` objects only on big-endian targets, does nothing on little-endian targets.
//pub mod leu40 { mod_odd_impls!(le, u40, u64, 5, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 6]` objects only on big-endian targets, does nothing on little-endian targets.
//pub mod leu48 { mod_odd_impls!(le, u48, u64, 6, from_le, to_le, "little", "big"); }
/// Swap bytes for `[u8; 7]` objects only on big-endian targets, does nothing on little-endian targets.
//pub mod leu56 { mod_odd_impls!(le, u56, u64, 7, from_le, to_le, "little", "big"); }
/// Swap bytes for `u64` objects only on big-endian targets, does nothing on little-endian targets.
pub mod leu64 { mod_std_impls!(le, u64, 8, from_le, to_le, "little", "big"); }

mod tests;
