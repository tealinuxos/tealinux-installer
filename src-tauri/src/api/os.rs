use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Os
{
    pub os_type: String,
    pub name: String,
    pub path: String
}

pub fn get_other_os() -> Option<Vec<Os>>
{
    let mut results: Vec<Os> = Vec::new();

    let prober = duct::cmd!("os-prober").read();

    let output = match prober
    {
        Ok(prober) => Some(prober),
        Err(_) => None
    };

    let output: Option<Vec<String>> = if let Some(o) = output
    {
        Some(o.split("\n").map(|s| s.to_string()).collect())
    }
    else
    {
        None
    };

    if let Some(outputs) = output
    {
        let re = Regex::new(r"^(/dev/\S+?)@?:([^:]+):([^:]+)").unwrap();

        for i in outputs
        {
            if let Some(entry) = re.captures(&i)
            {
                results.push(
                    Os {
                        path: entry[1].to_string(),
                        name: entry[2].to_string(),
                        os_type: entry[3].to_string()
                    }
                );
            }
        }
    }

    if results.is_empty()
    {
        None
    }
    else
    {
        Some(results)
    }
}
