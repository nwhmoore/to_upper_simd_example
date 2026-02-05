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

fn to_upper_all(input: &str) -> String {
    let mut output = Vec::with_capacity(input.len());
    let (pre, simd_slices, suf) = input.as_bytes().as_simd::<32>();

    output.extend(pre.to_ascii_uppercase());

    for chunk in simd_slices {
        let char_mask = chunk.simd_ge(Simd::splat(b'a')) & chunk.simd_le(Simd::splat(b'z'));
        let out = char_mask.select(chunk.bitand(Simd::splat(0b11011111)), *chunk);

        output.extend(out.as_array())
    }

    output.extend(suf.to_ascii_uppercase());

    String::from_utf8(output).unwrap()
}
