fn main() {

    println!("{:?}", solve().or(Some(0)));
}

fn solve() -> Option<usize> {
    let mut palindromes = Vec::new();
    for multiplier1 in (111..=999).rev() {
        for multiplier2 in (111..=999).rev() {
            let product = multiplier2 * multiplier1;
            if is_palindrome(product) {
                palindromes.push(product);
            }
        }
    }
    palindromes.iter().max().copied()
}

fn is_palindrome(n: usize) -> bool {
    let stringified_n = n.to_string();
    stringified_n == stringified_n.chars().rev().collect::<String>()
}
