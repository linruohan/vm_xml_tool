use crate::Domain;
use quick_xml::{de::from_reader, se::to_string};
use std::fs::File;
use std::io::{Read, Write};
pub fn read_vm_config(path: &str) -> Result<Domain, Box<dyn std::error::Error>> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    from_reader(contents.as_bytes()).map_err(|e| format!("XML parsing error: {}", e).into())
}

pub fn write_vm_config(domain: &Domain, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let xml_string = to_string(domain)?;

    let mut file = File::create(path)?;
    file.write_all(xml_string.as_bytes())?;
    println!("Wrote {} success !", path);

    Ok(())
}
