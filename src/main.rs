#![feature(portable_simd)]
use std::{
    ops::BitAnd,
    simd::{Select, Simd, prelude::*},
};

fn main() {
    let input = "ThiS iS A TeSt StrinG That is that Is longer \
    Than 32 chaRActers LOOngggg!! we can even test longer strings and \
    it still WORks?!! Throw in some SpEcIaL chars for good !@#%#$!*%% ing measure";
    println!("{}", to_upper_all(input));
}

fn to_upper_simd32(bytes: [u8; 32]) -> [u8; 32] {
    // cool vector code B)
    let simd_bytes = Simd::from(bytes);
    let char_mask = simd_bytes.simd_ge(Simd::splat(b'a')) & simd_bytes.simd_le(Simd::splat(b'z'));
    let out = char_mask.select(simd_bytes.bitand(Simd::splat(0b11011111)), simd_bytes);

    out.to_array()
}

fn to_upper_all(input: &str) -> String {
    let mut output = String::new();

    // boring built-in code T.T for remainder
    let remainder = input.len() % 32;
    output.push_str(&input[0..remainder].to_uppercase());

    // load in chunks of string for cool vector code B)
    for chunk in 0..input.len() / 32 {
        output.push_str(
            str::from_utf8(&to_upper_simd32(
                <[u8; 32]>::try_from(
                    input[remainder + chunk * 32..remainder + (chunk + 1) * 32].as_bytes(),
                )
                .unwrap(),
            ))
            .unwrap(),
        )
    }
    output
}
