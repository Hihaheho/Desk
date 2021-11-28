use crate::{Error, Result};
use dson::{Dson, Literal};
use serde::{
    ser::{
        self, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};

pub struct Serializer;

pub fn to_dson<T>(value: &T) -> Result<Dson>
where
    T: Serialize,
{
    let mut serializer = Serializer;
    let dson = value.serialize(&mut serializer)?;
    Ok(dson)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Dson;
    type Error = Error;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = VariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = SeqSerializer;
    type SerializeStructVariant = VariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(Dson::Labeled {
            label: if v { "true" } else { "false" }.into(),
            expr: Box::new(Dson::Product(vec![])),
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(Dson::Literal(Literal::Int(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(Error::Message("u64 is not supported".into()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(Dson::Literal(Literal::Float(v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Ok(Dson::Literal(Literal::String(v.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(Dson::Literal(Literal::String(v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            SerializeSeq::serialize_element(&mut seq, byte)?;
        }
        SerializeSeq::end(seq)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(Dson::Product(vec![]))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(Dson::Product(vec![]))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(Dson::Labeled {
            label: name.into(),
            expr: Box::new(Dson::Product(vec![])),
        })
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Dson::Labeled {
            label: variant.into(),
            expr: Box::new(Dson::Product(vec![])),
        })
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Ok(Dson::Labeled {
            label: name.into(),
            expr: Box::new(value.serialize(self)?),
        })
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Ok(Dson::Labeled {
            label: variant.into(),
            expr: Box::new(value.serialize(self)?),
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::default())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(SeqSerializer::default())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SeqSerializer::default())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _ariant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(VariantSerializer {
            label: variant.into(),
            values: vec![],
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer::default())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(SeqSerializer::default())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(VariantSerializer {
            label: variant.into(),
            values: vec![],
        })
    }
}

#[derive(Default)]
pub struct SeqSerializer(Vec<Dson>);

impl<'a> SerializeSeq for SeqSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(&mut Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Dson::Array(self.0))
    }
}
impl<'a> SerializeTuple for SeqSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(&mut Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Dson::Product(self.0))
    }
}
impl<'a> SerializeTupleStruct for SeqSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(&mut Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Dson::Set(self.0))
    }
}
impl<'a> SerializeTupleVariant for VariantSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.values.push(value.serialize(&mut Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Dson::Labeled {
            label: self.label,
            expr: Box::new(Dson::Product(self.values)),
        })
    }
}

#[derive(Default)]
pub struct MapSerializer(Vec<Dson>, Vec<Dson>);

impl<'a> SerializeMap for MapSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(&mut Serializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.1.push(value.serialize(&mut Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let entries = self
            .0
            .into_iter()
            .zip(self.1.into_iter())
            .map(|(key, value)| {
                Dson::Product(vec![
                    Dson::Labeled {
                        label: "key".into(),
                        expr: Box::new(key),
                    },
                    value,
                ])
            })
            .collect();
        Ok(Dson::Set(entries))
    }
}
impl<'a> SerializeStruct for SeqSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, name: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.0.push(Dson::Labeled {
            label: name.to_string(),
            expr: Box::new(value.serialize(&mut Serializer)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Dson::Set(self.0))
    }
}

pub struct VariantSerializer {
    label: String,
    values: Vec<Dson>,
}

impl<'a> SerializeStructVariant for VariantSerializer {
    type Ok = Dson;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, name: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.values.push(Dson::Labeled {
            label: name.to_string(),
            expr: Box::new(value.serialize(&mut Serializer)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Dson::Labeled {
            label: self.label,
            expr: Box::new(Dson::Product(self.values)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::to_dson;
    use dson::{Dson, Literal};
    use serde::Serialize;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };

        assert_eq!(
            to_dson(&test).unwrap(),
            Dson::Set(vec![
                Dson::Labeled {
                    label: "int".into(),
                    expr: Box::new(Dson::Literal(Literal::Int(1)))
                },
                Dson::Labeled {
                    label: "seq".into(),
                    expr: Box::new(Dson::Array(vec![
                        Dson::Literal(Literal::String("a".into())),
                        Dson::Literal(Literal::String("b".into()))
                    ]))
                },
            ])
        );
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        assert_eq!(
            to_dson(&E::Unit).unwrap(),
            Dson::Labeled {
                label: "Unit".into(),
                expr: Box::new(Dson::Product(vec![]))
            }
        );

        assert_eq!(
            to_dson(&E::Newtype(1)).unwrap(),
            Dson::Labeled {
                label: "Newtype".into(),
                expr: Box::new(Dson::Literal(Literal::Int(1)))
            }
        );

        assert_eq!(
            to_dson(&E::Tuple(1, 2)).unwrap(),
            Dson::Labeled {
                label: "Tuple".into(),
                expr: Box::new(Dson::Product(vec![
                    Dson::Literal(Literal::Int(1)),
                    Dson::Literal(Literal::Int(2))
                ]))
            }
        );

        assert_eq!(
            to_dson(&E::Struct { a: 1 }).unwrap(),
            Dson::Labeled {
                label: "Struct".into(),
                expr: Box::new(Dson::Product(vec![Dson::Labeled {
                    label: "a".into(),
                    expr: Box::new(Dson::Literal(Literal::Int(1)))
                }]))
            }
        );
    }
}
