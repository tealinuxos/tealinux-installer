use serde::{ Serialize, Deserialize };
use serde_json::Value;
use crate::utils::command::command_with_output;
use duct::cmd;
use procfs::{ CpuInfo, Current };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Pci
{
    cpu: String,
    vga: Vec<String>
}

impl Pci
{
    pub fn new() -> Self
    {
        let cpu: String = if let Some(cpu) = Self::get_cpuinfo() {
            cpu
        }
        else {
            "Unknown".to_owned()
        };

        let vga = {

            let lshw = cmd!("lshw", "-json", "-class", "display")
                .stderr_null()
                .read()
                .unwrap();

            let json: Value = serde_json::from_str(&lshw).unwrap();

            let mut vec: Vec<String> = Vec::new();

            let mut i = 0;

            while !json[i]["product"].is_null()
            {
                vec.push(json[i]["product"].as_str().unwrap().to_string());
                i += 1;
            }

            vec
        };

        Self { cpu, vga }
    }

    fn get_cpuinfo() -> Option<String>
    {
        let current = CpuInfo::current();

        match current
        {
            Ok(cpu) => {
                let model_name = cpu.model_name(0);
                let model_name = model_name.map(|s| s.to_string());

                model_name
            },
            Err(_) => {
                Some("Unknown".to_owned())
            }
        }
    }
}
