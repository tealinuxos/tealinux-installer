use std::fs::File;
use std::io::BufReader;
use crate::installer::BluePrint;
use std::io::Error;

pub fn read_blueprint() -> Result<BluePrint, Error>
{
    let file = File::open("/opt/tea-installer/installer.json")?;
    let reader = BufReader::new(file);

    let blueprint: BluePrint = serde_json::from_reader(reader)?;

    Ok(blueprint)
}
