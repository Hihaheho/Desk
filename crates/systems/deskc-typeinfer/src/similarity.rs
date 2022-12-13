use std::cmp::Ordering;

use crate::ctx::Ctx;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Similarity {
    Same,
    LabelMatch,
    LabelMismatch,
    BrandMatch,
    LabelToInner,
    InnerToLabel,
    BrandToInner,
    Number,
    Product,
    ProductToInner,
    Sum,
    InnerToSum,
    Map,
    Vector,
    Function,
    Infer,
    Instantiate,
}

impl Similarity {
    pub fn strength(&self) -> u8 {
        match self {
            Similarity::Same => 100,
            Similarity::LabelMatch | Similarity::BrandMatch => 10,
            Similarity::Infer | Similarity::Instantiate => 1,
            _ => 5,
        }
    }
}

impl PartialOrd for Similarity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}

impl Ord for Similarity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Similarities(Vec<Similarity>);

impl PartialOrd for Similarities {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut iter = self.0.iter();
        let mut other = other.0.iter();
        loop {
            match (iter.next(), other.next()) {
                (None, None) => break,
                (None, Some(_)) => return Some(Ordering::Less),
                (Some(_), None) => return Some(Ordering::Greater),
                (Some(a), Some(b)) => {
                    let cmp = a.cmp(b);
                    if cmp != Ordering::Equal {
                        return Some(cmp);
                    }
                }
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Similarities {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<Vec<Similarity>> for Similarities {
    fn from(similarities: Vec<Similarity>) -> Self {
        Self(similarities)
    }
}

impl Similarities {
    pub fn insert(&mut self, similarity: Similarity) {
        self.0.insert(0, similarity);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SimilaritiesList(pub Vec<Similarities>);

impl SimilaritiesList {
    pub fn push(&mut self, similarities: Similarities) {
        self.0.push(similarities);
    }
    pub fn max(&self) -> Similarities {
        self.0.iter().max().unwrap().clone()
    }
}

impl PartialOrd for SimilaritiesList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.iter().zip(other.0.iter()).any(|(l, r)| l < r) {
            if self.0.iter().zip(other.0.iter()).any(|(l, r)| l > r) {
                None
            } else {
                Some(Ordering::Less)
            }
        } else if self.0.iter().zip(other.0.iter()).any(|(l, r)| l > r) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WithSimilarities<T> {
    pub ctx: T,
    pub similarities: Similarities,
}

impl<T> WithSimilarities<T> {
    pub fn ctx_do<R, E>(
        self,
        func: impl FnOnce(T) -> Result<R, E>,
    ) -> Result<WithSimilarities<R>, E> {
        Ok(WithSimilarities {
            ctx: func(self.ctx)?,
            similarities: self.similarities,
        })
    }
    pub fn insert_similarity(mut self, similarity: Similarity) -> Self {
        self.similarities.insert(similarity);
        self
    }
}

impl Ctx {
    pub fn with_similarities(self, similarities: Similarities) -> WithSimilarities<Self> {
        WithSimilarities {
            ctx: self,
            similarities,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WithSimilaritiesList<T> {
    pub ctx: T,
    pub list: SimilaritiesList,
}

impl<T: PartialEq> PartialOrd for WithSimilaritiesList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.list.partial_cmp(&other.list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_strength() {
        assert_eq!(Similarity::Same.strength(), 100);
        assert_eq!(Similarity::LabelMatch.strength(), 10);
        assert_eq!(Similarity::BrandMatch.strength(), 10);
        assert_eq!(Similarity::LabelMismatch.strength(), 5);
        assert_eq!(Similarity::LabelToInner.strength(), 5);
        assert_eq!(Similarity::InnerToLabel.strength(), 5);
        assert_eq!(Similarity::Number.strength(), 5);
        assert_eq!(Similarity::Product.strength(), 5);
        assert_eq!(Similarity::ProductToInner.strength(), 5);
        assert_eq!(Similarity::Sum.strength(), 5);
        assert_eq!(Similarity::InnerToSum.strength(), 5);
        assert_eq!(Similarity::Map.strength(), 5);
        assert_eq!(Similarity::Vector.strength(), 5);
        assert_eq!(Similarity::Function.strength(), 5);
        assert_eq!(Similarity::Infer.strength(), 1);
        assert_eq!(Similarity::Instantiate.strength(), 1);
    }

    #[test]
    fn test_similarity_sort() {
        let mut vec = vec![
            Similarity::Infer,
            Similarity::Number,
            Similarity::Product,
            Similarity::ProductToInner,
            Similarity::LabelMatch,
            Similarity::Sum,
            Similarity::InnerToSum,
            Similarity::Same,
        ];
        vec.sort();
        vec.reverse();

        assert_eq!(
            vec,
            vec![
                Similarity::Same,
                Similarity::LabelMatch,
                Similarity::InnerToSum,
                Similarity::Sum,
                Similarity::ProductToInner,
                Similarity::Product,
                Similarity::Number,
                Similarity::Infer,
            ]
        );
    }

    #[test]
    fn test_similarities_sort() {
        let mut vec = vec![
            Similarities(vec![Similarity::Infer]),
            Similarities(vec![Similarity::Number]),
            Similarities(vec![Similarity::Product]),
            Similarities(vec![Similarity::ProductToInner]),
            Similarities(vec![Similarity::LabelMatch]),
            Similarities(vec![Similarity::Sum]),
            Similarities(vec![Similarity::InnerToSum]),
            Similarities(vec![Similarity::Same]),
        ];

        vec.sort();
        vec.reverse();

        assert_eq!(
            vec,
            vec![
                Similarities(vec![Similarity::Same]),
                Similarities(vec![Similarity::LabelMatch]),
                Similarities(vec![Similarity::InnerToSum]),
                Similarities(vec![Similarity::Sum]),
                Similarities(vec![Similarity::ProductToInner]),
                Similarities(vec![Similarity::Product]),
                Similarities(vec![Similarity::Number]),
                Similarities(vec![Similarity::Infer]),
            ]
        );
    }

    #[test]
    fn test_similarities_cmp() {
        let left = Similarities(vec![
            Similarity::InnerToSum,
            Similarity::LabelMatch,
            Similarity::Same,
        ]);
        let right = Similarities(vec![
            Similarity::ProductToInner,
            Similarity::LabelMatch,
            Similarity::Infer,
        ]);
        assert!(left > right);
        assert!(right < left);
    }

    #[test]
    fn test_similarities_list_cmp() {
        let left = SimilaritiesList(vec![
            Similarities(vec![Similarity::InnerToSum]),
            Similarities(vec![Similarity::ProductToInner]),
        ]);
        let right = SimilaritiesList(vec![
            Similarities(vec![Similarity::Same]),
            Similarities(vec![Similarity::ProductToInner]),
        ]);
        assert!(left < right);
        assert!(right > left);
    }

    #[test]
    fn test_similarities_list_equal() {
        let left = SimilaritiesList(vec![
            Similarities(vec![Similarity::ProductToInner]),
            Similarities(vec![Similarity::InnerToSum]),
        ]);
        let right = SimilaritiesList(vec![
            Similarities(vec![Similarity::InnerToSum]),
            Similarities(vec![Similarity::ProductToInner]),
        ]);
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Equal));
        assert_eq!(right.partial_cmp(&left), Some(Ordering::Equal));
    }

    #[test]
    fn test_similarities_list_cmp_none() {
        let left = SimilaritiesList(vec![
            Similarities(vec![Similarity::InnerToSum]),
            Similarities(vec![Similarity::ProductToInner]),
        ]);
        let right = SimilaritiesList(vec![
            Similarities(vec![Similarity::Same]),
            Similarities(vec![Similarity::Infer]),
        ]);
        assert_eq!(left.partial_cmp(&right), None);
        assert_eq!(right.partial_cmp(&left), None);
    }

    #[test]
    fn test_similarities_list_max() {
        let list = SimilaritiesList(vec![
            Similarities(vec![Similarity::Same]),
            Similarities(vec![Similarity::ProductToInner]),
        ]);
        assert_eq!(list.max(), Similarities(vec![Similarity::Same]));
    }

    #[test]
    fn test_similarities_longer_is_greater() {
        let left = Similarities(vec![Similarity::Number]);
        let right = Similarities(vec![Similarity::ProductToInner, Similarity::Number]);
        assert!(left < right);
        assert!(right > left);
    }
}
