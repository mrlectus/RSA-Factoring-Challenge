use std::{env, fs::read_to_string, str::FromStr};
use num_bigint::{BigUint, ToBigUint};


pub fn find_factors(value: &str) -> (BigUint, BigUint) {
    let mut left = 2.to_biguint().unwrap();
    let right = &BigUint::from_str(&value).unwrap();
    let right_end = &BigUint::from_str(&value).unwrap().sqrt();
    while left <= *right_end {
        if right % &left == BigUint::new(vec![0]) {
            return (right / &left, left);
        }
        left = left + 1.to_biguint().unwrap();
    }
    (BigUint::new(vec![0]), BigUint::new(vec![0]))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let f = read_to_string(file_path).expect("cannot find file");
    for x in f.lines() {
        let (v1, v2) = find_factors(&x);
        println!("{x}={v1}*{v2}");
    }
}

#[cfg(test)]
mod test {
    use num_bigint::ToBigUint;

    use crate::find_factors;

    #[test]
    fn test_find_factors() {
        assert_eq!(find_factors("9"), (3.to_biguint().unwrap(), 3.to_biguint().unwrap()));
    }
}
