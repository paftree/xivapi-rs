use serde::{de, ser::{self, Error as SerError, Serialize, Serializer}};

use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
  Message(String),
}

impl ser::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str(std::error::Error::description(self))
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Message(ref msg) => msg,
    }
  }
}

pub struct CommaSerializer {
  output: String,
}

impl CommaSerializer {
  pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut serializer = CommaSerializer {
      output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
  }

  pub fn with<'a, I, T, S>(inputs: &'a I, serializer: S) -> std::result::Result<S::Ok, S::Error>
      where S: Serializer,
            T: Serialize,
            for<'b> &'b I: IntoIterator<Item = &'b T>,
  {
    let inputs: Vec<&T> = inputs.into_iter().collect();
    serializer.serialize_str(&CommaSerializer::to_string(&inputs).unwrap())
  }
}

impl<'a> ser::Serializer for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  fn serialize_bool(self, v: bool) -> Result<()> {
    self.output += if v { "true" } else { "false" };
    Ok(())
  }

  fn serialize_i8(self, v: i8) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i16(self, v: i16) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i32(self, v: i32) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i64(self, v: i64) -> Result<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_u8(self, v: u8) -> Result<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u16(self, v: u16) -> Result<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u32(self, v: u32) -> Result<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u64(self, v: u64) -> Result<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_f32(self, v: f32) -> Result<()> {
    self.serialize_f64(f64::from(v))
  }

  fn serialize_f64(self, v: f64) -> Result<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_char(self, v: char) -> Result<()> {
    self.serialize_str(&v.to_string())
  }

  fn serialize_str(self, v: &str) -> Result<()> {
    self.output += v;
    Ok(())
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<()> {
    use serde::ser::SerializeSeq;
    let mut seq = self.serialize_seq(Some(v.len()))?;
    for byte in v {
      seq.serialize_element(byte)?;
    }
    seq.end()
  }

  fn serialize_none(self) -> Result<()> {
    self.serialize_unit()
  }

  fn serialize_some<T>(self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<()> {
    self.output += "null";
    Ok(())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
    self.serialize_unit()
  }

  fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
    self.serialize_str(variant)
  }

  fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    value.serialize(&mut *self)?;
    Ok(())
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
    Ok(self)
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
    Err(bad_type())
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    Err(bad_type())
  }

  fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
    Err(bad_type())
  }

  fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
    Err(bad_type())
  }
}

fn bad_type() -> Error {
  Error::custom("unsupported type")
}

impl<'a> ser::SerializeSeq for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    Ok(())
  }
}

impl<'a> ser::SerializeTuple for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    Ok(())
  }
}

impl<'a> ser::SerializeTupleStruct for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += ",";
    }
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<()> {
    Ok(())
  }
}

impl<'a> ser::SerializeTupleVariant for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    Err(bad_type())
  }

  fn end(self) -> Result<()> {
    Err(bad_type())
  }
}

impl<'a> ser::SerializeMap for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_key<T>(&mut self, _value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    Err(bad_type())
  }

  fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    Err(bad_type())
  }

  fn end(self) -> Result<()> {
    Err(bad_type())
  }
}

impl<'a> ser::SerializeStruct for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    Err(bad_type())
  }

  fn end(self) -> Result<()> {
    Err(bad_type())
  }
}

impl<'a> ser::SerializeStructVariant for &'a mut CommaSerializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where T: ?Sized + Serialize,
  {
    Err(bad_type())
  }

  fn end(self) -> Result<()> {
    Err(bad_type())
  }
}
