#[inline]
pub fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

pub fn fibonacci_slow_result<T, E>(n: u64, my_err: &'static str) -> Result<u64, &'static str> {
    match n {
        0 => Err("Fibonacci start with at 1"),
        1 => Ok(1),
        _ => { let _ = fibonacci_slow_result::<u64, &'static str>(n - 1, my_err);
            // { fibonacci_slow_result(n - 1,my_err + fibonacci_slow_result(n - 2,my_err));

            Ok(n)
        }
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
pub fn fibonacci_fast_result<T, E>(n: u64) -> Result<u64, &'static str> {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    match n {
        0 => Err("Fibonacci start with at 1"),
        _ => {
            for _ in 0..n {
                let c: u64 = a + b;
                a = b;
                b = c;
            }
            Ok(b)
        }
    }
}
