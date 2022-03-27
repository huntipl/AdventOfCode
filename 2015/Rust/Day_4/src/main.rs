// https://datatracker.ietf.org/doc/html/rfc1321
// useful debug: https://fthb321.github.io/MD5-Hash/MD5OurVersion2.html
// https://www.comparitech.com/blog/information-security/md5-algorithm-with-examples/

// endianess https://www.youtube.com/watch?v=NcaiHcBvDR4

// Debug info: https://rosettacode.org/wiki/MD5/Implementation_Debug

#![allow(unused_macros)]
#![allow(non_snake_case)]

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
    let debug = false;

    // let _desired_pre = "00000";

    let input = String::from("aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggg");
    let input = String::from("a");

    let mut input_bytes: Vec<u8> = input.clone().into_bytes();

    let original_byte_len: u64 = input_bytes.len() as u64;
    let to_append: u8 = 56 - original_byte_len as u8;

    let mut append_bytes: Vec<u8> = vec![];

    append_bytes.append(&mut vec![1 << 7]);
    append_bytes.append(&mut vec![0; to_append as usize - 1]);
    append_bytes.append(&mut (original_byte_len * 8).to_le_bytes().to_vec());

    input_bytes.append(&mut append_bytes);

    // ------------------Input Preparation complete------------------

    // M
    // let M = vec![&input_bytes]; // TODO split input in 512 chunks here
    let mut MM: Vec<u32> = vec![];
    for i in (0..input_bytes.len()).step_by(4) {
        let new_number: u32 = ((input_bytes[i + 0] as u32) << 0)
            + ((input_bytes[i + 1] as u32) << 8)
            + ((input_bytes[i + 2] as u32) << 16)
            + ((input_bytes[i + 3] as u32) << 24);
        MM.push(new_number);
    }

    if debug {
        println!(":::Input::: '{input}'");

        println!(":::Input bytes:::");
        printx!(input_bytes);

        println!(":::Appending bytes:::");
        printx!(append_bytes);

        println!(":::Control print of bits:::");
        // printb!(input_bytes);
        printx!(input_bytes);

        println!(":::Chunk split in words: [M0..M15]:::");
        print32h!(MM);
    }

    // those are reversed from the RFC doc as they are written in low order byte first
    let mut A = u32::from_str_radix("67452301", 16).unwrap();
    let mut B = u32::from_str_radix("EFCDAB89", 16).unwrap();
    let mut C = u32::from_str_radix("98BADCFE", 16).unwrap();
    let mut D = u32::from_str_radix("10325476", 16).unwrap();

    // T matrix, sine function
    let T: Vec<u32> = (0..64)
        .map(|x| (((x + 1) as f64).sin().abs() * 2_u64.pow(32) as f64) as u32)
        .collect();

    let k: Vec<u32> = (0..64)
        .into_iter()
        .map(|i| -> u32 {
            match i {
                0..=15 => i,
                16..=31 => (5 * i + 1) % 16,
                32..=47 => (3 * i + 5) % 16,
                48..=63 => (7 * i) % 16,
                _ => panic!("k out of range"),
            }
        })
        .collect();
    let s: Vec<u32> = vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let i: Vec<u32> = (1..=64).collect();

    let F = |X: u32, Y: u32, Z: u32| X & Y | !X & Z;
    let G = |X: u32, Y: u32, Z: u32| X & Z | Y & !Z;
    let H = |X: u32, Y: u32, Z: u32| X ^ Y ^ Z;
    let I = |X: u32, Y: u32, Z: u32| Y ^ (X | !Z);

    fn R(
        f: &dyn Fn(u32, u32, u32) -> u32,
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
        let a = a.wrapping_add(f(*b, *c, *d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[(i as usize) - 1].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(*b); // 5
        a
    }

    let AA = A;
    let BB = B;
    let CC = C;
    let DD = D;

    // Round 1

    for r in 0..4 {
        let mut step = 0;

        A = R(
            &F,
            &A,
            &B,
            &C,
            &D,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        D = R(
            &F,
            &D,
            &A,
            &B,
            &C,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        C = R(
            &F,
            &C,
            &D,
            &A,
            &B,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        B = R(
            &F,
            &B,
            &C,
            &D,
            &A,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );
    }

    // Round 2

    for r in 4..8 {
        let mut step = 0;

        A = R(
            &G,
            &A,
            &B,
            &C,
            &D,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        D = R(
            &G,
            &D,
            &A,
            &B,
            &C,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        C = R(
            &G,
            &C,
            &D,
            &A,
            &B,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        B = R(
            &G,
            &B,
            &C,
            &D,
            &A,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

    }

    // Round 3

    for r in 8..12 {
        let mut step = 0;

        A = R(
            &H,
            &A,
            &B,
            &C,
            &D,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        D = R(
            &H,
            &D,
            &A,
            &B,
            &C,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        C = R(
            &H,
            &C,
            &D,
            &A,
            &B,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        B = R(
            &H,
            &B,
            &C,
            &D,
            &A,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );
    }

    // Round 4

    for r in 12..16 {
        let mut step = 0;

        A = R(
            &I,
            &A,
            &B,
            &C,
            &D,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        D = R(
            &I,
            &D,
            &A,
            &B,
            &C,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        C = R(
            &I,
            &C,
            &D,
            &A,
            &B,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );

        step += 1;
        B = R(
            &I,
            &B,
            &C,
            &D,
            &A,
            k[r * 4 + step],
            s[r * 4 + step],
            i[r * 4 + step],
            &MM,
            &T,
        );
    }

    let A = A.wrapping_add(AA);
    let B = B.wrapping_add(BB);
    let C = C.wrapping_add(CC);
    let D = D.wrapping_add(DD);

    let A = A.to_be();
    let B = B.to_be();
    let C = C.to_be();
    let D = D.to_be();

    println!(":::MD5:::");
    let md5 = format!("{A:08x}{B:08x}{C:08x}{D:08x}");

    println!("{}", md5);
}
