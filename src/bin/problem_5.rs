fn main() {
    println!("{:?}", solve());
}

fn solve() -> usize {
    (1..=usize::MAX).filter(|x| divisable_by_all_from_one_to_twenty(*x)).find(|_| true).unwrap()
}

fn divisable_by_all_from_one_to_twenty(num: usize) -> bool {
    (1..=20).all(|x| num % x == 0)
}
