#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Set<T: Eq>(Vec<T>);

impl<T: Ord> Set<T> {
    pub fn new(mut vec: Vec<T>) -> Self {
        // Assumes that equal elements are pragmatically equal.
        vec.sort_unstable();
        vec.dedup();
        Self(vec)
    }

    pub fn is_superset_of(&self, other: &Self, eq: impl Fn(&T, &T) -> bool) -> bool {
        let mut self_iter = self.0.iter();
        for other_item in other.0.iter() {
            if self_iter
                .find(|self_item| eq(self_item, other_item))
                .is_none()
            {
                return false;
            }
        }
        return true;
    }

    pub fn contains(&self, one: &T, eq: impl Fn(&T, &T) -> bool) -> bool {
        self.iter().any(|item| eq(item, one))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
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
    fn is_superset_of_true() {
        assert!(Set::new(vec![2, 1, 3, 4]).is_superset_of(&Set::new(vec![4, 2]), |x, y| x == y));
    }

    #[test]
    fn is_superset_of_true_with_other_equiality_function() {
        assert!(Set::new(vec![4, 2, 2]).is_superset_of(&Set::new(vec![3, 3]), |x, y| x < y));
    }

    #[test]
    fn is_superset_of_false() {
        assert!(!Set::new(vec![2, 1, 3]).is_superset_of(&Set::new(vec![4, 2]), |x, y| x == y));
    }

    #[test]
    fn is_superset_of_empty_set_is_always_superset_of() {
        assert!(Set::new(vec![1]).is_superset_of(&Set::new(vec![]), |x, y| x == y));
    }

    #[test]
    fn is_superset_of_false_self_is_smaller_set() {
        assert!(!Set::new(vec![1]).is_superset_of(&Set::new(vec![1, 2]), |x, y| x == y));
    }

    #[test]
    fn contains_returns_true_if_contains() {
        assert!(Set::new(vec![1]).contains(&1, |x, y| x == y));
    }

    #[test]
    fn contains_returns_true_if_contains_with_other_equity_functions() {
        assert!(Set::new(vec![1]).contains(&2, |x, y| x < y));
    }

    #[test]
    fn contains_returns_false_if_not_contains() {
        assert!(!Set::new(vec![1]).contains(&2, |x, y| x == y));
    }

    #[test]
    fn size() {
        assert_eq!(Set::new(Vec::<i32>::new()).len(), 0);
        assert_eq!(Set::new(vec![true]).len(), 1);
        assert_eq!(Set::new(vec![0, 1, 2, 2]).len(), 3);
    }

    #[test]
    fn iter() {
        let set = Set::new(vec![0, 2, 1]);
        let mut iter = set.iter().cloned();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
}
