extern crate csv;
#[macro_use]
extern crate json;

use csv::Reader;
use json::JsonValue;

pub struct Args {
    pub input: String,
    pub output: Option<String>,
    pub is_nulled: bool,
    pub is_keyed: bool,
}

fn update_json_with_record_row(
    mut json: JsonValue,
    record: Vec<String>,
    headers: &[String],
    args: &Args,
) -> JsonValue {
    let record: Vec<String> = record;

    let mut element = object!{};
    for index in 0..headers.len() {
        if index >= record.len() {
            break;
        }

        let header: &str = &headers[index][..];
        let value: &str = &record[index];

        if !args.is_keyed {
            if value.is_empty() && args.is_nulled {
                element[header] = json::Null;
            } else {
                element[header] = value.into();
            }
        } else {
            let key: &str = &record[0];
            if index == 0 {
                json[key] = object!{};
            } else {
                if value.is_empty() && args.is_nulled {
                    json[key][header] = json::Null;
                } else {
                    json[key][header] = value.into();
                }
            }
        }
    }
    if !args.is_keyed {
        json.push(element.clone())
            .expect("Error pushing element to json");
    }
    json
}


pub fn parse_csv(contents: &mut String, args: &Args) -> JsonValue {
    let mut json: JsonValue;
    if !args.is_keyed {
        json = array![];
    } else {
        json = object!{};
    }

    let mut rdr: Reader<&[u8]> = Reader::from_reader(contents.as_bytes());
    let headers: Vec<String> = rdr.headers()
        .expect("There was an error reading the headers.");

    let mut records_iter = rdr.records();

    while let Some(record) = records_iter.next() {
        json = update_json_with_record_row(json, record.unwrap(), &headers, &args);
    }

    json
}


#[test]
fn test_is_not_keyed() {
    let mut json: JsonValue = array![];
    let mut args: Args = Args {
        input: String::from("input"),
        output: Some(String::from("output")),
        is_nulled: false,
        is_keyed: false,
    };
    let record: Vec<String> = vec![String::from("a"), String::from(""), String::from("c")];
    let headers: Vec<String> = vec![
        String::from("header_a"),
        String::from("header_b"),
        String::from("header_c"),
    ];
    json = update_json_with_record_row(json, record, &headers, &args);
    assert_eq!(
        json.to_string(),
        array![
            object!{
                "header_a" => "a",
                "header_b" => "",
                "header_c" => "c"
            }
        ].to_string()
    );

    args.is_nulled = true;
    let mut json: JsonValue = array![];
    let record: Vec<String> = vec![String::from("a"), String::from(""), String::from("c")];
    json = update_json_with_record_row(json, record, &headers, &args);
    assert_eq!(
        json.to_string(),
        array![
            object!{
                "header_a" => "a",
                "header_b" => json::Null,
                "header_c" => "c"
            }
        ].to_string()
    );
}

#[test]
fn test_is_nulled() {
    let mut json: JsonValue = object!{};
    let mut args: Args = Args {
        input: String::from("input"),
        output: Some(String::from("output")),
        is_nulled: false,
        is_keyed: true,
    };
    let record: Vec<String> = vec![String::from("a"), String::from(""), String::from("c")];
    let headers: Vec<String> = vec![
        String::from("header_a"),
        String::from("header_b"),
        String::from("header_c"),
    ];
    json = update_json_with_record_row(json, record, &headers, &args);
    assert_eq!(
        json.to_string(),
        object!{
            "a" => object!{
                "header_b" => "",
                "header_c" => "c"
            }
        }.to_string()
    );

    args.is_nulled = true;

    let record: Vec<String> = vec![String::from("a"), String::from(""), String::from("c")];
    json = update_json_with_record_row(json, record, &headers, &args);
    assert_eq!(
        json.to_string(),
        object!{
            "a" => object!{
                "header_b" => json::Null,
                "header_c" => "c"
            }
        }.to_string()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn updating_json() {
        let mut json: JsonValue = object!{};
        let args: Args = Args {
            input: String::from("input"),
            output: Some(String::from("output")),
            is_nulled: false,
            is_keyed: true,
        };
        let record: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        let headers: Vec<String> = vec![
            String::from("header_a"),
            String::from("header_b"),
            String::from("header_c"),
        ];
        json = update_json_with_record_row(json, record, &headers, &args);
        assert_eq!(
            json.to_string(),
            object!{
                "a" => object!{
                    "header_b" => "b",
                    "header_c" => "c"
                }
            }.to_string()
        );

        // If there is less column on the record
        let mut json: JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        let headers: Vec<String> = vec![String::from("header_a"), String::from("header_b")];
        json = update_json_with_record_row(json, record, &headers, &args);
        assert_eq!(
            json.to_string(),
            object!{
                "a" => object!{
                    "header_b" => "b"
                }
            }.to_string()
        );

        // If there is one column on the record.
        let mut json: JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        let headers: Vec<String> = vec![String::from("header_a")];
        json = update_json_with_record_row(json, record, &headers, &args);
        assert_eq!(
            json.to_string(),
            object!{
                "a" => object!{
                }
            }.to_string()
        );

        // If there are more record columns than headers
        let mut json: JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b")];
        let headers: Vec<String> = vec![
            String::from("header_a"),
            String::from("header_b"),
            String::from("header_c"),
        ];
        json = update_json_with_record_row(json, record, &headers, &args);
        assert_eq!(
            json.to_string(),
            object!{
                "a" => object!{
                    "header_b" => "b"
                }
            }.to_string()
        );
    }
}
