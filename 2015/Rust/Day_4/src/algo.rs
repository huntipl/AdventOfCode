#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]

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


pub fn md5(input: &str) -> String {
    let debug = false;

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
        X: &[u32],
        t: u32,
    ) -> u32 {
        let a = a.wrapping_add(f(*b, *c, *d));
        let a = X[k as usize].wrapping_add(a);
        let a = t.wrapping_add(a);
        let a = a.rotate_left(s);
        a.wrapping_add(*b)
    }

    let mut input_bytes: Vec<u8> = input.to_string().into_bytes();

    let original_bit_len: i32 = input_bytes.len() as i32 * 8;

    let to_append: u8;

    if original_bit_len % 512 < 448 {
        to_append = ((448 - (original_bit_len % 512)) / 8) as u8;
    } else {
        to_append = ((512 - (448 - (original_bit_len % 512))) / 8) as u8;
    }

    // TODO handle irregular bit len, not only bytes

    // TODO handle input with lengths greater than 64bit

    let mut append_bytes: Vec<u8> = vec![];

    append_bytes.append(&mut vec![1 << 7]);
    append_bytes.append(&mut vec![0; to_append as usize - 1]);
    let mut len_to_append = (original_bit_len as u64).to_le_bytes().to_vec();
    append_bytes.append(&mut len_to_append);

    input_bytes.append(&mut append_bytes);

    if debug {
        println!(":::Input::: '{input}'");
        println!(":::Input len bits::: {}", input.len() * 8);

        println!(":::Input bytes:::");
        printx!(input_bytes);

        println!(":::Appending bytes:::");
        printx!(append_bytes);

        println!(":::Control print of bits:::");
        // printb!(input_bytes);
        printx!(input_bytes);
    }

    // ------------------Input Preparation complete------------------

    let M = input_bytes.chunks_exact(64);

    for chunk in M {
        let mut MM: Vec<u32> = vec![];
        for i in (0..chunk.len()).step_by(4) {
            let new_number: u32 = 
                  ((chunk[i + 0] as u32) << 0)
                + ((chunk[i + 1] as u32) << 8)
                + ((chunk[i + 2] as u32) << 16)
                + ((chunk[i + 3] as u32) << 24);
            MM.push(new_number);
        }

        if debug {
            println!(":::Input::: '{input}'");
            println!(":::Input len bits::: {}", input.len() * 8);

            println!(":::chunk bytes:::");
            printx!(*chunk);

            println!(":::Chunk split in words: [M0..M15]:::");
            print32h!(MM);
        }

        let AA = A;
        let BB = B;
        let CC = C;
        let DD = D;

        // Round 1

        for r in 0..16 {
            let mut index = r * 4;

            match r {
                0..=3 => {
                    A = R(&F, &A, &B, &C, &D, k[index], s[index], &MM, T[index]);
                    index += 1;
                    D = R(&F, &D, &A, &B, &C, k[index], s[index], &MM, T[index]);
                    index += 1;
                    C = R(&F, &C, &D, &A, &B, k[index], s[index], &MM, T[index]);
                    index += 1;
                    B = R(&F, &B, &C, &D, &A, k[index], s[index], &MM, T[index]);
                }
                4..=7 => {
                    A = R(&G, &A, &B, &C, &D, k[index], s[index], &MM, T[index]);
                    index += 1;
                    D = R(&G, &D, &A, &B, &C, k[index], s[index], &MM, T[index]);
                    index += 1;
                    C = R(&G, &C, &D, &A, &B, k[index], s[index], &MM, T[index]);
                    index += 1;
                    B = R(&G, &B, &C, &D, &A, k[index], s[index], &MM, T[index]);
                }
                8..=11 => {
                    A = R(&H, &A, &B, &C, &D, k[index], s[index], &MM, T[index]);
                    index += 1;
                    D = R(&H, &D, &A, &B, &C, k[index], s[index], &MM, T[index]);
                    index += 1;
                    C = R(&H, &C, &D, &A, &B, k[index], s[index], &MM, T[index]);
                    index += 1;
                    B = R(&H, &B, &C, &D, &A, k[index], s[index], &MM, T[index]);
                }
                12..=15 => {
                    A = R(&I, &A, &B, &C, &D, k[index], s[index], &MM, T[index]);
                    index += 1;
                    D = R(&I, &D, &A, &B, &C, k[index], s[index], &MM, T[index]);
                    index += 1;
                    C = R(&I, &C, &D, &A, &B, k[index], s[index], &MM, T[index]);
                    index += 1;
                    B = R(&I, &B, &C, &D, &A, k[index], s[index], &MM, T[index]);
                }
                _ => panic!("out of scope"),
            }
        }

        A = A.wrapping_add(AA);
        B = B.wrapping_add(BB);
        C = C.wrapping_add(CC);
        D = D.wrapping_add(DD);
    }

    A = A.to_be();
    B = B.to_be();
    C = C.to_be();
    D = D.to_be();

    format!("{A:08x}{B:08x}{C:08x}{D:08x}").to_ascii_uppercase()
}