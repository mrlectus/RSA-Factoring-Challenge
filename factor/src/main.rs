use gcd::Gcd;
use rand::Rng;
use std::{env, fs::read_to_string};

pub fn quad_sieve(n: u128) -> (u128, u128) {
    let b_smoothness = 25;
    let m = (b_smoothness as f64).sqrt().ceil() as usize;

    let mut rng = rand::thread_rng();

    let mut sieve = vec![0u8; 2 * m];
    let mut factors = Vec::new();

    let mut x = rng.gen_range(1..n);
    let mut y = x;

    let mut i = 1;

    while factors.len() < b_smoothness {
        x = x.pow(2) % n;

        let diff = if x > y { x - y } else { y - x };
        let gcd = diff.gcd(n);

        if gcd != 1 && gcd != n {
            factors.push(gcd);
            if factors.len() == b_smoothness {
                break;
            }
        }

        if i == m {
            for j in 0..m {
                sieve[j] = 0;
                sieve[m + j] = 0;
                let k = ((x + j as u128) as f64).sqrt().ceil() as u128;
                sieve[j] = ((k - x - (j as u128).pow(2)) % 2) as u8;
                sieve[m + j] = ((k - y - (j as u128).pow(2)) % 2) as u8;
            }

            let mut smooth = false;
            let mut bitvec = vec![false; b_smoothness];

            'outer: for a in 0..m {
                let mut prod = 1;

                for b in 0..b_smoothness {
                    if sieve[a + b * m] == 1 {
                        bitvec[b] = true;
                        prod *= factors[b];
                        if prod > n {
                            smooth = false;
                            break 'outer;
                        }
                    }
                }

                if prod != 1 && (prod as f64).sqrt().powi(2) as usize == prod as usize {
                    let sqrt_prod = (prod as f64).sqrt() as usize;
                    for b in 0..b_smoothness {
                        if bitvec[b] && factors[b].gcd(sqrt_prod as u128) != 1 {
                            smooth = false;
                            break 'outer;
                        }
                    }

                    let mut p = x + a as u128;
                    let mut q = y + sqrt_prod as u128;

                    if p > q {
                        std::mem::swap(&mut p, &mut q);
                    }

                    let gcd = p.gcd(q);

                    if gcd != 1 && gcd != n {
                        return (gcd, n / gcd);
                    }

                    smooth = true;
                    break 'outer;
                }
            }

            if !smooth {
                y = x;
            }

            i = 1;
        } else {
            i += 1;
        }
    }

    (0, 0)
}

pub fn find_factors_v2(value: u128) -> (u128, u128) {
    if value % 2 == 0 {
        return (value / 2, 2);
    } else {
        let mut i = 3;
        while i * i <= value {
            if value % i == 0 {
                return ((value / i), i);
            }
            i += 2;
        }
    }
    (0, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let f = read_to_string(file_path).expect("cannot find file");
    for x in f.lines() {}
}

#[cfg(test)]
mod test {
    use crate::quad_sieve;

    #[test]
    fn test_find_factors() {
        assert_eq!(quad_sieve(10), (5, 2));
    }
}
