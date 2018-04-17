extern crate csv;
#[macro_use]
extern crate json;
//use csv::Reader;
use json::JsonValue;

pub struct Args {
    pub input: String,
    pub output: Option<String>,
    pub is_nulled: bool,
    pub is_keyed: bool,
}

pub fn update_json_with_record_row(
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

// pub fn read_csv() {
    
// } 
