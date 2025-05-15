use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DisplayServer
{
    name: String
}

impl DisplayServer
{
    pub fn new() -> Self
    {
        let env = std::env::var("XDG_SESSION_TYPE");

        if let Ok(ds) = env
        {
            if ds.is_empty()
            {
                Self { name: String::from("Unknown") }
            }
            else
            {
                Self { name: ds }
            }
        }
        else
        {
            Self { name: String::from("Unknown") }
        }
    }
}

