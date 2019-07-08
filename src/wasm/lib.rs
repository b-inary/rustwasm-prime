use num_bigint::BigUint;
use num_traits::{One, Zero};
use wasm_bindgen::prelude::*;

mod evaluation;
mod lucas;
mod miller_rabin;

/// Parses and evaluates the expression and returns an unsigned integer if success.
#[wasm_bindgen]
pub fn parse_integer(input: &str) -> Option<Box<[u8]>> {
  match evaluation::eval_expression(&input.chars().collect::<Vec<char>>()) {
    Ok(num) => Some(num.to_bytes_le().into()),
    Err(_) => None,
  }
}

#[wasm_bindgen]
pub fn parse_error_message(input: &str) -> Option<String> {
  evaluation::eval_expression(&input.chars().collect::<Vec<char>>()).err()
}

/// Performs trial division and square check.
#[wasm_bindgen]
pub fn trivial_prime_test(bytes: &[u8]) -> bool {
  let num = BigUint::from_bytes_le(bytes);

  // Exclude 0 and 1
  if num.is_zero() || num.is_one() {
    return false;
  }

  // Perform trial division
  let primes = sieve(1000);
  if !trial_division(&num, &primes) {
    return false;
  }

  // Confirm the number is not a square
  let sqrt = num.sqrt();
  &sqrt * &sqrt != num
}

/// Performs a Miller-Rabin test.
#[wasm_bindgen]
pub fn miller_rabin_test(bytes: &[u8]) -> bool {
  let num = BigUint::from_bytes_le(bytes);
  miller_rabin::prime_test(&num)
}

/// Performs an extra strong Lucas test.
#[wasm_bindgen]
pub fn extra_strong_lucas_test(bytes: &[u8]) -> bool {
  let num = BigUint::from_bytes_le(bytes);
  lucas::prime_test(&num)
}

/// Returns a list of primes less than `n` by sieve of Eratosthenes.
fn sieve(n: u32) -> Vec<u32> {
  let n = n as usize;
  assert!(n >= 2);

  let mut is_prime = vec![true; n];
  is_prime[0] = false;
  is_prime[1] = false;

  for i in 2..n {
    if is_prime[i] {
      for j in ((i * i)..n).step_by(i) {
        is_prime[j] = false;
      }
    }
  }

  is_prime.iter().enumerate().filter_map(|(i, v)| if *v { Some(i as u32) } else { None }).collect()
}

/// Performs trial division.
fn trial_division(num: &BigUint, primes: &[u32]) -> bool {
  for p in primes {
    if (num % p).is_zero() {
      return num == &(*p).into();
    }
  }
  true
}

#[test]
fn sieve_test() {
  assert_eq!(sieve(50), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]);
}
