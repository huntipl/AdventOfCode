// https://datatracker.ietf.org/doc/html/rfc1321
// useful debug: https://fthb321.github.io/MD5-Hash/MD5OurVersion2.html
// https://www.comparitech.com/blog/information-security/md5-algorithm-with-examples/

// endianess https://www.youtube.com/watch?v=NcaiHcBvDR4

// Debug info: https://rosettacode.org/wiki/MD5/Implementation_Debug
#![allow(non_snake_case)]

pub mod algo;

use std::{process::exit};

use crate::algo::md5;

fn main() {

    for x in 0..u64::MAX {
        let input = format!("ckczppom{x}");
        let result = md5(&input);
        let s_res = result[0..6].to_string();

        if s_res == "000000" {
            println!("{x}");
            exit(0)
        }
    }
}
