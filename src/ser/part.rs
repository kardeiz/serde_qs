use dtoa;
use itoa;
use ser::Error;
use serde;
use serde::ser;
use std::marker::PhantomData;
use std::str;

pub struct PartSerializer<SO, S: Sink<SO, Error>> {
    sink: S,
    marker: PhantomData<SO>,
}

impl<SO, S: Sink<SO, Error>> PartSerializer<SO, S> {
    pub fn new(sink: S) -> Self {
        PartSerializer {
            sink: sink,
            marker: PhantomData,
        }
    }
}

pub trait Sink<S, E>: Sized + ser::SerializeStruct<Ok=S, Error=E>
                            + ser::SerializeSeq<Ok=S, Error=E>
  where E: serde::ser::Error
{
// type Ok;
// type Error;
    fn serialize_static_str(self,
                            value: &'static str)
                            -> Result<S, Error>;

    fn serialize_str(self, value: &str) -> Result<S, Error>;
    fn serialize_string(self, value: String) -> Result<S, Error>;
    fn serialize_none(self) -> Result<S, Error>;

    fn serialize_some<T: ?Sized + ser::Serialize>
        (self,
         value: &T)
         -> Result<S, Error>;

    fn unsupported(&self) -> Error;
}

impl<SO, S: Sink<SO, Error>> ser::Serializer for PartSerializer<SO, S> {
    type Ok = SO;
    type Error = Error;
    type SerializeSeq = S;
    type SerializeTuple = ser::Impossible<SO, Error>;
    type SerializeTupleStruct = ser::Impossible<SO, Error>;
    type SerializeTupleVariant = ser::Impossible<SO, Error>;
    type SerializeMap = ser::Impossible<SO, Error>;
    type SerializeStruct = S;
    type SerializeStructVariant = ser::Impossible<SO, Error>;

    fn serialize_bool(self, v: bool) -> Result<SO, Error> {
        self.sink.serialize_static_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<SO, Error> {
        self.serialize_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<SO, Error> {
        self.serialize_floating(v)
    }

    fn serialize_f64(self, v: f64) -> Result<SO, Error> {
        self.serialize_floating(v)
    }

    fn serialize_char(self, v: char) -> Result<SO, Error> {
        self.sink.serialize_string(v.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<SO, Error> {
        self.sink.serialize_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<SO, Error> {
        match str::from_utf8(value) {
            Ok(value) => self.sink.serialize_str(value),
            Err(err) => Err(Error::Utf8(err)),
        }
    }

    fn serialize_unit(self) -> Result<SO, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<SO, Error> {
        self.sink.serialize_static_str(name.into())
    }

    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: usize,
                              variant: &'static str)
                              -> Result<SO, Error> {
        self.sink.serialize_static_str(variant.into())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>
        (self,
         _name: &'static str,
         value: &T)
         -> Result<SO, Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>
        (self,
         _name: &'static str,
         _variant_index: usize,
         _variant: &'static str,
         _value: &T)
         -> Result<SO, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_none(self) -> Result<SO, Error> {
        self.sink.serialize_none()
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self,
                                                  value: &T)
                                                  -> Result<SO, Error> {
        self.sink.serialize_some(value)
    }

    fn serialize_seq(self,
                     _len: Option<usize>)
                     -> Result<Self::SerializeSeq, Error> {
        Ok(self.sink)
    }


    fn serialize_seq_fixed_size(self,
                                _len: usize)
                                -> Result<Self::SerializeSeq, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_tuple(self,
                       _len: usize)
                       -> Result<Self::SerializeTuple, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> Result<Self::SerializeTuple, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_tuple_variant
        (self,
         _name: &'static str,
         _variant_index: usize,
         _variant: &'static str,
         _len: usize)
         -> Result<Self::SerializeTupleVariant, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_map(self,
                     _len: Option<usize>)
                     -> Result<Self::SerializeMap, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_struct(self,
                        _name: &'static str,
                        _len: usize)
                        -> Result<Self::SerializeStruct, Error> {
        // Err(self.sink.unsupported())
        Ok(self.sink)

    }

    fn serialize_struct_variant
        (self,
         _name: &'static str,
         _variant_index: usize,
         _variant: &'static str,
         _len: usize)
         -> Result<Self::SerializeStructVariant, Error> {
        Err(self.sink.unsupported())
    }
}

impl<SO, S: Sink<SO, Error>> PartSerializer<SO, S> {
    fn serialize_integer<I>(self, value: I) -> Result<SO, Error>
        where I: itoa::Integer,
    {
        let mut buf = [b'\0'; 20];
        let len = itoa::write(&mut buf[..], value).unwrap();
        let part = unsafe { str::from_utf8_unchecked(&buf[0..len]) };
        ser::Serializer::serialize_str(self, part)
    }

    fn serialize_floating<F>(self, value: F) -> Result<SO, Error>
        where F: dtoa::Floating,
    {
        let mut buf = [b'\0'; 24];
        let len = dtoa::write(&mut buf[..], value).unwrap();
        let part = unsafe { str::from_utf8_unchecked(&buf[0..len]) };
        ser::Serializer::serialize_str(self, part)
    }
}
