extern crate core;

use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::io::Bytes;
use serde_json::{Value};
use csv::{ByteRecord, QuoteStyle, Writer, WriterBuilder};
use serde::de::Unexpected::Str;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = r#"[
    {
        "name": "Testing",
        "age": 10,
        "phones": [
            "123 456 7890",
            "132 654 8367"
        ]
    },
    {
        "name": "Second",
        "age": 55,
        "phones": [
        ]
    }
    ]
    "#;

    let csv_data: Value = serde_json::from_str(data)?;

    if !csv_data.is_array() {
        panic!("The value {} is not an array", csv_data);
    }

    let mut items: Vec<HashMap<String, Value>> = vec!();

    for i in csv_data.as_array().unwrap_or(&vec![Value::Null]) {
        let mut next_item: HashMap<String, Value> = HashMap::new();
        if let Some(row) = i.as_object() {
            for (k, v) in row.iter() {
                println!("Test value: {}", v.to_string());

                next_item.insert(k.clone(), v.clone());
            }
        }

        items.push(next_item)
    }

    let mut all_keys: Vec<String>;

    {
        let mut discovered_keys: HashSet<String> = HashSet::new();

        for i in &items {
            for (k, _) in i {
                discovered_keys.insert(k.clone());
            }
        }

        all_keys = discovered_keys.iter().map(|k| k.clone()).collect::<Vec<String>>();
    }

    println!("Items: {:#?}", items.clone());
    println!("Keys: {:#?}", all_keys.clone());

    let mut writer = WriterBuilder::new().delimiter(b',')
        .quote_style(QuoteStyle::Necessary)
        .double_quote(false)
        .from_path("/Users/bryanwolf/projects/itis/testfile.csv")?;
        //.from_writer(vec![]);

    writer.write_record(all_keys.clone())?;

    for item_mapping in items {
        let mut values : ByteRecord = ByteRecord::new();

        for h in &all_keys {
            if let Some(v) = item_mapping.get(h.as_str()) {
                if let Some(value) = v.as_str()  {
                    values.push_field(value.as_bytes());
                } else if let Some(value) = v.as_i64() {

                    values.push_field(value.to_string().as_bytes());
                }
                else if let Some(value) = v.as_array() {
                    let merged_values = value.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join("|");
                    values.push_field(merged_values.as_bytes());
                }
                else {
                    panic!("Unexpected value type: {:#?}", values)
                }
            }
        }

        writer.write_byte_record(&values)?;
    }

    writer.flush()?;


    // let data = writer.into_inner()?;
    // println!("Data: {}", String::from_utf8(data)?);

    Ok(())
}
