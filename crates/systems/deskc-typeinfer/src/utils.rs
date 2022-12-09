use crate::{
    ctx::{Ctx, Id},
    internal_type::Type,
};

// TODO: use subtyping before concat or push the type.
pub(crate) fn sum_all(_ctx: &Ctx, types: Vec<Type>) -> Type {
    let mut sum = types
        .into_iter()
        .map(|ty| match ty {
            Type::Sum(sum) => sum,
            other => vec![other],
        })
        .reduce(|a, b| a.into_iter().chain(b).collect())
        .unwrap_or_default();

    sum.sort();
    sum.dedup();
    if sum.len() == 1 {
        sum.pop().unwrap()
    } else {
        Type::Sum(sum)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct IdGen {
    pub next_id: Id,
}

impl IdGen {
    pub fn next_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct IdentGen {
    pub id_gen: IdGen,
}

impl IdentGen {
    pub fn next_ident(&mut self) -> String {
        let mut id = self.id_gen.next_id();
        let atoz: Vec<_> = ('a'..='z').into_iter().collect();
        let mut ret = String::new();
        let index = id % 26;
        ret.push(atoz[index]);
        id /= 26;
        while id != 0 {
            let index = match id % 26 {
                // 0 == 'z'
                0 => 25,
                // 1 == 'a'
                other => other - 1,
            };
            ret.insert(0, atoz[index]);
            id /= 26;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident_gen() {
        let mut gen = IdentGen::default();
        assert_eq!(gen.next_ident(), "a");
        assert_eq!(gen.next_ident(), "b");
        for _ in 0..26 - 4 {
            gen.next_ident();
        }
        assert_eq!(gen.next_ident(), "y");
        assert_eq!(gen.next_ident(), "z");
        assert_eq!(gen.next_ident(), "aa");
        assert_eq!(gen.next_ident(), "ab");
        for _ in 0..26 - 4 {
            gen.next_ident();
        }
        assert_eq!(gen.next_ident(), "ay");
        assert_eq!(gen.next_ident(), "az");
        assert_eq!(gen.next_ident(), "ba");
        assert_eq!(gen.next_ident(), "bb");
    }
}
