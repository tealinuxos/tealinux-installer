use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DesktopEnvironment
{
    pub name: String
}

impl DesktopEnvironment
{
    pub fn new() -> Self
    {
        let env = std::env::var("XDG_CURRENT_DESKTOP");

        if let Ok(de) = env
        {
            if de.is_empty()
            {
                Self { name: String::from("Unknown") }
            }
            else
            {
                Self { name: de }
            }
        }
        else
        {
            Self { name: String::from("Unknown") }
        }
    }
}

