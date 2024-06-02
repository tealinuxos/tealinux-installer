use duct::cmd;

pub fn command_with_output(commands: String) -> String
{
    let vec_args: Vec<String> = commands
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    cmd(&vec_args[0], &vec_args[1..]).read().unwrap()
}

pub fn command_with_status(commands: String) -> bool
{
    let vec_args: Vec<String> = commands
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let status = cmd(&vec_args[0], &vec_args[1..])
        // .stdout_null()
        // .stderr_null()
        // .unchecked()
        .run()
        .unwrap();

    if let true = status.status.success()
    {
        return true;
    }

    false
}

pub fn command_with_output_to_file(commands: String, file: std::fs::File) -> bool
{
    let vec_args: Vec<String> = commands
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let command = cmd(&vec_args[0], &vec_args[1..]).stdout_file(file).run().unwrap();

    command.status.success()
}

pub fn command_with_input(commands: String, input: String) -> bool
{
    let vec_args: Vec<String> = commands
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let command = cmd(&vec_args[0], &vec_args[1..]).stdin_bytes(input).run().unwrap();

    command.status.success()
}
