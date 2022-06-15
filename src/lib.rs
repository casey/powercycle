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
      if y <= self.n {
        return y - 1;
      }
    }
  }

  pub fn new(n: u64) -> Self {
    let mut p = n + 1;

    while !(is_prime((p - 1) / 2) && is_prime(p)) {
      p += 1;
    }

    for g in 2..p {
      if mod_exp(g as u128, 2, p as u128) != 1
        && mod_exp(g as u128, (p as u128 - 1) / 2, p as u128) != 1
      {
        return Self { n, p, g };
      }
    }

    panic!("Unable to find cycle!");
  }
}

#[cfg(test)]
mod tests {
  use {super::*, std::collections::HashSet};

  #[test]
  fn small_cycles() {
    for i in 0..1000 {
      let cycle = Cycle::new(i);

      let mut seen = HashSet::new();

      for n in 0..i {
        let x = cycle.apply(n);

        if seen.contains(&x) {
          panic!();
        }

        seen.insert(x);
      }

      assert_eq!(seen.len(), i as usize);
    }
  }

  #[test]
  fn large_cycle() {
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
