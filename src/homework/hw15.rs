use itertools::Itertools;

struct Cryptarithm {
    digits: [i32; 8],
}

impl Cryptarithm {
    fn new(digits: [i32; 8]) -> Self {
        Cryptarithm { digits }
    }

    fn is_valid(&self) -> bool {
        let [m, _, _, a, s, _, _, _] = self.digits;
        m != 0 && s != 0
    }

    fn calculate(&self) -> (i32, i32) {
        let [m, u, x, a, s, l, o, n] = self.digits;
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;
        (muxa, slon)
    }

    fn is_solution(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        let (muxa, slon) = self.calculate();
        muxa * self.digits[3] == slon
    }

    fn format_solution(&self) {
        let [m, u, x, a, s, l, o, n] = self.digits;
        let (muxa, slon) = self.calculate();
        println!("  {:4}", muxa);
        println!("x   {}", a);
        println!("------");
        println!("  {:4}", slon);
        println!(
            "Letters: M:{}, U:{}, X:{}, A:{}, S:{}, L:{}, O:{}, N:{}",
            m, u, x, a, s, l, o, n
        );
        println!();
    }
}

fn find_solutions() -> Vec<Cryptarithm> {
    let mut results = Vec::new();
    for digits in (0..10).permutations(8).map(|v| {
        let mut arr = [0; 8];
        for (i, &val) in v.iter().enumerate() {
            arr[i] = val;
        }
        arr
    }) {
        let crypt = Cryptarithm::new(digits);
        if crypt.is_solution() {
            results.push(crypt);
        }
    }
    results
}

fn main() {
    let solutions = find_solutions();
    solutions.iter().for_each(|s| s.format_solution());
    println!("Total solutions: {}", solutions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_solutions() {
        let solutions = find_solutions();
        for crypt in &solutions {
            let [m, u, x, a, s, l, o, n] = crypt.digits;
            let (muxa, slon) = crypt.calculate();
            assert_eq!(muxa * a, slon, "Multiplication check failed");
            assert_eq!(
                crypt.digits.iter().collect::<std::collections::HashSet<_>>().len(),
                8,
                "Unique digits check failed"
            );
            assert_ne!(m, 0, "M cannot be zero");
            assert_ne!(s, 0, "S cannot be zero");
        }
    }
}
