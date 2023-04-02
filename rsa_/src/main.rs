use prime_factorization::Factorization;
use std::{env, fs::read_to_string};

fn main() {
    let f = env::args().nth(1).unwrap();
    let prime = read_to_string(f)
        .unwrap()
        .trim()
        .to_string()
        .parse::<u128>()
        .unwrap();
    let factor_repr = Factorization::run(prime);
    println!(
        "{}={}*{}",
        prime, factor_repr.factors[1], factor_repr.factors[0]
    );
}

#[cfg(test)]
mod tests {
    use prime_factorization::Factorization;

    #[test]
    fn test_find_rsa() {
        let num: u128 = 3_746_238_285_234_848_709_827;
        let factor_repr = Factorization::run(num);
        assert_eq!(factor_repr.factors, vec![103_979, 36_028_797_018_963_913]);
        let num: u128 = 2497885147362973;
        let factor_repr = Factorization::run(num);
        assert_eq!(factor_repr.factors, vec![49978553, 49979141]);
        let num: u128 = 239821585064027;
        let factor_repr = Factorization::run(num);
        assert_eq!(factor_repr.factors, vec![15485867, 15486481]);
    }
}
