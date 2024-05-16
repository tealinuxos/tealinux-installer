pub fn split_by(input: String, pattern: &str) -> Vec<String>
{
    let vec: Vec<String> = input
        .split(pattern)
        .map(|i| String::from(i))
        .collect();

    vec
}
