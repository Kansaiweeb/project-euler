use euler_solutions::prime::is_prime;

fn main() {
    println!("{}", solve(10));
    println!("{}", solve(2_000_000));
}

fn solve(max_num: usize) -> usize {
    let mut sum = 0;
    for n in 2..max_num {
        if is_prime(n) {
            sum += n;
        }
    }
    sum
}
