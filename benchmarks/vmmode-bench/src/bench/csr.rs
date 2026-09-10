use super::print_stats;
use super::poison_cache;

use core::arch::asm;
pub const SAMPLE_COUNT: usize = 10000000;

macro_rules! bench_csr_rw {
    ($csr_name:ident, $csr_str:expr, rw) => {
        paste::paste! {
            // この配列へのアクセスをすると稀にベンチマーク結果に大きな遅延が発生する。
            static mut [<RESULT_R_ $csr_name>]: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
            pub fn [<bench_read_ $csr_name:lower>]() {
                poison_cache(); // Poison the cache to ensure accurate timing
                let mut _val: u64;
                for i in 0..SAMPLE_COUNT {
                    let start: u64;
                    let end: u64;
                    unsafe {
                        asm!(
                            "rdcycle {start}",
                            concat!("csrr {val}, ", $csr_str), // ここがJITコンパイラによって、JIT生成されたエミュレーションコードへのジャンプに置き換えられる
                            "rdcycle {end}",
                            start = out(reg) start,
                            val = out(reg) _val,
                            end = out(reg) end,
                        );
                        // この配列へのアクセスが遅延の原因となることがある
                        [<RESULT_R_ $csr_name>][i] = end - start;

                        if end - start > 300 {
                            crate::println!("Warning: {} read took too long: {} cycles in iteration {}", $csr_str, end - start, i);
                        }
                    }
                }
                print_stats($csr_str, "read", unsafe { core::slice::from_raw_parts([<RESULT_R_ $csr_name>].as_ptr(), SAMPLE_COUNT) });
            }

            static mut [<RESULT_W_ $csr_name>]: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
            pub fn [<bench_write_ $csr_name:lower>]() {
                poison_cache(); // Poison the cache to ensure accurate timing
                
                let mut _val: u64 = 0;
                for i in 0..SAMPLE_COUNT {
                    let start: u64;
                    let end: u64;
                    unsafe {
                        asm!(
                            "rdcycle {start}",
                            concat!("csrw ", $csr_str, ", {val}"),
                            "rdcycle {end}",
                            start = out(reg) start,
                            val = in(reg) _val,
                            end = out(reg) end,
                        );
                        [<RESULT_W_ $csr_name>][i] = end - start;
                        if end - start > 300 {
                            crate::println!("Warning: {} write took too long: {} cycles in iteration {}", $csr_str, end - start, i);
                        }
                    }
                }
                print_stats($csr_str, "write", unsafe { core::slice::from_raw_parts([<RESULT_W_ $csr_name>].as_ptr(), SAMPLE_COUNT) });
            }
        }
    };

    ($csr_name:ident, $csr_str:expr, r) => {
        paste::paste! {
            static mut [<RESULT_R_ $csr_name>]: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
            pub fn [<bench_read_ $csr_name:lower>]() {
                poison_cache(); // Posion the cache to ensure accurate timing
                let mut _val: u64;
                for i in 0..SAMPLE_COUNT {
                    let start: u64;
                    let end: u64;
                    unsafe {
                        asm!(
                            "rdcycle {start}",
                            concat!("csrr {val}, ", $csr_str),
                            "rdcycle {end}",
                            start = out(reg) start,
                            val = out(reg) _val,
                            end = out(reg) end,
                        );
                        [<RESULT_R_ $csr_name>][i] = end - start;
                        if end - start > 300 {
                            crate::println!("Warning: {} read took too long: {} cycles in iteration {}", $csr_str, end - start, i);
                        }
                    }
                }
                print_stats($csr_str, "read", unsafe { core::slice::from_raw_parts([<RESULT_R_ $csr_name>].as_ptr(), SAMPLE_COUNT) });
            }

            pub fn [<bench_write_ $csr_name:lower>]() {
                // 空の関数として定義
            }
        }
    };

    ($csr_name:ident, $csr_str:expr, w) => {
        paste::paste! {
            pub fn [<bench_read_ $csr_name:lower>]() {
                // Empty function for read operation
            }

            static mut [<RESULT_W_ $csr_name>]: [u64; SAMPLE_COUNT] = [0; SAMPLE_COUNT];
            pub fn [<bench_write_ $csr_name:lower>]() {
                poison_cache(); // Posion the cache to ensure accurate timing
                let mut _val: u64 = 0;
                for i in 0..SAMPLE_COUNT {
                    let start: u64;
                    let end: u64;
                    unsafe {
                        asm!(
                            "rdcycle {start}",
                            concat!("csrw ", $csr_str, ", {val}"),
                            "rdcycle {end}",
                            start = out(reg) start,
                            val = in(reg) _val,
                            end = out(reg) end,
                        );
                        [<RESULT_W_ $csr_name>][i] = end - start;
                        if end - start > 300 {
                            crate::println!("Warning: {} write took too long: {} cycles in iteration {}", $csr_str, end - start, i);
                        }
                    }
                }
                print_stats($csr_str, "write", unsafe { core::slice::from_raw_parts([<RESULT_W_ $csr_name>].as_ptr(), SAMPLE_COUNT) });
            }
        }
    };
}

macro_rules! define_bench_csrs {
    ($($csr_name:ident => ($csr_str:expr, $mode:tt)),* $(,)?) => {
        $(
            bench_csr_rw!($csr_name, $csr_str, $mode);
        )*
        paste::paste! {
            pub fn bench_all() {
                $(
                    [<bench_read_ $csr_name:lower>]();
                    [<bench_write_ $csr_name:lower>]();
                )*
            }
        }
    };
}

define_bench_csrs! {
    MCAUSE   => ("mcause", rw),
    // MIE      => ("mie", r),
    // MEDELEG  => ("medeleg", rw),
    // MEPC     => ("mepc", rw),
    // MHARTID  => ("mhartid", r),
    // MIDELEG  => ("mideleg", rw),
    // MIE      => ("mie", rw),
    // MIP      => ("mip", rw),
    // MISA     => ("misa", rw),
    // MSCRATCH => ("mscratch", rw),
    // MSTATUS  => ("mstatus", rw),
    // MTVAL    => ("mtval", rw),
    // MTVEC    => ("mtvec", rw),
}
