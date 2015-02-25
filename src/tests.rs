#![cfg(test)]
use test::Bencher;

const LOREM_IPSUM: &'static [u8] = b"\t\n\n\tSed ut perspiciatis, unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam eaque ipsa, quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt, explicabo. nemo enim ipsam voluptatem, quia voluptas sit, aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos, qui ratione voluptatem sequi nesciunt, neque porro quisquam est, qui dolorem ipsum, quia dolor sit, amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt, ut labore et dolore magnam aliquam quaerat voluptatem. ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? quis autem vel eum iure reprehenderit, qui in ea voluptate velit esse, quam nihil molestiae consequatur, vel illum, qui dolorem eum fugiat, quo voluptas nulla pariatur?\t\n\n\tAt vero eos et accusamus et iusto odio dignissimos ducimus, qui blanditiis praesentium voluptatum deleniti atque corrupti, quos dolores et quas molestias excepturi sint, obcaecati cupiditate non provident, similique sunt in culpa, qui officia deserunt mollitia animi, id est laborum et dolorum fuga. et harum quidem rerum facilis est et expedita distinctio. nam libero tempore, cum soluta nobis est eligendi optio, cumque nihil impedit, quo minus id, quod maxime placeat, facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet, ut et voluptates repudiandae sint et molestiae non recusandae. itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.\t\n\n\t";

const MUSPI_MEROL: &'static [u8] = b"\t\n\n\t.talleper seroirepsa subirolod sidnerefrep tua rutauqesnoc saila seroiam subitatpulov sidneicier tua tu ,sutceled etneipas a rutenet cih murer murae euqati .eadnasucer non eaitselom te tnis eadnaiduper setatpulov te tu ,teineve epeas subitatissecen murer tua sitibed siiciffo tua te madsubiuq metua subiropmet .sudnelleper rolod sinmo ,tse adnemussa satpulov sinmo ,sumissop erecaf ,taecalp emixam douq ,di sunim ouq ,tidepmi lihin euqmuc ,oitpo idnegile tse sibon atulos muc ,eropmet orebil man .oitcnitsid atidepxe te tse silicaf murer mediuq murah te .aguf murolod te murobal tse di ,imina aitillom tnuresed aiciffo iuq ,apluc ni tnus euqilimis ,tnedivorp non etatidipuc itaceacbo ,tnis irutpecxe saitselom sauq te serolod souq ,itpurroc euqta itineled mutatpulov muitnesearp siitidnalb iuq ,sumicud somissingid oido otsui te sumasucca te soe orev tA\t\n\n\t?rutairap allun satpulov ouq ,taiguf mue merolod iuq ,mulli lev ,rutauqesnoc eaitselom lihin mauq ,esse tilev etatpulov ae ni iuq ,tiredneherper erui mue lev metua siuq ?rutauqesnoc idommoc ae xe diuqila tu isin ,masoirobal tipicsus siroproc mallu menoitaticrexe murtson siuq ,mainev aminim da mine tu .metatpulov tareauq mauqila mangam erolod te erobal tu ,tnudicni aropmet idom suie mauqmun non aiuq des ,tilev icsipida ,rutetcesnoc ,tema ,tis rolod aiuq ,muspi merolod iuq ,tse mauqsiuq orrop euqen ,tnuicsen iuqes metatpulov enoitar iuq ,soe serolod ingam rutnuuqesnoc aiuq des ,tiguf tua tido tua rutanrepsa ,tis satpulov aiuq ,metatpulov maspi mine omen .obacilpxe ,tnus atcid eativ eataeb otcetihcra isauq te sitatirev erotnevni olli ba eauq ,aspi euqae mairepa mer matot ,muitnadual euqmerolod muitnasucca metatpulov tis rorre sutan etsi sinmo ednu ,sitaicipsrep tu deS\t\n\n\t";

const LOREM_IPSUM_160_U64_SWAP_MEMORY: &'static [u8] = b" deS\t\n\n\tpsrep tu,sitaicimo ednu etsi sine sutan tis rorratpulov ucca met muitnasqmerolodadual eut ,muitnmer matomairepa i euqae auq ,asplli ba etnevni oirev ero";

#[test]
fn test_u8_reverse_memory_u8() {
    let mut dst = [0u8; 11];
    let src: &[u8] = b"hello world";
    unsafe {
        super::u8::reverse_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len());
    }
    assert_eq!(&dst[..], b"dlrow olleh");
}

#[test]
fn test_u8_reverse_memory_u64() {
    let mut dst = [0u8; 8];
    let src: &[u8] = b"hi world";
    unsafe {
        super::u8::reverse_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len());
    }
    assert_eq!(&dst[..], b"dlrow ih");
}

#[test]
fn test_u8_reverse_memory_u8x16_lorem_ipsum() {
    let mut dst = [0u8; 160];
    let src: &[u8] = &LOREM_IPSUM[..160];
    unsafe {
        super::u8::reverse_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len());
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    println!("exp = '{}'", String::from_utf8((&MUSPI_MEROL[1582..]).to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], &MUSPI_MEROL[1582..]);
}

#[test]
fn test_u8_reverse_memory_lorem_ipsum() {
    let mut dst = [0u8; 1742];
    let src: &[u8] = LOREM_IPSUM;
    assert_eq!(dst.len(), LOREM_IPSUM.len());
    assert_eq!(dst.len(), MUSPI_MEROL.len());
    unsafe {
        super::u8::reverse_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len());
    }
    assert_eq!(&dst[..], MUSPI_MEROL);
}

#[test]
fn test_u8_reverse_memory_muspi_merol() {
    let mut dst = [0u8; 1742];
    let src: &[u8] = MUSPI_MEROL;
    assert_eq!(dst.len(), LOREM_IPSUM.len());
    assert_eq!(dst.len(), MUSPI_MEROL.len());
    unsafe {
        super::u8::reverse_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len());
    }
    assert_eq!(&dst[..], LOREM_IPSUM);
}

#[bench]
fn bench_u8_reverse_memory_lorem_ipsum(b: & mut Bencher) {
    let mut dst = [0u8; 1742];
    let src: &[u8] = LOREM_IPSUM;
    b.iter(
        || {
            unsafe {
                super::u8::reverse_memory(
                    (&mut dst[..]).as_mut_ptr(),
                    src.as_ptr(), src.len());
            }
        }
    );
    b.bytes = src.len() as u64;
}

#[bench]
fn bench_u8_reverse_memory_muspi_merol(b: & mut Bencher) {
    let mut dst = [0u8; 1742];
    let src: &[u8] = MUSPI_MEROL;
    b.iter(
        || {
            unsafe {
                super::u8::reverse_memory(
                    (&mut dst[..]).as_mut_ptr(),
                    src.as_ptr(), src.len());
            }
        }
    );
    b.bytes = src.len() as u64;
}

#[bench]
fn bench_u8_reverse_memory_ones_1000(b: & mut Bencher) {
    let mut dst = [0u8; 1000];
    let src = [1u8; 1000];
    b.iter(
        || {
            unsafe {
                super::u8::reverse_memory(
                    (&mut dst[..]).as_mut_ptr(),
                    (&src[..]).as_ptr(), src.len());
            }
        }
    );
    b.bytes = src.len() as u64;
}

#[bench]
fn bench_u8_reverse_memory_ones_1024(b: & mut Bencher) {
    let mut dst = [0u8; 1024];
    let src = [1u8; 1024];
    b.iter(
        || {
            unsafe {
                super::u8::reverse_memory(
                    (&mut dst[..]).as_mut_ptr(),
                    (&src[..]).as_ptr(), src.len());
            }
        }
    );
    b.bytes = src.len() as u64;
}

#[test]
fn test_u64_swap_memory_hi() {
    let mut dst = [0u8; 8];
    let src: &[u8] = b"hi world";
    unsafe {
        super::u64::swap_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len()/8);
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], b"dlrow ih");
}

#[test]
fn test_u64_swap_memory_inplace_hi() {
    let mut dst = [0u8; 8];
    let src: &[u8] = b"hi world";
    ::std::slice::bytes::copy_memory(&mut dst, src);
    unsafe {
        super::u64::swap_memory_inplace(
            (&mut dst[..]).as_mut_ptr(),
            dst.len()/8);
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], b"dlrow ih");
}

#[test]
fn test_u64_swap_memory_pangram() {
    let mut dst = [0u8; 32];
    let src: &[u8] = b"Five boxing wizards jump quickly";
    unsafe {
        super::u64::swap_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len()/8);
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], b"xob eviFaziw gnipmuj sdrylkciuq ");
}

#[test]
fn test_u64_swap_memory_inplace_pangram() {
    let mut dst = [0u8; 32];
    let src: &[u8] = b"Five boxing wizards jump quickly";
    ::std::slice::bytes::copy_memory(&mut dst, src);
    unsafe {
        super::u64::swap_memory_inplace(
            (&mut dst[..]).as_mut_ptr(),
            dst.len()/8);
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], b"xob eviFaziw gnipmuj sdrylkciuq ");
}

#[test]
fn test_u64_swap_memory_lorem_ipsum() {
    let mut dst = [0u8; 160];
    let src: &[u8] = &LOREM_IPSUM[..160];
    unsafe {
        super::u64::swap_memory(
            (&mut dst[..]).as_mut_ptr(),
            src.as_ptr(), src.len()/8);
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    println!("exp = '{}'", String::from_utf8((LOREM_IPSUM_160_U64_SWAP_MEMORY).to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], LOREM_IPSUM_160_U64_SWAP_MEMORY);
}

#[test]
fn test_u64_swap_memory_inplace_lorem_ipsum() {
    let mut dst = [0u8; 160];
    let src: &[u8] = &LOREM_IPSUM[..160];
    ::std::slice::bytes::copy_memory(&mut dst, src);
    unsafe {
        super::u64::swap_memory_inplace(
            (&mut dst[..]).as_mut_ptr(),
            dst.len()/8);
    }
    println!("src = '{}'", String::from_utf8(src.to_vec()).unwrap().as_slice());
    println!("dst = '{}'", String::from_utf8(dst.to_vec()).unwrap().as_slice());
    println!("exp = '{}'", String::from_utf8((LOREM_IPSUM_160_U64_SWAP_MEMORY).to_vec()).unwrap().as_slice());
    assert_eq!(&dst[..], LOREM_IPSUM_160_U64_SWAP_MEMORY);
}

//
// tests from Burntsushi/byteorder (modified)
//

fn qc_sized<T: Sized>(f: fn(n: T) -> bool, size: T) {
    assert!(f(size));
}

macro_rules! qc_byte_order {
    ($name:ident, $ty_int:ident, $ty_uint:ident, $bytes:expr, $bemod:ident, $lemod:ident) => (
        mod $name {
            use super::qc_sized;

            #[test]
            fn big_endian() {
                let max = ::std::$ty_int::MAX as u64 - 1 >> (8 * (8 - $bytes));
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 8];
                    //println!("big_endian({:016x})", n);
                    super::super::beunknown::encode(&mut buf[8 - $bytes..], n as u64, $bytes);
                    //println!("{}", (&buf[..]).to_hex());
                    let m = super::super::beunknown::decode(&mut buf[8 - $bytes..], $bytes) as $ty_int;
                    //println!("{:016x}", m);
                    n == m
                }
                qc_sized(prop as fn($ty_int) -> bool, max as $ty_int - 1);
            }

            #[test]
            fn little_endian() {
                let max = ::std::$ty_int::MAX as u64 - 1 >> (8 * (8 - $bytes));
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 8];
                    //println!("little_endian({:016x})", n);
                    super::super::leunknown::encode(&mut buf[..$bytes], n as u64, $bytes);
                    //println!("{}", (&buf[..]).to_hex());
                    let m = super::super::leunknown::decode(&mut buf[..$bytes], $bytes) as $ty_int;
                    //println!("{:016x}", m);
                    n == m
                }
                qc_sized(prop as fn($ty_int) -> bool, max as $ty_int - 1);
            }
        }
    );
    ($name:ident, $ty_int:ident, $ty_uint:ident, $bemod:ident, $lemod:ident) => (
        mod $name {
            use super::qc_sized;

            #[test]
            fn big_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 8];
                    super::super::$bemod::encode(&mut buf[(8 - ::std::$ty_int::BYTES)..], n as $ty_uint);
                    n == super::super::$bemod::decode(&mut buf[(8 - ::std::$ty_int::BYTES)..]) as $ty_int
                }
                qc_sized(prop as fn($ty_int) -> bool,
                         ::std::$ty_int::MAX as $ty_int - 1);
            }

            #[test]
            fn little_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 8];
                    super::super::$lemod::encode(&mut buf[..(::std::$ty_int::BYTES)], n as $ty_uint);
                    n == super::super::$lemod::decode(&mut buf[..(::std::$ty_int::BYTES)]) as $ty_int
                }
                qc_sized(prop as fn($ty_int) -> bool,
                         ::std::$ty_int::MAX as $ty_int - 1);
            }
        }
    );
}

qc_byte_order!(prop_u16, u16, u16, beu16, leu16);
qc_byte_order!(prop_i16, i16, u16, beu16, leu16);
qc_byte_order!(prop_u32, u32, u32, beu32, leu32);
qc_byte_order!(prop_i32, i32, u32, beu32, leu32);
qc_byte_order!(prop_u64, u64, u64, beu64, leu64);
qc_byte_order!(prop_i64, i64, u64, beu64, leu64);

qc_byte_order!(prop_uint_1, u64, u64, 1, beunknown, leunknown);
qc_byte_order!(prop_uint_2, u64, u64, 2, beunknown, leunknown);
qc_byte_order!(prop_uint_3, u64, u64, 3, beunknown, leunknown);
qc_byte_order!(prop_uint_4, u64, u64, 4, beunknown, leunknown);
qc_byte_order!(prop_uint_5, u64, u64, 5, beunknown, leunknown);
qc_byte_order!(prop_uint_6, u64, u64, 6, beunknown, leunknown);
qc_byte_order!(prop_uint_7, u64, u64, 7, beunknown, leunknown);
qc_byte_order!(prop_uint_8, u64, u64, 8, beunknown, leunknown);

qc_byte_order!(prop_int_1, i64, u64, 1, beunknown, leunknown);
qc_byte_order!(prop_int_2, i64, u64, 2, beunknown, leunknown);
qc_byte_order!(prop_int_3, i64, u64, 3, beunknown, leunknown);
qc_byte_order!(prop_int_4, i64, u64, 4, beunknown, leunknown);
qc_byte_order!(prop_int_5, i64, u64, 5, beunknown, leunknown);
qc_byte_order!(prop_int_6, i64, u64, 6, beunknown, leunknown);
qc_byte_order!(prop_int_7, i64, u64, 7, beunknown, leunknown);
qc_byte_order!(prop_int_8, i64, u64, 8, beunknown, leunknown);

// Test that all of the byte conversion functions panic when given a
// buffer that is too small.
//
// These tests are critical to ensure safety, otherwise we might end up
// with a buffer overflow.
macro_rules! too_small {
    ($name:ident, $maximally_small:expr, $zero:expr, $bemod:ident, $lemod:ident) => (
        mod $name {

            #[test]
            #[should_fail]
            fn read_big_endian() {
                let buf = [0; $maximally_small];
                super::super::$bemod::decode(&buf);
            }

            #[test]
            #[should_fail]
            fn read_little_endian() {
                let buf = [0; $maximally_small];
                super::super::$lemod::decode(&buf);
            }

            #[test]
            #[should_fail]
            fn write_big_endian() {
                let mut buf = [0; $maximally_small];
                super::super::$bemod::encode(&mut buf, $zero);
            }

            #[test]
            #[should_fail]
            fn write_little_endian() {
                let mut buf = [0; $maximally_small];
                super::super::$lemod::encode(&mut buf, $zero);
            }
        }
    );
    ($name:ident, $maximally_small:expr, $bemod:ident, $lemod:ident) => (
        mod $name {

            #[test]
            #[should_fail]
            fn read_big_endian() {
                let buf = [0; $maximally_small];
                super::super::$bemod::decode(&buf, $maximally_small + 1);
            }

            #[test]
            #[should_fail]
            fn read_little_endian() {
                let buf = [0; $maximally_small];
                super::super::$lemod::decode(&buf, $maximally_small + 1);
            }
        }
    );
}

too_small!(small_u16, 1, 0, beu16, leu16);
too_small!(small_i16, 1, 0, beu16, leu16);
too_small!(small_u32, 3, 0, beu32, leu32);
too_small!(small_i32, 3, 0, beu32, leu32);
too_small!(small_u64, 7, 0, beu64, leu64);
too_small!(small_i64, 7, 0, beu64, leu64);

too_small!(small_uint_1, 1, beunknown, leunknown);
too_small!(small_uint_2, 2, beunknown, leunknown);
too_small!(small_uint_3, 3, beunknown, leunknown);
too_small!(small_uint_4, 4, beunknown, leunknown);
too_small!(small_uint_5, 5, beunknown, leunknown);
too_small!(small_uint_6, 6, beunknown, leunknown);
too_small!(small_uint_7, 7, beunknown, leunknown);

too_small!(small_int_1, 1, beunknown, leunknown);
too_small!(small_int_2, 2, beunknown, leunknown);
too_small!(small_int_3, 3, beunknown, leunknown);
too_small!(small_int_4, 4, beunknown, leunknown);
too_small!(small_int_5, 5, beunknown, leunknown);
too_small!(small_int_6, 6, beunknown, leunknown);
too_small!(small_int_7, 7, beunknown, leunknown);

#[test]
fn test_read_bytes_ext() {
    let rdr = vec![2, 5, 3, 0];
    assert_eq!(517, super::beu16::decode(&rdr[0..2]));
    assert_eq!(768, super::beu16::decode(&rdr[2..4]));
}

#[test]
fn test_write_bytes_ext() {
    let mut wtr = vec![0; 4];
    super::beu16::encode(&mut wtr[0..2], 517);
    super::beu16::encode(&mut wtr[2..4], 768);
    assert_eq!(wtr, vec![2, 5, 3, 0]);
}
