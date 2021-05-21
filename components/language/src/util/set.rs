#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Set<T: Eq>(Vec<T>);

impl<T: Ord + PartialEq> Set<T> {
    pub fn new(mut vec: Vec<T>) -> Self {
        vec.sort_unstable();
        vec.dedup();
        Self(vec)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if let Some(first) = other.0.first() {
            other
                .0
                .iter()
                .zip(self.0.iter().skip_while(|&element| element != first))
                .all(|(one, other)| one == other)
        } else {
            // It's empty set.
            true
        }
    }

    pub fn contains(&self, one: &T) -> bool {
        self.0.contains(one)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equals() {
        assert_eq!(Set::new(vec![1]), Set::new(vec![1]));
    }

    #[test]
    fn not_equals() {
        assert_ne!(Set::new(vec![1]), Set::new(vec![0]));
    }

    #[test]
    fn equals_in_any_order() {
        assert_eq!(Set::new(vec![1, 2]), Set::new(vec![2, 1]));
    }

    #[test]
    fn equals_with_duplicates() {
        assert_eq!(Set::new(vec![1, 2, 1]), Set::new(vec![2, 1]));
    }

    #[test]
    fn is_subset_true() {
        assert!(Set::new(vec![2, 1, 3]).is_subset(&Set::new(vec![3, 2])));
    }

    #[test]
    fn is_subset_false() {
        assert!(!Set::new(vec![2, 1, 3]).is_subset(&Set::new(vec![4, 2])));
    }

    #[test]
    fn is_subset_empty_set_is_always_subset() {
        assert!(Set::new(vec![1]).is_subset(&Set::new(vec![])));
    }

    #[test]
    fn contains_returns_true_if_contains() {
        assert!(Set::new(vec![1]).contains(&1));
    }

    #[test]
    fn contains_returns_false_if_not_contains() {
        assert!(!Set::new(vec![1]).contains(&2));
    }
}
