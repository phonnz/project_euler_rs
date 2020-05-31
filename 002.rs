// Fibonacci
// By considering the terms in the Fibonacci sequence whose values do not exceed four million
// Find the sum of the even-valued terms.
fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn main(){
    let mut terms = Vec::new();
    let mut current_term = 0;
    let max_value = 4000000;

    loop {
        let val = fib(current_term);
        if val < max_value && val % 2 == 0  {
            terms.push(val);
        }else if val > max_value {
            break;
        }
        current_term += 1;
    }
    println!("{:?}", terms);
    let res: u64 = terms.iter().fold(0, |acc, &x| acc + x);
    println!("{:?}", res)
}