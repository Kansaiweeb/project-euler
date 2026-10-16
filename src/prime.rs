pub fn is_prime_naive(num: usize) -> bool {
    for factor in (2..=(num as f64).sqrt() as usize).rev() {
        if num % factor == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime(n: usize) -> bool {
    if n < 4 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    if n < 9 {
        return true;
    }

    if n % 3 == 0 {
        return false;
    }

    let r = n.isqrt();
    let mut f = 5;
    while f <= r {
        if n % f == 0 {
            return false;
        }

        if n % (f + 2) == 0 {
            return false;
        }

        f += 6;
    }
    true
}

#[test]
pub fn test_is_prime() {
    assert!(is_prime(11));
    assert!(is_prime(7));
    assert!(is_prime(13));
    assert!(is_prime(5));
}
