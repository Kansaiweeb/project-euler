fn main() {
    println!("{}", solve(100));
}

fn solve(n: usize) -> usize {
    let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<usize>();
    let square_of_sum = (1..=n).sum::<usize>().pow(2);
    square_of_sum - sum_of_squares
}
