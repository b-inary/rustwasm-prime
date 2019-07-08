use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{cast::ToPrimitive, Num, One, Pow, Zero};
use std::ops::Mul;

/// Evaluates the expression and returns an unsigned integer if success.
pub fn eval_expression(input: &[char]) -> Result<BigUint, String> {
  let val = ExprParser::parse(input)?;
  match val.to_biguint() {
    Some(num) => Ok(num),
    None => Err("Input is negative".into()),
  }
}

struct ExprParser<'a> {
  input: &'a [char],
  pos: usize,
}

impl<'a> ExprParser<'a> {
  fn parse(input: &[char]) -> Result<BigInt, String> {
    let mut parser = ExprParser { input, pos: 0 };
    let val = parser.parse_expr()?;
    if parser.pos < input.len() {
      return Err(format!("Unexpected character '{}'", input[parser.pos]));
    }
    Ok(val)
  }

  fn parse_expr(&mut self) -> Result<BigInt, String> {
    let mut val = self.parse_factor()?;
    while let Some(op @ '+') | Some(op @ '-') = self.input.get(self.pos) {
      self.pos += 1;
      let rhsval = self.parse_factor()?;
      match op {
        '+' => val += rhsval,
        '-' => val -= rhsval,
        _ => return Err("Something wrong".into()),
      }
    }
    Ok(val)
  }

  fn parse_factor(&mut self) -> Result<BigInt, String> {
    let mut val = self.parse_power()?;
    while let Some(op @ '*') | Some(op @ '/') | Some(op @ '%') = self.input.get(self.pos) {
      self.pos += 1;
      let rhsval = self.parse_power()?;
      match op {
        '*' => val *= rhsval,
        '/' => {
          if rhsval.is_zero() {
            return Err("Division by zero".into());
          }
          val /= rhsval;
        }
        '%' => {
          if rhsval.is_zero() {
            return Err("Modulo by zero".into());
          }
          val %= rhsval;
        }
        _ => return Err("Something wrong".into()),
      }
    }
    Ok(val)
  }

  fn parse_power(&mut self) -> Result<BigInt, String> {
    let mut val = self.parse_unary()?;
    if let Some(['*', '*']) = self.input.get(self.pos..self.pos + 2) {
      self.pos += 2;
      let rhsval = self.parse_power()?;
      if rhsval.sign() == Sign::Minus {
        return Err("Negative exponent".into());
      }
      if rhsval > BigInt::from(std::u32::MAX) {
        return Err("Exponent too big".into());
      }
      val = val.pow(rhsval.to_u32().unwrap());
    }
    Ok(val)
  }

  fn parse_unary(&mut self) -> Result<BigInt, String> {
    let mut neg = false;

    // Unary plus/minus
    match self.input.get(self.pos) {
      Some('+') => self.pos += 1,
      Some('-') => {
        neg = true;
        self.pos += 1;
      }
      _ => (),
    }

    let mut val = self.parse_paren()?;

    // Factorial
    if let Some('!') = self.input.get(self.pos) {
      if val.sign() == Sign::Minus {
        return Err("Negative factorial".into());
      }
      if val > BigInt::from(std::u32::MAX) {
        return Err("Factorial too big".into());
      }
      val = factorial(val.to_u32().unwrap());
      self.pos += 1;
    }

    // Negation
    if neg {
      val = -val;
    }

    Ok(val)
  }

  fn parse_paren(&mut self) -> Result<BigInt, String> {
    if let Some('(') = self.input.get(self.pos) {
      self.pos += 1;
      let val = self.parse_expr()?;
      return match self.input.get(self.pos) {
        Some(')') => {
          self.pos += 1;
          Ok(val)
        }
        Some(c) => Err(format!("Unexpected character '{}'", c)),
        None => Err("Unclosed parenthesis".into()),
      };
    }
    self.parse_num()
  }

  fn parse_num(&mut self) -> Result<BigInt, String> {
    if self.pos >= self.input.len() {
      return Err("Missing number".into());
    }

    let radix = match self.input.get(self.pos) {
      Some('0') => match self.input.get(self.pos + 1) {
        Some('b') => 2,
        Some('o') => 8,
        Some('x') => 16,
        _ => 10,
      },
      _ => 10,
    };
    if radix != 10 {
      self.pos += 2;
    }

    let mut endpos = self.pos;
    while endpos < self.input.len() && self.input[endpos].is_digit(radix) {
      endpos += 1;
    }
    if self.pos == endpos {
      return Err(format!("Unexpected character '{}'", self.input[self.pos]));
    }

    let valstr = self.input[self.pos..endpos].iter().collect::<String>();
    let val = BigInt::from_str_radix(&valstr, radix).unwrap();
    self.pos = endpos;
    Ok(val)
  }
}

fn factorial(n: u32) -> BigInt {
  (2..=n).fold(BigInt::one(), Mul::mul)
}

#[test]
fn evaluation_test() {
  fn assert_success(s: &str, result: u32) {
    assert_eq!(eval_expression(&s.chars().collect::<Vec<char>>()), Ok(BigUint::from(result)));
  }
  fn assert_failure(s: &str, msg: &str) {
    assert_eq!(eval_expression(&s.chars().collect::<Vec<char>>()), Err(msg.into()));
  }
  assert_success("0", 0);
  assert_success("0x1b", 27);
  assert_success("1+2*3", 7);
  assert_success("(1+2)*3", 9);
  assert_success("4**3**2", 262144);
  assert_success("-(-10!)", 3628800);
  assert_failure("", "Missing number");
  assert_failure("-42", "Input is negative");
  assert_failure("ほげ", "Unexpected character 'ほ'");
  assert_failure("(1+2*3", "Unclosed parenthesis");
  assert_failure("2**(-3)", "Negative exponent");
  assert_failure("(-10)!", "Negative factorial");
}
