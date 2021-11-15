struct Naive {
    text: Vec<u8>,
    pattern: Vec<u8>,
    current: usize,
}

fn naive(text: String, pattern: String) -> Naive {
    Naive { text: text.into_bytes(), pattern: pattern.into_bytes(), current: 0 }
}

impl Iterator for Naive {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let m = self.pattern.len();
        let n = self.text.len();

        if m > n { return None; }
        if m == 0 { return None; }

        for j in self.current..(n - m + 1) {
            let mut i = 0;

            while i < m && self.text[j + i] == self.pattern[i] {
                i += 1;
            }

            if i == m {
                self.current = j + 1;
                return Some(j);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::naive::naive;

    #[test]
    fn finds_matches() {
        let text = "abab".to_string();
        let pattern = "ab".to_string();
        let mut matches = naive(text, pattern);
        assert_eq!(matches.next(), Some(0));
        assert_eq!(matches.next(), Some(2));
        assert_eq!(matches.next(), None);
    }
}
