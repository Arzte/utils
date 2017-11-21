#![feature(test)]

extern crate serenity_utils;
extern crate test;

use test::{Bencher, black_box};

#[bench]
fn bench_parse_emoji(b: &mut Bencher) {
    b.iter(|| {
        serenity_utils::parse_emoji("<:smugAnimeFace:302516740095606785>");
    });
}
