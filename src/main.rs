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

fn to_upper_simd32(simd_slice: &u8x32) -> [u8; 32] {
    // cool vector code B)
    let char_mask = simd_slice.simd_ge(Simd::splat(b'a')) & simd_slice.simd_le(Simd::splat(b'z'));
    let out = char_mask.select(simd_slice.bitand(Simd::splat(0b11011111)), *simd_slice);
    *out.as_array()
}

fn to_upper_all(input: &str) -> String {
    let mut output = String::new();
    let (pre, simd_slices, suf) = input.as_bytes().as_simd::<32>();
    
    output.push_str(&str::from_utf8(pre).unwrap().to_uppercase());

    for chunk in simd_slices {
        output.push_str(&str::from_utf8(&to_upper_simd32(chunk)).unwrap())
    }

    output.push_str(&str::from_utf8(suf).unwrap().to_uppercase());

    output
}
