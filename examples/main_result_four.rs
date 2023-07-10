/*challenge with Result
in progress
follow here
https://doc.rust-lang.org/rust-by-example/error/result/early_returns.html
*/

// works
// run command
// cargo run --example main_result_third

use rust_criterion_bench_comparing_fn::*;

fn slow(_input: u64) -> Result<u64, &'static str> {
    let _result: Result<u64, &str> = fibonacci_fast_result::<u64, &str>(_input);

    _result
}

fn print(result: Result<u64, &'static str>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    //println!("slow version => {:?} ", slow(8));
    print(slow(9));
    print(slow(0));

    // from here 
    // https://dev.to/kelvinkirima014/generating-the-nth-fibonacci-in-rust-238k
    for int in 0..15 {
        println! ( " {} {:?} ", int,print(slow(int)));
    }

    // Compiling error
    // print(slow('w'));
}
