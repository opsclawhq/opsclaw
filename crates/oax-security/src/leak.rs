use aho_corasick::AhoCorasick;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeakWarning {
    pub needle: String,
}

pub struct LeakDetector {
    ac: AhoCorasick,
    patterns: Vec<String>,
}

impl LeakDetector {
    pub fn new(patterns: &[&str]) -> Self {
        let ac = AhoCorasick::new(patterns).expect("valid leak patterns");
        Self {
            ac,
            patterns: patterns.iter().map(|p| (*p).to_string()).collect(),
        }
    }

    pub fn scan(&self, input: &str) -> Vec<LeakWarning> {
        self.ac
            .find_iter(input)
            .map(|m| LeakWarning {
                needle: self.patterns[m.pattern().as_usize()].clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::LeakDetector;

    #[test]
    fn detects_known_secret_patterns() {
        let detector = LeakDetector::new(&["ghp_", "AKIA"]);
        let leaks = detector.scan("token=ghp_12345 and key=AKIAZZZ");

        assert_eq!(leaks.len(), 2);
    }

    #[test]
    fn returns_empty_when_no_leaks() {
        let detector = LeakDetector::new(&["ghp_", "AKIA"]);
        let leaks = detector.scan("all clear");
        assert!(leaks.is_empty());
    }
}
