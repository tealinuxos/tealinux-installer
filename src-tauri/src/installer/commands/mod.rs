use duct::cmd;
use super::utils::split_by;

pub fn command_with_output(commands: String) -> String
{
    let mut vec_args = split_by(commands, " ");

    let output = cmd(&vec_args[0], &vec_args[1..]).read().unwrap();

    output
}

pub fn command_with_status(commands: String) -> bool
{
    let vec_args = split_by(commands, " ");

    let status = cmd(&vec_args[0], &vec_args[1..])
        .stdout_null()
        .stderr_null()
        .unchecked()
        .run()
        .unwrap();

    if let true = status.status.success()
    {
        return true;
    }

    false
}
