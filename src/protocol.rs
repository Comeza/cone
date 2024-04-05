use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::fmt::Write;

#[derive(Default)]
pub struct ProtocolSerializer {
    output: String,
}

impl ProtocolSerializer {
    pub fn to_string<T>(t: &T) -> Result<String, <&mut ProtocolSerializer as Serializer>::Error>
    where
        T: Serialize,
    {
        let mut ser = Self::default();
        t.serialize(&mut ser)?;
        Ok(ser.output)
    }
}

#[derive(Debug)]
pub enum ProtocolSerializeError {
    UnsupportedType(&'static str),
    FormatError(std::fmt::Error),
    Custom(String),
}

impl std::fmt::Display for ProtocolSerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolSerializeError::UnsupportedType(t) => write!(f, "Type '{t}' is not supported"),
            ProtocolSerializeError::FormatError(fmterr) => write!(f, "Format error: {fmterr}"),
            ProtocolSerializeError::Custom(custom) => write!(f, "{custom}"),
        }
    }
}

impl std::error::Error for ProtocolSerializeError {}

impl serde::ser::Error for ProtocolSerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl From<std::fmt::Error> for ProtocolSerializeError {
    fn from(value: std::fmt::Error) -> Self {
        Self::FormatError(value)
    }
}

impl SerializeSeq for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl SerializeTuple for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl SerializeTupleStruct for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        write!(self.output, " ")?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTupleVariant for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        write!(self.output, " ")?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeMap for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl SerializeStruct for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        write!(self.output, " ")?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeStructVariant for &mut ProtocolSerializer {
    type Ok = <Self as Serializer>::Ok;
    type Error = <Self as Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        write!(self.output, " ")?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl Serializer for &mut ProtocolSerializer {
    type Ok = ();

    type Error = ProtocolSerializeError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // maybe output this with quotes
        Ok(write!(self.output, "{v}")?)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ProtocolSerializeError::UnsupportedType("&[u8]"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "None")?)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{name}")?)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(write!(self.output, "{variant}")?)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        write!(self.output, "{} ", name.to_uppercase())?;
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_newtype_struct(variant, value)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ProtocolSerializeError::UnsupportedType("Sequences"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ProtocolSerializeError::UnsupportedType("Tuple"))
    }

    fn serialize_tuple_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        write!(self.output, "{name}")?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        write!(self.output, "{variant}")?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ProtocolSerializeError::UnsupportedType("Maps"))
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        write!(self.output, "{name}")?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        write!(self.output, "{variant}")?;
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::*;
    use serde::Serialize;

    #[test]
    fn ser_simple() {
        #[derive(Serialize)]
        enum SimpleProtocol {
            A,
        }

        let out = ProtocolSerializer::to_string(&SimpleProtocol::A).unwrap();
        assert_eq!(out, "A");
    }

    #[test]
    fn ser_variant() {
        #[derive(Serialize)]
        enum SimpleProtocol {
            A(String),
            B(u8),
            C(u8, String),
        }

        let out = ProtocolSerializer::to_string(&SimpleProtocol::A("Test".into())).unwrap();
        assert_eq!(out, "A Test");
        let out = ProtocolSerializer::to_string(&SimpleProtocol::B(10)).unwrap();
        assert_eq!(out, "B 10");
        let out = ProtocolSerializer::to_string(&SimpleProtocol::C(10, "lol".into())).unwrap();
        assert_eq!(out, "C 10 lol");
    }
}
