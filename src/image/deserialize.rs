//! Custom Serde deserializers
//!
use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer};
use std::{fmt, marker::PhantomData};

/// Source value may be quoted or a bare number. Output should always be a
/// string.
struct StringOrNumber(PhantomData<String>);

impl<'de> de::Visitor<'de> for StringOrNumber {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or number")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(value.to_owned())
    }
}

/// Source value may be a string or list of strings. Output value should always
/// be a list of strings.
struct StringOrVec(PhantomData<Vec<String>>);

impl<'de> de::Visitor<'de> for StringOrVec {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or list of strings")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(vec![value.to_owned()])
    }

    fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
    where
        S: de::SeqAccess<'de>,
    {
        Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
    }
}

struct DateTimeString(PhantomData<NaiveDateTime>);

impl<'de> de::Visitor<'de> for DateTimeString {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("date-time string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        NaiveDateTime::parse_from_str(value, "%Y:%m:%d %H:%M:%S%:z")
            .map_err(de::Error::custom)
    }
}

pub fn string_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrNumber(PhantomData))
}

/// Handle vector fields that have been serialized as a bare string when the
/// vector has only one member
pub fn string_sequence<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrVec(PhantomData))
}

pub fn date_time_string<'de, D>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DateTimeString(PhantomData))
}

#[cfg(test)]
mod tests {
    use super::{date_time_string, string_number, string_sequence};
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::Deserialize;
    use serde_json::from_str;

    #[test]
    fn sequence_test() {
        #[derive(Debug, Deserialize)]
        struct SomeStruct {
            #[serde(deserialize_with = "string_sequence")]
            field: Vec<String>,
        }

        let x: SomeStruct = from_str(r#"{ "field": ["a","b"] }"#).unwrap();
        //println!("{:?}", x);
        assert_eq!(x.field[0], "a");

        let x: SomeStruct = from_str(r#"{ "field": "c"}"#).unwrap();
        //println!("{:?}", x);
        assert_eq!(x.field[0], "c");
    }

    #[test]
    fn number_test() {
        #[derive(Debug, Deserialize)]
        struct SomeStruct {
            #[serde(deserialize_with = "string_number")]
            field: String,
        }

        let x: SomeStruct = from_str(r#"{ "field": "1/320" }"#).unwrap();
        assert_eq!(x.field, "1/320");

        let x: SomeStruct = from_str(r#"{ "field": 10 }"#).unwrap();
        assert_eq!(x.field, "10");

        let x: SomeStruct = from_str(r#"{ "field": 0.3 }"#).unwrap();
        assert_eq!(x.field, "0.3");
    }

    #[test]
    fn date_time_test() {
        #[derive(Debug, Deserialize)]
        struct SomeStruct {
            #[serde(deserialize_with = "date_time_string")]
            field: NaiveDateTime,
        }
        let dt: NaiveDateTime =
            NaiveDate::from_ymd(2018, 2, 8).and_hms(11, 1, 12);
        let x: SomeStruct =
            from_str(r#"{ "field": "2018:02:08 11:01:12-06:00"}"#).unwrap();

        assert_eq!(x.field, dt);
    }
}
