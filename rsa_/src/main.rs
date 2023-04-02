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
    #[test]
    fn test_find_rsa() {}
}
