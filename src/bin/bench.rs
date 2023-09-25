#![feature(hashmap_internals)]
#![allow(deprecated)]
use std::{hash::Hasher, time::Instant};

use siphash_opt::{siphash::sys::SipHashState, SipHasher};

const WRITES: u64 = 1_000;
const TRIES: u128 = 1_000_000;

macro_rules! barrier{
    () => {
        unsafe{
            core::arch::asm!("xchg rsi, rbx; cpuid; xchg rsi, rbx", out("eax") _, out("ecx") _, out("esi") _, out("edx") _);
        }
    }
}

fn baseline() -> u128 {
    let count = std::hint::black_box(WRITES);
    let start = Instant::now();
    barrier!();
    for _ in 0..count {
        unsafe {
            core::arch::asm!("");
        }
    }
    barrier!();
    start.elapsed().as_nanos()
}

fn bench_sipround() -> u128 {
    let mut hasher = SipHashState::from_keys(
        std::hint::black_box(0x6a09e667f3bcc908),
        std::hint::black_box(0xbb67ae8584caa73b),
    );

    let count = std::hint::black_box(WRITES);

    let start = Instant::now();
    barrier!();
    for _ in 0..count {
        hasher.round();
    }
    std::hint::black_box(hasher.finish());
    barrier!();
    start.elapsed().as_nanos()
}

fn bench_siphash<const C: usize, const D: usize>() -> u128 {
    let ikey = std::hint::black_box(0x428a2f98d728ae22u64);
    let mut hasher = SipHasher::<C, D>::new_with_keys(
        std::hint::black_box(0x6a09e667f3bcc908),
        std::hint::black_box(0xbb67ae8584caa73b),
    );

    let count = std::hint::black_box(WRITES);

    let start = Instant::now();
    barrier!();
    for _ in 0..count {
        hasher.update(ikey);
    }
    std::hint::black_box(hasher.finish());
    barrier!();
    start.elapsed().as_nanos()
}

fn bench_std_siphash_1_3() -> u128 {
    let ikey = std::hint::black_box(0x428a2f98d728ae22u64);
    let mut hasher = std::hash::SipHasher13::new_with_keys(
        std::hint::black_box(0x6a09e667f3bcc908),
        std::hint::black_box(0xbb67ae8584caa73b),
    );

    let count = std::hint::black_box(WRITES);

    let start = Instant::now();
    barrier!();
    for _ in 0..count {
        hasher.write_u64(ikey);
    }
    std::hint::black_box(hasher.finish());
    barrier!();
    start.elapsed().as_nanos()
}

fn bench_std_siphash_2_4() -> u128 {
    let ikey = std::hint::black_box(0x428a2f98d728ae22u64);
    let mut hasher = std::hash::SipHasher::new_with_keys(
        std::hint::black_box(0x6a09e667f3bcc908),
        std::hint::black_box(0xbb67ae8584caa73b),
    );

    let count = std::hint::black_box(WRITES);

    let start = Instant::now();
    barrier!();
    for _ in 0..count {
        hasher.write_u64(ikey);
    }
    std::hint::black_box(hasher.finish());
    barrier!();
    start.elapsed().as_nanos()
}

struct Stats<'a> {
    name: &'a str,
    min: u128,
    max: u128,
    average: u128,
}

impl<'a> core::fmt::Display for Stats<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: Minimum - {}, Maximum - {}, Average - {}",
            self.name, self.min, self.max, self.average
        ))
    }
}

fn run_microbenchmark(bench: fn() -> u128, name: &str) -> Stats {
    let mut min = !0;
    let mut max = 0;
    let mut total = 0;
    eprintln!("Benchmarking {}: {} times", name, TRIES);
    for _ in 0..TRIES {
        let val = bench();
        min = min.min(val);
        max = max.max(val);
        total += val;
    }

    Stats {
        min,
        max,
        average: total / TRIES,
        name,
    }
}

pub fn main() {
    let base = run_microbenchmark(baseline, "Baseline");
    let round = run_microbenchmark(bench_sipround, "SipRound");
    let sip1_3 = run_microbenchmark(bench_siphash::<1, 3>, "SipHash-1-3");
    let sip2_4 = run_microbenchmark(bench_siphash::<2, 4>, "SipHash-2-4");
    let std_sip1_3 = run_microbenchmark(bench_std_siphash_1_3, "SipHash-1-3 (standard library)");
    let std_sip2_4 = run_microbenchmark(bench_std_siphash_2_4, "SipHash-2-4 (standard library)");

    println!("{}", base);
    println!("{}", round);
    println!("{}", sip1_3);
    println!("{}", sip2_4);
    println!("{}", std_sip1_3);
    println!("{}", std_sip2_4);
}
