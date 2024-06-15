use serde::{ Serialize, Deserialize };
use battery::Manager;
use battery::units::ratio::percent;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Battery
{
    capacity: Option<u32>
}

impl Battery
{
    pub fn new() -> Self
    {
        let battery = Manager::new().expect("failed to create a battery manager");

        let mut batteries = battery.batteries().expect("failed to get batteries");

        let capacity: Option<u32> = match batteries.next()
        {
            Some(battery) => {

                match battery
                {
                    Ok(bat) => {
                        let percentage = bat.state_of_charge().get::<percent>();
                        let percentage = percentage.floor();

                        Some(percentage as u32)
                    }

                    Err(_) => None
                }
            },

            None => None
        };


        Self { capacity }
    }
}
