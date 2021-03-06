mod date_and_time;
mod numeric;

use std::io::prelude::*;

use deserialize::{self, FromSql};
use serialize::{self, Output, ToSql};
use super::Sqlite;
use super::connection::SqliteValue;
use sql_types;

/// The returned pointer is *only* valid for the lifetime to the argument of
/// `from_sql`. This impl is intended for uses where you want to write a new
/// impl in terms of `String`, but don't want to allocate. We have to return a
/// raw pointer instead of a reference with a lifetime due to the structure of
/// `FromSql`
impl FromSql<sql_types::VarChar, Sqlite> for *const str {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        let text = not_none!(value).read_text();
        Ok(text as *const _)
    }
}

/// The returned pointer is *only* valid for the lifetime to the argument of
/// `from_sql`. This impl is intended for uses where you want to write a new
/// impl in terms of `Vec<u8>`, but don't want to allocate. We have to return a
/// raw pointer instead of a reference with a lifetime due to the structure of
/// `FromSql`
impl FromSql<sql_types::Binary, Sqlite> for *const [u8] {
    fn from_sql(bytes: Option<&SqliteValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes).read_blob();
        Ok(bytes as *const _)
    }
}

impl FromSql<sql_types::SmallInt, Sqlite> for i16 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_integer() as i16)
    }
}

impl FromSql<sql_types::Integer, Sqlite> for i32 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_integer())
    }
}

impl FromSql<sql_types::Integer, Sqlite> for u8 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_integer() as u8)
    }
}

impl FromSql<sql_types::Integer, Sqlite> for u16 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_integer() as u16)
    }
}

impl FromSql<sql_types::Integer, Sqlite> for u32 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_integer() as u32)
    }
}

impl FromSql<sql_types::Bool, Sqlite> for bool {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_integer() != 0)
    }
}

impl FromSql<sql_types::BigInt, Sqlite> for i64 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_long())
    }
}

impl FromSql<sql_types::Float, Sqlite> for f32 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_double() as f32)
    }
}

impl FromSql<sql_types::Double, Sqlite> for f64 {
    fn from_sql(value: Option<&SqliteValue>) -> deserialize::Result<Self> {
        Ok(not_none!(value).read_double())
    }
}

impl ToSql<sql_types::Bool, Sqlite> for bool {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let int_value = if *self { 1 } else { 0 };
        <i32 as ToSql<sql_types::Integer, Sqlite>>::to_sql(&int_value, out)
    }
}

impl ToSql<sql_types::Integer, Sqlite> for u8 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let int_value = *self as i32;
        <i32 as ToSql<sql_types::Integer, Sqlite>>::to_sql(&int_value, out)
    }
}

impl ToSql<sql_types::Integer, Sqlite> for u16 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let int_value = *self as i32;
        <i32 as ToSql<sql_types::Integer, Sqlite>>::to_sql(&int_value, out)
    }
}

impl ToSql<sql_types::Integer, Sqlite> for u32 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let int_value = *self as i32;
        <i32 as ToSql<sql_types::Integer, Sqlite>>::to_sql(&int_value, out)
    }
}
