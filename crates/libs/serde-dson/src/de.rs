use crate::{Error, Result};
use dson::{Dson, Literal, MapElem};
use serde::{
    de::{self, Deserialize, EnumAccess, MapAccess, SeqAccess, VariantAccess},
    forward_to_deserialize_any,
};

pub fn from_dson<'a, T>(dson: Dson) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer(dson);
    T::deserialize(&mut deserializer)
}

pub struct Deserializer(Dson);

impl Deserializer {
    pub fn from_dson(dson: Dson) -> Self {
        Self(dson)
    }
}

fn unwrap(dson: &mut Dson) {
    while let Dson::Attributed { expr, .. }
    | Dson::Labeled { expr, .. }
    | Dson::Comment { expr, .. }
    | Dson::Typed { expr, .. } = dson
    {
        *dson = (**expr).clone();
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unwrap(&mut self.0);
        match &self.0 {
            Dson::Literal(Literal::Integer(int)) => visitor.visit_i64(*int),
            Dson::Literal(Literal::Real(float)) => visitor.visit_f64(float.0),
            Dson::Literal(Literal::Rational(a, b)) => visitor.visit_f64(*a as f64 / *b as f64),
            Dson::Literal(Literal::String(string)) => visitor.visit_string(string.clone()),
            Dson::Product(values) => {
                if values.is_empty() {
                    visitor.visit_unit()
                } else {
                    visitor.visit_seq(ValuesDeserializer::new(values.clone()))
                }
            }
            Dson::Vector(values) => visitor.visit_seq(ValuesDeserializer::new(values.clone())),
            Dson::Map(values) => visitor.visit_map(MapDeserializer::new(values.clone())),
            // These must be handled in unwrap(&mut self.0).
            Dson::Labeled { .. } => panic!(),
            Dson::Attributed { .. } => panic!(),
            Dson::Typed { .. } => panic!(),
            Dson::Comment { .. } => todo!(),
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit seq identifier ignored_any
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Labeled { label: v, expr: _ } if v == "true" => visitor.visit_bool(true),
            Dson::Labeled { label: v, expr: _ } if v == "false" => visitor.visit_bool(false),
            _ => Err(Error::ExpectedBool {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Product(values) if values.is_empty() => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Product(values) if values.is_empty() => visitor.visit_unit(),
            _ => Err(Error::ExpectedProduct {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Map(elems) => {
                let values = elems
                    .iter()
                    .map(|MapElem { key, value }| {
                        Ok(MapElem {
                            key: key.clone(),
                            value: value.clone(),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                visitor.visit_map(MapDeserializer::new(values))
            }
            _ => Err(Error::ExpectedMap {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Labeled {
                label: variant,
                expr,
            } => visitor.visit_enum(EnumDeserializer(variant.clone(), *expr.clone())),
            _ => Err(Error::ExpectedLabel {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Labeled { label, expr } => {
                if name == label {
                    visitor
                        .visit_newtype_struct(&mut Deserializer::from_dson(expr.as_ref().clone()))
                } else {
                    Err(Error::LabelMismatch {
                        expected: name.into(),
                        got: label.clone(),
                    })
                }
            }
            _ => Err(Error::ExpectedLabel {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.0 {
            Dson::Product(values) => visitor.visit_seq(ValuesDeserializer::new(values.clone())),
            _ => Err(Error::ExpectedProduct {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unwrap(&mut self.0);
        match &self.0 {
            Dson::Product(values) => visitor.visit_seq(ValuesDeserializer::new(values.clone())),
            _ => Err(Error::ExpectedProduct {
                got: self.0.clone(),
            }),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unwrap(&mut self.0);
        match &self.0 {
            Dson::Product(values) => {
                let values = values
                    .iter()
                    .map(|dson| match dson {
                        Dson::Labeled { label, expr } => Ok(MapElem {
                            key: Dson::Literal(Literal::String(label.clone())),
                            value: *expr.clone(),
                        }),
                        _ => Err(Error::ExpectedLabel {
                            got: (*dson).clone(),
                        }),
                    })
                    .collect::<Result<Vec<_>>>()?;
                visitor.visit_map(MapDeserializer::new(values))
            }
            _ => Err(Error::ExpectedProduct {
                got: self.0.clone(),
            }),
        }
    }
}

pub struct ValuesDeserializer(Vec<Dson>);

impl ValuesDeserializer {
    fn new(values: Vec<Dson>) -> Self {
        ValuesDeserializer(values.into_iter().rev().collect())
    }
}

impl<'de> SeqAccess<'de> for ValuesDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.0
            .pop()
            .map(|dson| seed.deserialize(&mut Deserializer::from_dson(dson)))
            .transpose()
    }
}

pub struct MapDeserializer(Vec<MapElem>);
impl MapDeserializer {
    fn new(values: Vec<MapElem>) -> Self {
        MapDeserializer(values.into_iter().rev().collect())
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.0
            .last()
            .map(|value| value.key.clone())
            .map(|value| seed.deserialize(&mut Deserializer::from_dson(value)))
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer::from_dson(self.0.pop().unwrap().value))
    }
}

pub struct EnumDeserializer(String, Dson);

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = Deserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(&mut Deserializer(Dson::Literal(Literal::String(
                self.0.clone(),
            ))))?,
            Deserializer(self.1),
        ))
    }
}

impl<'de> VariantAccess<'de> for Deserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer(self.0))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(&mut Deserializer(self.0), len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(&mut Deserializer(self.0), "not used", fields, visitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let dson = Dson::Product(vec![
            Dson::Labeled {
                label: "int".into(),
                expr: Box::new(Dson::Literal(Literal::Integer(1))),
            },
            Dson::Labeled {
                label: "seq".into(),
                expr: Box::new(Dson::Vector(vec![
                    Dson::Literal(Literal::String("a".into())),
                    Dson::Literal(Literal::String("b".into())),
                ])),
            },
        ]);
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, from_dson(dson).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, String),
            Struct { a: u32 },
        }

        let dson = Dson::Labeled {
            label: "Unit".into(),
            expr: Box::new(Dson::Product(vec![])),
        };
        let expected = E::Unit;
        assert_eq!(expected, from_dson(dson).unwrap());

        let dson = Dson::Labeled {
            label: "Newtype".into(),
            expr: Box::new(Dson::Literal(Literal::Integer(1))),
        };
        let expected = E::Newtype(1);
        assert_eq!(expected, from_dson(dson).unwrap());

        let dson = Dson::Labeled {
            label: "Tuple".into(),
            expr: Box::new(Dson::Product(vec![
                Dson::Literal(Literal::Integer(1)),
                Dson::Literal(Literal::String("a".into())),
            ])),
        };
        let expected = E::Tuple(1, "a".into());
        assert_eq!(expected, from_dson(dson).unwrap());

        let dson = Dson::Labeled {
            label: "Struct".into(),
            expr: Box::new(Dson::Product(vec![Dson::Labeled {
                label: "a".into(),
                expr: Box::new(Dson::Literal(Literal::Integer(1))),
            }])),
        };
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_dson(dson).unwrap());
    }

    #[test]
    fn test_map() {
        use std::collections::HashMap;

        let dson = Dson::Map(vec![
            MapElem {
                key: "a".into(),
                value: 1.into(),
            },
            MapElem {
                key: "b".into(),
                value: 2.into(),
            },
        ]);
        let expected = [("a".to_string(), 1), ("b".to_string(), 2)]
            .into_iter()
            .collect::<HashMap<_, i64>>();
        assert_eq!(expected, from_dson(dson).unwrap());
    }

    #[test]
    fn test_vec() {
        let dson = Dson::Vector(vec![1.into(), 2.into()]);
        let expected = vec![1_i32, 2_i32];
        assert_eq!(expected, from_dson::<'_, Vec<i32>>(dson).unwrap());
    }

    #[test]
    fn test_map_key_number() {
        use std::collections::HashMap;

        let dson = Dson::Map(vec![MapElem {
            key: 1.into(),
            value: 2.into(),
        }]);
        let expected = [(1, 2)].into_iter().collect::<HashMap<_, i64>>();
        assert_eq!(expected, from_dson(dson).unwrap());
    }

    #[test]
    fn test_struct_in_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        pub struct Wrapper {
            pub inner: Inner,
        }
        #[derive(Deserialize, PartialEq, Debug)]
        pub struct Inner(pub u32);
        let expected = Wrapper { inner: Inner(1) };
        let dson = Dson::Product(vec![Dson::Labeled {
            label: "inner".into(),
            expr: Box::new(Dson::Labeled {
                label: "Inner".into(),
                expr: Box::new(1.into()),
            }),
        }]);
        assert_eq!(expected, from_dson(dson).unwrap());
    }
}
