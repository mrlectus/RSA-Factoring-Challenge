use std::{fs::read_to_string, env};
pub fn find_factors(num: i128) -> (i128, i128) {
    let end = num as f64;
    let end = end.sqrt();
    for i in 2..end.floor() as i128 + 1 {
        if num % i == 0 {
            return ((num / i), i)
        }
    }
    (-1, -1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let f = read_to_string(file_path).expect("cannot find file");
    for x in f.lines() {
        let (v1, v2) = find_factors(x.parse().unwrap());
        println!("{x}={v1}*{v2}");
    }
}

#[cfg(test)]
mod test {
    use crate::find_factors;

    #[test]
    fn test_find_factors() {
        assert_eq!(find_factors(239809320265259), (15485783,15485773));
    }
}
