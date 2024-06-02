use serde::{ Serialize, Deserialize };
use online::check;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Online
{
    status: bool
}

impl Online
{
    pub fn new() -> Self
    {
        let status = check(None).is_ok();

        Self { status }
    }
}
