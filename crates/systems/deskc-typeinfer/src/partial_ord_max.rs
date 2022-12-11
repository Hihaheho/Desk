use std::cmp::Ordering;

pub trait PartialOrdMax: Iterator {
    fn partial_max(self) -> Option<Self::Item>;
}

impl<T> PartialOrdMax for T
where
    T: Iterator,
    T::Item: PartialOrd + Clone,
{
    fn partial_max(mut self) -> Option<Self::Item> {
        let mut out = vec![];
        let mut max_candidate = self.next();
        while let Some(next) = self.next() {
            if let Some(max) = &max_candidate {
                match max.partial_cmp(&next) {
                    None => {
                        out.push((*max).clone());
                        out.push(next);
                        max_candidate = None
                    }
                    Some(Ordering::Less) => max_candidate = Some(next),
                    Some(Ordering::Equal) | Some(Ordering::Greater) => {}
                }
            } else {
                if out.iter().any(|out| match out.partial_cmp(&next) {
                    None => true,
                    Some(Ordering::Greater) => true,
                    _ => false,
                }) {
                    continue;
                }
                max_candidate = Some(next)
            }
        }
        return max_candidate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tuple(i32, i32);
    impl PartialOrd for Tuple {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.0 == other.0 && self.1 == other.1 {
                Some(Ordering::Equal)
            } else if self.0 <= other.0 && self.1 <= other.1 {
                Some(Ordering::Less)
            } else if self.0 >= other.0 && self.1 >= other.1 {
                Some(Ordering::Greater)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_empty() {
        let iter = Vec::<i32>::new().into_iter();
        assert_eq!(iter.partial_max(), None);
    }

    #[test]
    fn test_single() {
        let iter = vec![1].into_iter();
        assert_eq!(iter.partial_max(), Some(1));
    }

    #[test]
    fn test_ord() {
        let iter = vec![1, 2, 3, 3].into_iter();
        assert_eq!(iter.partial_max(), Some(3));
    }

    #[test]
    fn partial_ord_none_min_first() {
        let iter = vec![Tuple(1, 2), Tuple(2, 3), Tuple(3, 2)].into_iter();
        assert_eq!(iter.partial_max(), None);
    }

    #[test]
    fn partial_ord_none_compare_with_previous_candidate() {
        let iter = vec![Tuple(2, 3), Tuple(3, 2), Tuple(3, 2)].into_iter();
        assert_eq!(iter.partial_max(), None);
    }
    #[test]
    fn partial_ord_none_compare_with_last_next() {
        let iter = vec![Tuple(2, 3), Tuple(3, 2), Tuple(2, 3)].into_iter();
        assert_eq!(iter.partial_max(), None);
    }
    #[test]
    fn partial_ord_none_next_is_less() {
        let iter = vec![Tuple(2, 3), Tuple(3, 2), Tuple(1, 2)].into_iter();
        assert_eq!(iter.partial_max(), None);
    }

	#[test]
	fn partial_ord_some_next_is_greater() {
		let iter = vec![Tuple(2, 3), Tuple(3, 2), Tuple(4, 3)].into_iter();
		assert_eq!(iter.partial_max(), Some(Tuple(4, 3)));
	}
	#[test]
	fn partial_ord_none_next_is_equal() {
		let iter = vec![Tuple(2, 3), Tuple(3, 2), Tuple(3, 2)].into_iter();
		assert_eq!(iter.partial_max(), None);
	}

}
