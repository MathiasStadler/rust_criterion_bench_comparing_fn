/*
challenge with Result
in progress
*/

use rust_criterion_bench_comparing_fn::*;

fn slow(_input: u64) -> u64 {
    let _result: u64 = fibonacci_slow(_input);
    
    _result
}

fn fast(_input: u64) -> u64 {
    let _result: u64 = fibonacci_fast(_input);

    _result
}

fn main() {
    println!("slow version => {:?} ", slow(8));
    println!("fast version => {:?} ", fast(8));
}
