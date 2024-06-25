use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Payload
{
    pub percentage: u32,
    pub message: String
}
