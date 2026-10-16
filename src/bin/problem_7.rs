use euler_solutions::prime::is_prime;

fn main() {
    println!("{:?}", solve(6));
    println!("{:?}", solve(10001));
}

fn solve(nth: usize) -> usize {
    let mut n = 1;
    let mut current_num = 3;
    loop {
        if is_prime(current_num) {
            n += 1;
        }

        if n == nth {
            return current_num;
        }
        current_num += 1;
    }
}

