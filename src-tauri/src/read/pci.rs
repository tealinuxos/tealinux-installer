use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Pci
{
    cpu: String,
    vga: String
}

impl Pci
{
    pub fn new(cpu: String, vga: String) -> Self
    {
        Self { cpu, vga }
    }
}
