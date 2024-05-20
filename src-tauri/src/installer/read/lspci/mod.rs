pub fn lspci() -> Vec<String>
{
    let mut vec: Vec<String> = Vec::new();

    vec.push(String::from("Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz"));
    vec.push(String::from("UHD Graphics 620"));

    vec
}
