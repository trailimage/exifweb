//! Custom Serde deserializers

use chrono::{DateTime, FixedOffset, Local};
use lazy_static::*;
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use std::{fmt, marker::PhantomData};

/// Source value may be quoted or a bare number. Output should always be a
/// string.
struct StringOrNumber(PhantomData<Option<String>>);

// impl StringOrNumber {
//     fn out<T>(value: dyn ToString) -> Result<Option<String>, E> {
//         Ok(Some(value.to_string()))
//     }
// }

impl<'de> de::Visitor<'de> for StringOrNumber {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or number")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Some(value.to_string()))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Some(value.to_string()))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(Some(value.to_string()))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Some(value.to_owned()))
    }
}

pub fn string_number<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrNumber(PhantomData))
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

/// Handle vector fields that have been serialized as a bare string when the
/// vector has only one member
pub fn string_sequence<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrVec(PhantomData))
}

struct DateTimeString(PhantomData<DateTime<FixedOffset>>);

impl<'de> de::Visitor<'de> for DateTimeString {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("date-time string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        lazy_static! {
            static ref TZ: Regex = Regex::new(r"[+-]\d{2}:\d{2}$").unwrap();
            static ref OFFSET: String =
                format!("{}", Local::today().format("%:z"));
        }
        let mut d = value.to_owned();

        if !TZ.is_match(value) {
            // append local timezone offset if not included
            d.push_str(&OFFSET);
        }

        DateTime::parse_from_str(&d, "%Y:%m:%d %H:%M:%S%:z")
            .map_err(de::Error::custom)
    }
}

pub fn date_time_string<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DateTimeString(PhantomData))
}

struct RegExString(PhantomData<Regex>);

impl<'de> de::Visitor<'de> for RegExString {
    type Value = Regex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("regular expression")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Regex::new(value).map_err(de::Error::custom)
    }
}

pub fn regex_string<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(RegExString(PhantomData))
}

#[cfg(test)]
mod tests {
    use super::{
        date_time_string, regex_string, string_number, string_sequence,
    };
    use chrono::{DateTime, FixedOffset, Local};
    use regex::Regex;
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
            field: Option<String>,
        }

        let x: SomeStruct = from_str(r#"{ "field": "1/320" }"#).unwrap();
        assert_eq!(x.field, Some("1/320".to_owned()));

        let x: SomeStruct = from_str(r#"{ "field": 10 }"#).unwrap();
        assert_eq!(x.field, Some("10".to_owned()));

        let x: SomeStruct = from_str(r#"{ "field": 0.3 }"#).unwrap();
        assert_eq!(x.field, Some("0.3".to_owned()));

        // let x: SomeStruct = from_str(r#"{ "field": null }"#).unwrap();
        // assert_eq!(x.field, None);
    }

    #[test]
    fn date_time_test() {
        #[derive(Debug, Deserialize)]
        struct SomeStruct {
            #[serde(deserialize_with = "date_time_string")]
            field: DateTime<FixedOffset>,
        }
        let dt: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2018-02-08T11:01:12-06:00").unwrap();

        let x: SomeStruct =
            from_str(r#"{ "field": "2018:02:08 11:01:12-06:00"}"#).unwrap();

        assert_eq!(x.field, dt);

        // adds missing timezone
        let dt: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2013-10-05T16:18:35-06:00").unwrap();

        let x: SomeStruct =
            from_str(r#"{ "field": "2013:10:05 16:18:35"}"#).unwrap();

        assert_eq!(x.field, dt);
    }

    #[test]
    fn regex_test() {
        #[derive(Debug, Deserialize)]
        struct SomeStruct {
            #[serde(deserialize_with = "regex_string")]
            field: Regex,
        }
        let x: SomeStruct =
            from_str(r#"{ "field": "^(?P<index>\\d+)\\."}"#).unwrap();

        assert!(x.field.is_match("1.something"));
    }
}
