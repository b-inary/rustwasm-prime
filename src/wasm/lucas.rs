use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};

/// Performs an extra strong Lucas test.
#[allow(clippy::many_single_char_names)]
pub fn prime_test(num: &BigUint) -> bool {
  assert!(*num > BigUint::one());
  assert!(num.is_odd());

  let mut p = 3u32;
  let mut d = 5u32;
  while jacobi(&d.into(), num) != -1 {
    p += 1;
    d = p * p - 4;
  }

  // `num + 1 == 2**s * t` where `t` is odd
  let mut s = 0;
  let mut t = num + 1u32;
  while t.is_even() {
    s += 1;
    t >>= 1;
  }

  // Compose Lucas sequence
  let mut bits = t.bits();
  let mut u = BigUint::one();
  let mut v = BigUint::from(p);
  while bits > 1 {
    bits -= 1;
    u = &u * &v % num;
    v = (&v * &v + num - 2u32) % num;
    if (&t >> (bits - 1)).is_odd() {
      let (uu, vv) = (u, v);
      u = &uu * p + &vv;
      v = &uu * d + &vv * p;
      if u.is_odd() {
        u += num;
      }
      if v.is_odd() {
        v += num;
      }
      u >>= 1;
      v >>= 1;
    }
  }

  u %= num;
  v %= num;

  if u.is_zero() && (v == BigUint::from(2u32) || v == num - 2u32) {
    return true;
  }

  if v.is_zero() {
    return true;
  }

  for _ in 1..s {
    v = (&v * &v + num - 2u32) % num;
    if v.is_zero() {
      return true;
    }
  }

  false
}

/// Computes the Jacobi symbol (m/n). Assumes `n` be an odd integer.
fn jacobi(m: &BigUint, n: &BigUint) -> i32 {
  let mut m = m.clone();
  let mut n = n.clone();

  if m > n {
    m %= &n;
  }

  let three = BigUint::from(3u32);
  let five = BigUint::from(5u32);
  let seven = BigUint::from(7u32);

  let mut res = 1;
  while !m.is_zero() {
    while m.is_even() {
      m >>= 1;
      let r = &n & &seven;
      if r == three || r == five {
        res = -res;
      }
    }
    std::mem::swap(&mut m, &mut n);
    if &m & &three == three && &n & &three == three {
      res = -res;
    }
    m %= &n;
  }

  res
}

#[cfg(test)]
mod tests {
  use super::*;
  use num_integer::Roots;
  use num_traits::pow;
  use std::collections::HashSet;

  #[test]
  fn jacobi_test() {
    fn jacobi_u32(m: u32, n: u32) -> i32 {
      jacobi(&m.into(), &n.into())
    }
    assert_eq!(jacobi_u32(45, 77), -1);
    assert_eq!(jacobi_u32(60, 121), 1);
    assert_eq!(jacobi_u32(365, 1847), 1);
    assert_eq!(jacobi_u32(1001, 9907), -1);
    assert_eq!(jacobi_u32(1236, 20003), 1);
  }

  #[test]
  fn lucas_test() {
    let max = 50000;
    let odd_primes: HashSet<u32> = crate::sieve(max).into_iter().skip(1).collect();
    let lucas_primes: HashSet<u32> =
      (3..max).step_by(2).filter(|n| !is_square(*n) && prime_test(&(*n).into())).collect();

    assert!(odd_primes.is_subset(&lucas_primes));

    // See: OEIS A217719
    assert_eq!(
      &lucas_primes - &odd_primes,
      vec![989, 3239, 5777, 10877, 27971, 29681, 30739, 31631, 39059].into_iter().collect()
    );
  }

  fn is_square(n: u32) -> bool {
    pow(n.sqrt(), 2) == n
  }
}
