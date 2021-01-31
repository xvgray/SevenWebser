use serde::*;

//use serde_json::{*};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Property {
    name: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Params {
    property: Vec<Property>,
}

pub fn dane_systemu_to_json(cpu: &str, ram: &str, os: &str)  {
    //let mut parametry = Params::from(Params);
    let procesor = Property { name: String::from("Processor"), value: String::from(cpu) };
    let pamiec = Property { name: String::from("Total Memory"), value: String::from(ram) };
    let system_op = Property { name: String::from("OS Data"), value: String::from(os) };
    let parametry = Params {
        property: vec!(procesor, pamiec, system_op)
    };
    let serialize_ustawienia = serde_json::to_string(&parametry).unwrap();
    let mut plik_json_data_system = File::create("Mods/SevenWebser/scripts/data_system.json").expect("Unable to create file");
    plik_json_data_system.write_all(serialize_ustawienia.as_bytes()).expect("Unable to write data");

}