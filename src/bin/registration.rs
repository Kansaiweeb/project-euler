fn main() {
    println!("{:?}", solve(5));
    println!("{:?}", solve(955_000));
}


fn solve(num_limit: usize) -> usize {
    (1..=num_limit).filter(|&x| x % 2 == 1).map(|x| x * x).sum()
}