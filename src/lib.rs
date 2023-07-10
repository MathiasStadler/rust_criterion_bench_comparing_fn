#[inline]
pub fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

#[inline]
pub fn fibonacci_fast(n: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c: u64 = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}


#[inline]
pub fn fibonacci_fast_result(n: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c: u64 = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
