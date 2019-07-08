use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;

/// Performs a Miller-Rabin test with base 2.
pub fn prime_test(num: &BigUint) -> bool {
  assert!(*num > BigUint::one());
  assert!(num.is_odd());

  let num_minus_one = num - 1u32;

  // `num == 2**s * t + 1` where `t` is odd
  let mut s = 0;
  let mut t = num_minus_one.clone();
  while t.is_even() {
    s += 1;
    t >>= 1;
  }

  // Select base = 2
  let mut x = BigUint::from(2u32).modpow(&t, num);

  if x.is_one() || x == num_minus_one {
    return true;
  }

  for _ in 1..s {
    x = &x * &x % num;
    if x == num_minus_one {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn miller_rabin_test() {
    let max = 50000;
    let odd_primes: HashSet<u32> = crate::sieve(max).into_iter().skip(1).collect();
    let miller_rabin_primes: HashSet<u32> =
      (3..max).step_by(2).filter(|n| prime_test(&(*n).into())).collect();

    assert!(odd_primes.is_subset(&miller_rabin_primes));

    // See: OEIS A001262
    assert_eq!(
      &miller_rabin_primes - &odd_primes,
      vec![2047, 3277, 4033, 4681, 8321, 15841, 29341, 42799, 49141].into_iter().collect()
    );
  }
}
