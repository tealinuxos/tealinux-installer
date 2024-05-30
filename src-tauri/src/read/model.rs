use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Model
{
    brand_name: String,
    model_number: String
}

impl Model
{
    pub fn new(brand_name: String, model_number: String) -> Self
    {
        Self { brand_name, model_number }
    }
}
