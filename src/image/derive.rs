use serde::{
    de::{value::SeqAccessDeserializer, IntoDeserializer, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker};

// Adapted from https://stackoverflow.com/questions/41151080/deserialize-a-json-string-or-array-of-strings-into-a-vec

#[derive(Debug, Deserialize)]
struct SequenceString(String);

/// Deserializes a string or a sequence of strings into a vector of the target
/// type
pub fn deserialize_sequence_string<'de, T, D>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct CustomVisitor<T>(marker::PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for CustomVisitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, f: &mut fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "a string or sequence of strings")
        }

        fn visit_str<E: serde::de::Error>(
            self,
            v: &str,
        ) -> Result<Self::Value, E> {
            let value = ({
                let deserializer = IntoDeserializer::into_deserializer(v);

                Deserialize::deserialize(deserializer)
            })?;
            Ok(vec![value])
        }

        fn visit_seq<A: SeqAccess<'de>>(
            self,
            visitor: A,
        ) -> Result<Self::Value, A::Error> {
            Deserialize::deserialize(SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(CustomVisitor(marker::PhantomData))
}

#[cfg(test)]
mod tests {
    use super::{deserialize_sequence_string, SequenceString};
    use serde::Deserialize;
    use serde_json::from_str;

    #[test]
    fn derive_test() {
        #[derive(Debug, Deserialize)]
        struct SomeStruct {
            #[serde(deserialize_with = "deserialize_sequence_string")]
            field1: Vec<SequenceString>,
        }

        let x: SomeStruct = from_str(r#"{ "field1": ["a","b"] }"#).unwrap();
        println!("{:?}", x);
        assert_eq!(x.field1[0].0, "a");

        let x: SomeStruct = from_str(r#"{ "field1": "c"}"#).unwrap();
        println!("{:?}", x);
        assert_eq!(x.field1[0].0, "c");
    }
}
