use {mod_exp::mod_exp, primes::is_prime};

#[derive(Debug, PartialEq)]
pub struct Cycle {
  n: u64,
  p: u64,
  g: u64,
}

impl Cycle {
  pub fn apply(&self, x: u64) -> u64 {
    let mut y = x + 1;

    loop {
      y *= self.g;
      y %= self.p;
      if y - 1 < self.n {
        return y - 1;
      }
    }
  }

  pub fn new(n: u64) -> Self {
    assert_ne!(n, 0, "Cycle size may not be 0");

    let mut p = n + 1;

    while !(is_prime(p / 2) && is_prime(p)) {
      p += 1;
    }

    for g in 1..p {
      if mod_exp(g as u128, p as u128 / 2, p as u128) != 1 {
        return Self { n, p, g };
      }
    }

    unreachable!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn small_cycles() {
    for n in 1..10000 {
      let cycle = Cycle::new(n);

      let mut i = 0;
      let mut x = 0;
      loop {
        x = cycle.apply(x);

        assert!(x < n);

        if x == 0 {
          assert_eq!(i + 1, n);
          break;
        }

        i += 1;
      }
    }
  }

  #[test]
  fn ord_cycle() {
    assert_eq!(
      Cycle::new(2099999997690000),
      Cycle {
        n: 2099999997690000,
        p: 2099999997690767,
        g: 5
      }
    );
  }
}
