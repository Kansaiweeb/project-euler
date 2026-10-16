fn main() {
    println!("{}", solve(1000));
}

fn solve(sum: usize) -> usize {
    for a in 1..=sum - 2 {
        for b in 1..=sum - 2 {
            let c_candidate = a.pow(2) + b.pow(2);
            if let Some(c) = is_perfect_square(c_candidate) {
                if a + b + c == sum {
                    return a * b * c;
                }
            }
        }
    }

    panic!("No triplet found");
}

fn is_perfect_square(num: usize) -> Option<usize> {
    let root = (num as f64).sqrt() as usize;
    if root * root == num {
        return Some(root);
    }
    None
}
