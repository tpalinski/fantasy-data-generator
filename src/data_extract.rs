use std::fs::File;

use serde::de::DeserializeOwned;

pub struct DataExtract<T>(T);

impl <T> DataExtract<T> {
    pub fn get_data(path: &str) -> Vec<T>
    where T: DeserializeOwned
    {
        let file = File::open(path).expect("Error opening file");
        let mut rdr = csv::Reader::from_reader(file);
        let res: Vec<T> = rdr.deserialize().map(|record| {
            let s: T = match record {
                Ok(r) => r,
                Err(_) => {
                    panic!("Error while deserializing data!");
                }
            };
            return s;
        }).collect();
        res
    }

    pub fn write_data(data: Vec<T>, path: &str)
    where T: serde::Serialize
    {
        let file = File::create(path).expect("error while opening file");
        let mut wrt = csv::Writer::from_writer(file);
        for record in data {
            let _ = wrt.serialize(record);
        }
    }
}
