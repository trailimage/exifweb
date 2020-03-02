use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Item {
    #[serde(rename = "li")]
    value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Items {
    #[serde(rename = "li")]
    value: Vec<String>,
}

/// Alternative container
/// https://www.w3.org/TR/rdf-schema/#ch_alt
#[derive(Deserialize, Debug, PartialEq)]
pub struct Alt {
    #[serde(rename = "Alt")]
    item: Item,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct List {
    #[serde(rename = "Bag")]
    items: Items,
}

#[derive(Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "UsageTerms")]
    usage_terms: Alt,

    title: Alt,
    rights: Alt,
    description: Alt,

    #[serde(rename = "subject")]
    tags: List,
}

#[derive(Deserialize, Debug)]
pub struct XMP {
    #[serde(rename = "Description")]
    description: Description,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn xmp_item_test() {
        let src = r#"<rdf:Alt xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <rdf:li xml:lang="x-default">Time to move on</rdf:li>
         </rdf:Alt>"#;

        let should_be = Item {
            value: "Time to move on".to_string(),
        };

        let item: Item = from_str(src).unwrap();

        assert_eq!(item, should_be);
    }
    #[test]
    fn xmp_text_test() {
        let src = r#"<xmpRights:UsageTerms
                xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
                xmlns:xmpRights="http://ns.adobe.com/xap/1.0/rights/">
            <rdf:Alt>
                <rdf:li xml:lang="x-default">All Rights Reserved</rdf:li>
            </rdf:Alt>
        </xmpRights:UsageTerms>"#;

        let item = Item {
            value: "All Rights Reserved".to_string(),
        };
        let should_be = Alt { item };
        let text: Alt = from_str(src).unwrap();

        assert_eq!(text, should_be);
    }

    #[test]
    fn xmp_list_test() {
        let src = r#"<dc:subject
                xmlns:dc="http://purl.org/dc/elements/1.1/"
                xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <rdf:Bag>
                <rdf:li>BMW R1200GS Adventure</rdf:li>
                <rdf:li>Fire</rdf:li>
                <rdf:li>Honda CRF 230L</rdf:li>
                <rdf:li>Motorcycle</rdf:li>
                <rdf:li>Mountain</rdf:li>
                <rdf:li>Tent</rdf:li>
                <rdf:li>Yamaha XT 250</rdf:li>
            </rdf:Bag>
        </dc:subject>"#;

        let items = Items {
            value: vec![
                "BMW R1200GS Adventure".to_string(),
                "Fire".to_string(),
                "Honda CRF 230L".to_string(),
                "Motorcycle".to_string(),
                "Mountain".to_string(),
                "Tent".to_string(),
                "Yamaha XT 250".to_string(),
            ],
        };

        let should_be = List { items };
        let list: List = from_str(src).unwrap();

        assert_eq!(list, should_be);
    }
}
