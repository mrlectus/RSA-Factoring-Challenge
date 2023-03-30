use std::{env, fs::read_to_string, str::FromStr};

use rug::Integer;

pub fn find_factors(value: &str) -> (Integer, Integer) {
    let int = Integer::from_str(&value).unwrap();
    if int.is_even() {
        return (int / 2, Integer::from(2));
    } else {
        let mut i = Integer::from(3);
        while i <= int {
            if &int % Integer::from(&i) == 0 {
                return (int / Integer::from(&i), Integer::from(&i));
            }
            i += Integer::from(2);
        }
    }
    (Integer::from(0), Integer::from(0))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let f = read_to_string(file_path).expect("cannot find file");
    for x in f.lines() {
        let (v1, v2) = find_factors(x);
        println!("{x}={v1}*{v2}");
    }
}

#[cfg(test)]
mod test {
    use crate::find_factors;
    use rug::Integer;
    #[test]
    fn test_find_factors() {
        assert_eq!(find_factors("4"), (Integer::from(2), Integer::from(2)));
        assert_eq!(find_factors("9"), (Integer::from(3), Integer::from(3)));
    }
}
