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
