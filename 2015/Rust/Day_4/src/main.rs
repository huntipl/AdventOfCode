// https://datatracker.ietf.org/doc/html/rfc1321
// useful debug: https://fthb321.github.io/MD5-Hash/MD5OurVersion2.html
// https://www.comparitech.com/blog/information-security/md5-algorithm-with-examples/

// Debug info: https://rosettacode.org/wiki/MD5/Implementation_Debug

#![allow(unused_macros)]
#![allow(non_snake_case)]

#[cfg(target_endian = "big")]
fn print_endian() {
    println!("Big endian")
}

#[cfg(target_endian = "little")]
fn print_endian() {
    println!("Little endian")
}

macro_rules! printb {
    // print 8bit
    ($b_arr:expr) => {
        let mut count = 0;
        for b in &$b_arr {
            print!("{b:08b} ");
            count += 1;
            if count == 8 {
                count = 0;
                print!("\n");
            }
        }
        println!("Len: {} bits", $b_arr.len() * 8);
    };
}

macro_rules! printx {
    // print 8bit as hex
    ($b_arr:expr) => {
        let mut count = 0;
        for b in &$b_arr {
            print!("{b:02x} ");
            count += 1;
            if count == 10 {
                count = 0;
                print!("\n");
            }
        }
        println!("Len: {} bits", $b_arr.len() * 8);
    };
}


// macro_rules! printbb {
//     // print 16bit
//     ($b_arr:expr) => {
//         let mut count = 0;
//         for b in &$b_arr {
//             print!("{b:016b} ");
//             count += 1;
//             if count == 4 {
//                 count = 0;
//                 print!("\n");
//             }
//         }
//         println!("Len: {} bits", $b_arr.len() * 8 * 2);
//     };
// }

macro_rules! print32h {
    // print 32bit hex
    ($b_arr:expr) => {
        let mut i = 0;
        for b in &$b_arr {
            print!("{i}:\t{b:08x}");
            i += 1;
            print!("\n");
        }
        println!("Len: {} bits", $b_arr.len() * 8 * 2);
    };
}

macro_rules! print32 {
    // print 32bit dec
    ($b_arr:expr) => {
        let mut i = 0;
        for b in &$b_arr {
            print!("M{i}:\t{b}");
            i += 1;
            print!("\n");
        }
        println!("Len: {} bits", $b_arr.len() * 8 * 2);
    };
}

fn main() {
    // let _data = std::fs::read_to_string("input").unwrap();

    // let _desired_pre = "00000";

    let input = String::from("");
    // let _desired_result = "51dc30ddc473d43a6011e9ebba6ca770";

    println!(":::Input::: '{input}'");
    
    let mut input_bytes: Vec<u8> = Vec::new();
    input_bytes = input.into_bytes();
    println!(":::Input bytes:::");
    printx!(input_bytes);

    let original_byte_len: u64 = input_bytes.len() as u64;
    let to_append: u8 = 56 - original_byte_len as u8;

    let mut append_bytes: Vec<u8> = vec![];

    append_bytes.append(&mut vec![1 << 7]);
    append_bytes.append(&mut vec![0; to_append as usize - 1]);
    append_bytes.append(&mut (original_byte_len * 8).to_le_bytes().to_vec());

    println!(":::Appending bytes:::");
    printx!(append_bytes);
    input_bytes.append(&mut append_bytes);

    println!(":::Control print of bits:::");
    // printb!(input_bytes);
    printx!(input_bytes);

    

    // ------------------Input Preparation complete------------------

    // M
    let M = vec![&input_bytes];
    let mut MM: Vec<u32> = vec![];
    for i in (0..input_bytes.len()).step_by(4) {
        let new_number: u32 = 
              ((input_bytes[i + 0] as u32) << 0)
            + ((input_bytes[i + 1] as u32) << 8)
            + ((input_bytes[i + 2] as u32) << 16)
            + ((input_bytes[i + 3] as u32) << 24);
        MM.push(new_number);
    }

    // dec -> bits -> byte -> 32bit
    // 10 -> 00001010 -> 0a -> 00 00 00 0a (Big Endian)
    // 0a 00 00 00 (little endian)

    // in out "" exapmple:
    // 80 00 00 00 ... -> 00 00 00 80 


    println!(":::Chunk split in words: [M0..M15]:::");
    print32h!(MM);

    // those are reversed from the RFC doc as they are written in low order byte first
    let mut A = u32::from_str_radix("67452301", 16).unwrap();
    let mut B = u32::from_str_radix("EFCDAB89", 16).unwrap();
    let mut C = u32::from_str_radix("98BADCFE", 16).unwrap();
    let mut D = u32::from_str_radix("10325476", 16).unwrap();


    // println!("::: A B C D PRE:::");
    // println!("A: {A:#08x}\nB: {B:#08x}\nC: {C:#08x}\nD: {D:#08x}");


    // T matrix, sine function
    let T: Vec<u32> = (0..64)
        .map(|x| (((x + 1) as f64).sin().abs() * 2_u64.pow(32) as f64) as u32)
        .collect();

    // println!(":::T matrix:::");
    // print32h!(T);
    // print32!(T);

    let k: Vec<u32> = (0..64).into_iter().map(|i| -> u32 {
        match i {
            0..=15 => {i}
            16..=31 => {(5*i+1)%16}
            32..=47 => {(3*i+5)%16}
            48..=63 => {(7*i)%16}
            _ => panic!("k out of range")
        }
    }).collect();
    let s: Vec<u32> = vec![7,12,17,22,7,12,17,22,7,12,17,22,7,12,17,22,5,9,14,20,5,9,14,20,5,9,14,20,5,9,14,20,4,11,16,23,4,11,16,23,4,11,16,23,4,11,16,23,6,10,15,21,6,10,15,21,6,10,15,21,6,10,15,21];
    // let i: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64];
    let i: Vec<u32> = (1..=64).collect();

    // println!(":::k vector:::");
    // println!("{k:?}");

    // println!(":::s vector:::");
    // println!("{s:?}");

    // println!(":::i vector:::");
    // println!("{i:?}");


    fn F(X: u32, Y: u32, Z: u32) -> u32 {
        X & Y | !X & Z
    }
    fn G(X: u32, Y: u32, Z: u32) -> u32 {
        X & Z | Y & !Z
    }
    fn H(X: u32, Y: u32, Z: u32) -> u32 {
        X ^ Y ^ Z
    }
    fn I(X: u32, Y: u32, Z: u32) -> u32 {
        Y ^ (X | !Z)
    }

    fn R1(
        a: &u32,
        b: &u32,
        c: &u32,
        d: &u32,
        k: u32,
        s: u32,
        i: u32,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(F(*b, *c, *d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[(i as usize) - 1].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(*b); // 5
        a
    }

    fn R2(
        a: &u32,
        b: &u32,
        c: &u32,
        d: &u32,
        k: u32,
        s: u32,
        i: u32,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(G(*b, *c, *d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i as usize - 1].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(*b); // 5
        a
    }

    fn R3(
        a: &u32,
        b: &u32,
        c: &u32,
        d: &u32,
        k: u32,
        s: u32,
        i: u32,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(H(*b, *c, *d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i as usize - 1].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(*b); // 5
        a
    }

    fn R4(
        a: &u32,
        b: &u32,
        c: &u32,
        d: &u32,
        k: u32,
        s: u32,
        i: u32,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(I(*b, *c, *d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i as usize - 1].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(*b); // 5
        a
    }

    // Process in Blocks M[0..N] - we only have one 512bit block for our case
    // let X = &M[0];


    let AA = A;
    let BB = B;
    let CC = C;
    let DD = D;

    // Round 1

    for r in 0..4 {
        // println!(":::Round 1:{r}:::");
        let mut step = 0;

        A = R1(&A, &B, &C, &D, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        D = R1(&D, &A, &B, &C, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        C = R1(&C, &D, &A, &B, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        B = R1(&B, &C, &D, &A, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        
    }
    // println!("After round 1: {A:x?} {B:x?} {C:x?} {D:x?}");

    // Round 2

    for r in 4..8 {
        // println!(":::Round 2:{r}:::");
        let mut step = 0;

        A = R2(&A, &B, &C, &D, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        D = R2(&D, &A, &B, &C, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        C = R2(&C, &D, &A, &B, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        B = R2(&B, &C, &D, &A, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
    }
    // println!("After round 2: {A:x?} {B:x?} {C:x?} {D:x?}");

    // Round 3

    for r in 8..12 {
        // println!(":::Round 3:{r}:::");
        let mut step = 0;

        A = R3(&A, &B, &C, &D, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        D = R3(&D, &A, &B, &C, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        C = R3(&C, &D, &A, &B, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        B = R3(&B, &C, &D, &A, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
    }
    // println!("After round 3: {A:x?} {B:x?} {C:x?} {D:x?}");

    // Round 4

    for r in 12..16 {
        // println!(":::Round 4:{r}:::");
        let mut step = 0;

        A = R4(&A, &B, &C, &D, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        D = R4(&D, &A, &B, &C, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        C = R4(&C, &D, &A, &B, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
        step += 1;
        B = R4(&B, &C, &D, &A, k[r*4+step], s[r*4+step], i[r*4+step], &MM, &T);
        // println!("k={} s={} i={} T={:x?}", k[r*4+step], s[r*4+step], i[r*4+step], T[i[r*4+step] as usize -1]);
        // println!("After {step} step: {A:x?} {B:x?} {C:x?} {D:x?}");
    }
    // println!("After round 4: {A:x?} {B:x?} {C:x?} {D:x?}");

    // println!("Final prints:");
    // println!("After all rounds: A {A:x?} {B:x?} {C:x?} {D:x?}");
    // println!("After all rounds: AA {AA:x?} {BB:x?} {CC:x?} {DD:x?}");

    let A = A.wrapping_add(AA);
    let B = B.wrapping_add(BB);
    let C = C.wrapping_add(CC);
    let D = D.wrapping_add(DD);

    // println!("After final increment {A:x?} {B:x?} {C:x?} {D:x?}");

    let A = A.to_be();
    let B = B.to_be();
    let C = C.to_be();
    let D = D.to_be();

    println!(":::MD5:::");
    println!("{A:08x}{B:08x}{C:08x}{D:08x}");

}
