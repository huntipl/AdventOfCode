// https://datatracker.ietf.org/doc/html/rfc1321
// useful debug: https://fthb321.github.io/MD5-Hash/MD5OurVersion2.html
// https://www.comparitech.com/blog/information-security/md5-algorithm-with-examples/


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
            print!("M{i}:\t{b:08x}");
            i += 1;
            print!("\n");
        }
        println!("Len: {} bits", $b_arr.len() * 8 * 2);
    };
}

fn main() {
    // let _data = std::fs::read_to_string("input").unwrap();

    // let _desired_pre = "00000";

    print_endian();
    //
    // MD5
    // peter => "51dc30ddc473d43a6011e9ebba6ca770"

    let test = "peter".to_string();
    // let test = "They are deterministic".to_string();

    let input = test;
    // let _desired_result = "51dc30ddc473d43a6011e9ebba6ca770";

    // result - 128bit, 4x32bit

    println!(":::Input::: {input}");
    let mut input_bytes: Vec<u8> = Vec::new();
    input_bytes = input.into_bytes();
    println!(":::Converting to bytes::: {input_bytes:?}");

    let original_byte_len: u64 = input_bytes.len() as u64;
    let to_append: u8 = 56 - original_byte_len as u8;

    input_bytes.append(&mut vec![1 << 7]);
    input_bytes.append(&mut vec![0; to_append as usize - 1]);

    input_bytes.append(&mut (original_byte_len * 8).to_be_bytes().to_vec());

    let new_len = input_bytes.len();

    println!(":::Appending bytes::: {input_bytes:?} len:{new_len}");
    println!(":::Control print of bits:::");

    printb!(input_bytes);

    // M
    let M = vec![&input_bytes];
    let mut MM: Vec<u32> = vec![];
    for i in (0..input_bytes.len()).step_by(4) {
        let new_number: u32 = ((input_bytes[i] as u32) << 24)
            + ((input_bytes[i + 1] as u32) << 16)
            + ((input_bytes[i + 2] as u32) << 8)
            + ((input_bytes[i + 3] as u32) << 0);
        MM.push(new_number);
    }

    println!("32bit array:");
    print32h!(MM);

    // Preparation complete;

    let A = u32::from_str_radix("01234567", 16).unwrap();
    let B = u32::from_str_radix("89abcdef", 16).unwrap();
    let C = u32::from_str_radix("fedcba98", 16).unwrap();
    let D = u32::from_str_radix("76543210", 16).unwrap();

    let T: Vec<u32> = (0..64)
        .map(|x| (((x + 1) as f64).sin().abs() * 2_u64.pow(32) as f64) as u32)
        .collect();

    // println!("T matrix:");
    // print32h!(T);

    let k: Vec<u32> = vec![0,4,8,12,1,5,9,13,5,1,13,9,0,12,8,4,1,5,9,13,6,10,14,2,8,4,0,12,7,3,15,11,2,6,10,14,11,15,3,7,11,7,3,15,14,10,6,2,3,7,11,15,0,4,8,12,14,10,6,2,5,1,13,9];
    let s: Vec<u32> = vec![7,7,7,7,5,5,5,5,4,4,4,4,6,6,6,6,12,12,12,12,9,9,9,9,11,11,11,11,10,10,10,10,17,17,17,17,14,14,14,14,16,16,16,16,15,15,15,15,22,22,22,22,20,20,20,20,23,23,23,23,21,21,21,21,];
    let i: Vec<i32> = vec![];

    println!("{k:?} {s:?} {i:?}");
    panic!();

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
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: usize,
        s: u32,
        i: usize,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(F(b, c, d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i - 1 as usize].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(b); // 5
        a
    }

    fn R2(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: usize,
        s: u32,
        i: usize,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(G(b, c, d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i - 1 as usize].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(b); // 5
        a
    }

    fn R3(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: usize,
        s: u32,
        i: usize,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(H(b, c, d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i - 1 as usize].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(b); // 5
        a
    }

    fn R4(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        k: usize,
        s: u32,
        i: usize,
        X: &Vec<u32>,
        T: &Vec<u32>,
    ) -> u32 {
        let a = a.wrapping_add(I(b, c, d)); // 1
        let a = X[k as usize].wrapping_add(a); // 2
        let a = T[i - 1 as usize].wrapping_add(a); // 3
        let a = a.rotate_left(s); // 4
        let a = a.wrapping_add(b); // 5
        a
    }

    // Process in Blocks M[0..N] - we only have one 512bit block for our case
    // let X = &M[0];

    let AA = &A;
    let BB = &B;
    let CC = &C;
    let DD = &D;

    // Round 1

    let A = R1(A, B, C, D, 0, 7, 1, &MM, &T);
    let D = R1(D, A, B, C, 1, 12, 2, &MM, &T);
    let C = R1(C, D, A, B, 2, 17, 3, &MM, &T);
    let B = R1(B, C, D, A, 3, 22, 4, &MM, &T);

    let A = R1(A, B, C, D, 4, 7, 5, &MM, &T);
    let D = R1(D, A, B, C, 5, 12, 6, &MM, &T);
    let C = R1(C, D, A, B, 6, 17, 7, &MM, &T);
    let B = R1(B, C, D, A, 7, 22, 8, &MM, &T);

    let A = R1(A, B, C, D, 8, 7, 9, &MM, &T);
    let D = R1(D, A, B, C, 9, 12, 10, &MM, &T);
    let C = R1(C, D, A, B, 10, 17, 11, &MM, &T);
    let B = R1(B, C, D, A, 11, 22, 12, &MM, &T);

    let A = R1(A, B, C, D, 12, 7, 13, &MM, &T);
    let D = R1(D, A, B, C, 13, 12, 14, &MM, &T);
    let C = R1(C, D, A, B, 14, 17, 15, &MM, &T);
    let B = R1(B, C, D, A, 15, 22, 16, &MM, &T);

    // Round 2
    let A = R2(A, B, C, D, 1, 5, 17, &MM, &T);
    let D = R2(D, A, B, C, 6, 9, 18, &MM, &T);
    let C = R2(C, D, A, B, 11, 14, 19, &MM, &T);
    let B = R2(B, C, D, A, 0, 20, 20, &MM, &T);

    let A = R2(A, B, C, D, 5, 5, 21, &MM, &T);
    let D = R2(D, A, B, C, 10, 9, 22, &MM, &T);
    let C = R2(C, D, A, B, 15, 14, 23, &MM, &T);
    let B = R2(B, C, D, A, 4, 20, 24, &MM, &T);

    let A = R2(A, B, C, D, 9, 5, 25, &MM, &T);
    let D = R2(D, A, B, C, 14, 9, 26, &MM, &T);
    let C = R2(C, D, A, B, 3, 14, 27, &MM, &T);
    let B = R2(B, C, D, A, 8, 20, 28, &MM, &T);

    let A = R2(A, B, C, D, 13, 5, 29, &MM, &T);
    let D = R2(D, A, B, C, 2, 9, 30, &MM, &T);
    let C = R2(C, D, A, B, 7, 14, 31, &MM, &T);
    let B = R2(B, C, D, A, 12, 20, 32, &MM, &T);

    // Round 3
    let A = R3(A, B, C, D, 5, 4, 33, &MM, &T);
    let D = R3(D, A, B, C, 8, 11, 34, &MM, &T);
    let C = R3(C, D, A, B, 11, 16, 35, &MM, &T);
    let B = R3(B, C, D, A, 14, 23, 36, &MM, &T);

    let A = R3(A, B, C, D, 1, 4, 37, &MM, &T);
    let D = R3(D, A, B, C, 4, 11, 38, &MM, &T);
    let C = R3(C, D, A, B, 7, 16, 39, &MM, &T);
    let B = R3(B, C, D, A, 10, 23, 40, &MM, &T);

    let A = R3(A, B, C, D, 13, 4, 41, &MM, &T);
    let D = R3(D, A, B, C, 0, 11, 42, &MM, &T);
    let C = R3(C, D, A, B, 3, 16, 43, &MM, &T);
    let B = R3(B, C, D, A, 6, 23, 44, &MM, &T);

    let A = R3(A, B, C, D, 9, 4, 45, &MM, &T);
    let D = R3(D, A, B, C, 12, 11, 46, &MM, &T);
    let C = R3(C, D, A, B, 15, 16, 47, &MM, &T);
    let B = R3(B, C, D, A, 2, 23, 48, &MM, &T);

    // Round 4
    let A = R4(A, B, C, D, 0, 6, 49, &MM, &T);
    let D = R4(D, A, B, C, 7, 10, 50, &MM, &T);
    let C = R4(C, D, A, B, 14, 15, 51, &MM, &T);
    let B = R4(B, C, D, A, 5, 21, 52, &MM, &T);

    let A = R4(A, B, C, D, 12, 6, 53, &MM, &T);
    let D = R4(D, A, B, C, 3, 10, 54, &MM, &T);
    let C = R4(C, D, A, B, 10, 15, 55, &MM, &T);
    let B = R4(B, C, D, A, 1, 21, 56, &MM, &T);

    let A = R4(A, B, C, D, 8, 6, 57, &MM, &T);
    let D = R4(D, A, B, C, 15, 10, 58, &MM, &T);
    let C = R4(C, D, A, B, 6, 15, 59, &MM, &T);
    let B = R4(B, C, D, A, 13, 21, 60, &MM, &T);

    let A = R4(A, B, C, D, 4, 6, 61, &MM, &T);
    let D = R4(D, A, B, C, 1, 10, 62, &MM, &T);
    let C = R4(C, D, A, B, 2, 15, 63, &MM, &T);
    let B = R4(B, C, D, A, 9, 21, 64, &MM, &T);

    let A = A.wrapping_add(*AA);
    let B = B.wrapping_add(*BB);
    let C = C.wrapping_add(*CC);
    let D = D.wrapping_add(*DD);

    println!("{A:08x} {B:08x} {C:08x} {D:08x}");



    // time to mess about with the data
    // for x in (0..input_bytes.len()).step_by(2){
    //     println!("{},{}: {} {}", x, x+1, input_bytes[x], input_bytes[x+1]);

    //         /* Copy block i into X. */
    //      For j = 0 to 15 do
    //      Set X[j] to M[i*16+j].
    //    end /* of loop on j */
    // }

    // let X = M

    // use something smarter like: https://stackoverflow.com/questions/57029974/how-to-split-string-into-chunks-in-rust-to-insert-spaces
}
