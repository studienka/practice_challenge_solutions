//Take a string and if the first letter in 'r' or 'R', return a success message, otherwise return a fail message
fn are_you_playing_banjo(name: &str) -> String {
    let success_message = format!("{} plays banjo", name);
    let fail_message = format!("{} does not play banjo", name);
    //using pattern matching to determine the output
    match name.chars().next() {
        Some(c) if c.to_ascii_lowercase() == 'r' => success_message,
        Some(_) => fail_message,
        None => String::new(),
    }
}