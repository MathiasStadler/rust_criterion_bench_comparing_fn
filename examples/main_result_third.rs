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



fn main() {
    
    
    println!("slow version => {:?} ", slow(8));
   
}
