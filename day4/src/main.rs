use std::ops::Deref;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
struct SectionId(u64);

impl From<&str> for SectionId {
    fn from(from: &str) -> Self {
        let Ok(id) = from.parse::<u64>() else {
            panic!("Invalid number");
        };

        Self(id)
    }
}

impl Deref for SectionId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SectionRange {
    start: SectionId,
    end: SectionId,
}

impl SectionRange {
    fn is_fully_within(&self, other: SectionRange) -> bool {
        // Is my start gteq other, and is my end lteq other
        self.start >= other.start && self.end <= other.end
    }

    fn is_overlapping_at_all(&self, other: SectionRange) -> bool {
        // is my start somewhere within others range
        (self.start >= other.start && self.start <= other.end) | 
        // OR is my end somewhere within others range
        (self.end <= other.end && self.end >= other.start)
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Elf {
    assigned: SectionRange,
}
#[derive(Debug, Clone, PartialEq)]
struct ElfPair(Elf, Elf);

impl ElfPair {
    /// Does one of the elves assignment fully contain the others?
    fn is_any_containing_other(&self) -> bool {
        self.0.assigned.is_fully_within(self.1.assigned)
            | self.1.assigned.is_fully_within(self.0.assigned)
    }

    /// Is there any overlap at all in the assignments?
    fn is_overlap_with_other(&self) -> bool {
        self.0.assigned.is_overlapping_at_all(self.1.assigned)
            | self.1.assigned.is_overlapping_at_all(self.0.assigned)
    }
}
impl From<&str> for ElfPair {
    /// TODO: eeew
    fn from(from: &str) -> Self {
        let pairs = from.split(',');
        let mut sections = Vec::new();
        for pair in pairs {
            let range = pair.split('-');
            for r in range {
                sections.push(SectionId::from(r));
            }
        }

        ElfPair(
            Elf {
                assigned: SectionRange {
                    start: sections[0],
                    end: sections[1],
                },
            },
            Elf {
                assigned: SectionRange {
                    start: sections[2],
                    end: sections[3],
                },
            },
        )
    }
}
fn main() {
    let sum_fully_contains: usize = include_str!("../input.txt")
        .lines()
        .map(ElfPair::from)
        .filter(|pair| pair.is_any_containing_other())
        .count();

    println!(
        "Number of pairs fully containing the other: {}",
        sum_fully_contains
    );

    // Part #2

    let any_overlap: usize = include_str!("../input.txt")
        .lines()
        .map(ElfPair::from)
        .filter(|pair| pair.is_overlap_with_other())
        .count();
    println!(
        "Number of pairs that has any overlap at all: {}",
        any_overlap
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let case1 = ElfPair(
            Elf {
                assigned: SectionRange {
                    start: SectionId(2),
                    end: SectionId(4),
                },
            },
            Elf {
                assigned: SectionRange {
                    start: SectionId(6),
                    end: SectionId(8),
                },
            },
        );

        let case2 = ElfPair::from("2-4,6-8");
        assert_eq!(case1, case2);
    }

    #[test]
    fn comparing() {
        let case1 = ElfPair::from("2-4,6-8");
        assert!(!case1.is_any_containing_other());
        assert!(!case1.is_overlap_with_other());
        let case2 = ElfPair::from("2-3,4-5");
        assert!(!case2.is_any_containing_other());
        assert!(!case2.is_overlap_with_other());
        let case3 = ElfPair::from("5-7,7-9");
        assert!(!case3.is_any_containing_other());
        assert!(case3.is_overlap_with_other());
        let case4 = ElfPair::from("2-8,3-7");
        assert!(case4.is_any_containing_other());
        assert!(case4.is_overlap_with_other());
        let case5 = ElfPair::from("6-6,4-6");
        assert!(case5.is_any_containing_other());
        assert!(case5.is_overlap_with_other());
        let case6 = ElfPair::from("2-6,4-8");
        assert!(!case6.is_any_containing_other());
        assert!(case6.is_overlap_with_other());

        let sim = vec![
            case1.clone(),
            case2.clone(),
            case3.clone(),
            case4.clone(),
            case5.clone(),
            case6.clone(),
        ]
        .iter()
        .filter(|pair| pair.is_any_containing_other())
        .count();
        assert_eq!(2, sim);

        let sim2 = vec![case1, case2, case3, case4, case5, case6]
            .iter()
            .filter(|pair| pair.is_overlap_with_other())
            .count();
        assert_eq!(4, sim2);
    }

    #[test]
    fn misc() {
        let id1 = SectionId::from("42");
        let id2 = SectionId(42);

        assert_eq!(id1, id2);

        let range = SectionRange {
            start: SectionId(1),
            end: SectionId(4),
        };

        let elf1 = Elf { assigned: range };
        let elf2 = Elf { assigned: range };
        let pair = ElfPair(elf1, elf2);

        assert_eq!(1u64, *pair.0.assigned.start);
        assert_eq!(4u64, *pair.0.assigned.end);
    }
}

