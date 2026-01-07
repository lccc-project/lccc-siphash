macro_rules! barrier{
    () => {
        unsafe{core::arch::asm!("mov rsi, rbx; cpuid; mov rbx, rsi", inlateout("eax") 0 => _, lateout("ecx") _, lateout("edx") _, out("esi") _);}
    }
}

const RUNS: u128 = 1_000_000;
const WRITE_COUNT: u128 = 10_000;

macro_rules! time{
    {
        loop{$($s:tt)*}$(then{$($e:tt)*})?
    } => {
        {
            let start = std::time::Instant::now();
            barrier!();

            for _ in 0..WRITE_COUNT{
                $($s)*
            }
            $($($e)*)?
            barrier!();
            let time = start.elapsed();
            time.as_nanos()
        }
    }
}

pub struct Stats<'a> {
    label: &'a str,
    min: u128,
    max: u128,
    mean: u128,
}

impl<'a> core::fmt::Display for Stats<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Benchmark {} ({} iterations): min - {}, max - {}, mean - {}",
            self.label, RUNS, self.min, self.max, self.mean
        ))
    }
}

pub fn baseline() -> u128 {
    time! {
        loop{unsafe{core::arch::asm!("");}}
    }
}
pub fn sipround() -> u128 {
    let mut hasher = lccc_siphash::SipHashState::from_keys(
        core::hint::black_box(0x6a09e667f3bcc908),
        core::hint::black_box(0xbb67ae8584caa73b),
    );
    time! {
        loop{
            hasher.round();
        }then{
            core::hint::black_box(hasher.finish());
        }
    }
}

pub fn siphash<const C: usize, const D: usize>() -> u128 {
    use core::hash::Hasher;
    let ikey = core::hint::black_box(0x428a2f98d728ae22);
    let mut hasher = lccc_siphash::SipHasher::<C, D>::new_with_keys(
        core::hint::black_box(0x6a09e667f3bcc908),
        core::hint::black_box(0xbb67ae8584caa73b),
    );
    time! {
        loop{
            hasher.write_u64(core::hint::black_box(ikey));
        }then{
            core::hint::black_box(hasher.finish());
        }
    }
}

pub fn raw_siphash<const C: usize, const D: usize>() -> u128 {
    use core::hash::Hasher;
    let ikey = core::hint::black_box(0x428a2f98d728ae22);
    let mut hasher = lccc_siphash::RawSipHasher::<C, D>::from_keys(
        core::hint::black_box(0x6a09e667f3bcc908),
        core::hint::black_box(0xbb67ae8584caa73b),
    );
    time! {
        loop{
            hasher.write_u64(core::hint::black_box(ikey));
        }then{
            core::hint::black_box(hasher.finish());
        }
    }
}

pub fn std_hasher() -> u128 {
    use core::hash::Hasher;
    let ikey = core::hint::black_box(0x428a2f98d728ae22);
    let mut hasher = std::hash::DefaultHasher::new();
    time! {
        loop{
            hasher.write_u64(core::hint::black_box(ikey));
        }then{
            core::hint::black_box(hasher.finish());
        }
    }
}

pub fn run_benchmark(label: &str, bench_fn: fn() -> u128) -> Stats {
    let mut min = !0;
    let mut max = 0;
    let mut total = 0;

    println!("Benchmarking {} ({} Iterations)", label, RUNS);

    for _ in 0..RUNS {
        let val = bench_fn();
        total += val;
        min = min.min(val);
        max = max.max(val);
    }

    Stats {
        label,
        min,
        max,
        mean: total / RUNS,
    }
}

fn main() {
    let benches = [
        run_benchmark("Baseline", baseline),
        run_benchmark("Sipround", sipround),
        run_benchmark("SipHash-1-3", siphash::<1, 3>),
        run_benchmark("SipHash-2-4", siphash::<2, 4>),
        run_benchmark("RawSipHash-1-3", raw_siphash::<1, 3>),
        run_benchmark("RawSipHash-2-4", raw_siphash::<2, 4>),
        run_benchmark("std", std_hasher),
    ];

    for bench in benches {
        println!("{}", bench)
    }
}
