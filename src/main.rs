use std::io;
use std::time::Instant;

const PRIMES_TO_PRINT: usize = 10;

fn main() {
    let stdin = io::stdin();
    let mut b = String::new();
    loop {
        println!("Enter Number (q to quit): ");
        stdin.read_line(&mut b).unwrap();
        if b.trim() == String::from("q") {
            break;
        }
        let n = b.trim().parse::<i32>();
        match n {
            Ok(i) => {
                let start = Instant::now();
                let maybe_primes = primes_to(i);
                let duration = start.elapsed();
                match maybe_primes {
                    Some(primes) => {
                        println!("");
                        println!(
                            "It took {:?} to find out that there are {} primes up to {}.",
                            duration,
                            primes.len(),
                            i
                        );
                        let num_to_print = min(primes.len(), PRIMES_TO_PRINT);
                        let printing_primes = primes
                            .iter()
                            .rev()
                            .skip(1)
                            .take(num_to_print - 1)
                            .rev()
                            .map(|n| (*n).to_string())
                            .collect::<Vec<String>>();
                        let primes_string = string_join(&printing_primes, ", ");
                        println!(
                            "The top {} primes are: {}, and {}",
                            num_to_print,
                            primes_string,
                            primes[primes.len() - 1]
                        );
                        println!("");
                    }
                    None => println!("No primes up to {}.", i),
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        b.clear();
    }
    println!("thanks for playing!");
}

fn string_join(strs: &Vec<String>, delim: &str) -> String {
    if strs.len() == 0 {
        return String::from("");
    }
    if strs.len() == 1 {
        return String::from(&strs[0]);
    }
    let mut joint = String::new();
    for s in 0..(strs.len() - 1) {
        joint += &strs[s];
        joint += delim;
    }
    joint += &strs[strs.len() - 1];
    joint
}

fn primes_to(n: i32) -> Option<Vec<i32>> {
    if n < 3 {
        return None;
    }
    let max = (n as f64).sqrt().ceil();
    let max = max as i32 + 1;
    let mut candidates: Vec<i32> = vec![1; (n as usize) + 1];
    let mut primes: Vec<i32> = Vec::new();
    for i in 2..(max as usize + 1) {
        if candidates[i] == 1 {
            filter_vec(&mut candidates, i);
        }
    }
    for i in 2..(candidates.len() as usize) {
        if candidates[i] == 1 {
            primes.push(i as i32);
        }
    }
    Some(primes)
}

fn filter_vec(cand: &mut Vec<i32>, index: usize) {
    for i in ((2 * index)..cand.len()).step_by(index) {
        cand[i] = 0;
    }
}

fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return b;
    } else {
        return a;
    }
}
