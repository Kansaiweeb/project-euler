use euler_solutions::prime::is_prime;

fn main() {
    println!("{}", solve(13195));
    print!("{}", solve(600851475143));
    
}


fn solve(target: usize) -> usize {
    let max_factor = (target as f64).sqrt() as usize;

    for factor in (2..=max_factor).rev() {
        if target % factor == 0 && is_prime(factor){
            return factor;
        }
    }

    target
}