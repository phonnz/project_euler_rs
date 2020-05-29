// List natural numbers below 10, multiples of 3 or 5.
// => 3,5,6,9 SUM = 23
// Find multiples of 3 or 5 below 1000
fn find_multiples(range: u32, a: u32, b: u32) -> u32 {
    println!("Stargint compute for numbers below {}, and multiples of {} and  {}", range, a, b);
    let mut multiples = Vec::new();
    let mut result = 0;
    
    let mut n: u32 = 0;
    loop {
        if n >= range {
            break;
        }
        if n % a == 0 || n % b == 0 {
            multiples.push(n);
        }
        n+=1;
    }

    println!("{:?}", multiples);
    for i in multiples.iter() {
        result += i;
    }
    return result;
}

fn main() {
    println!("Result:{}", find_multiples(10, 3, 5));
    println!("Result:{}", find_multiples(1000, 3, 5));       
}
