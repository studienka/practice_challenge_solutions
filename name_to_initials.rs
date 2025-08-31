//take a first and last name divided by a single whitespace as a string and return uppercase initials divided by a "."
fn abbrev_name(name: &str) -> String {
    if let Some((first, second)) = name.split_once(' ') { //split at first whitespace into 2
        // get the first letter of each part as an uppercase string
        let firstletter = first.chars().next().expect("REASON").to_uppercase().to_string(); 
        let secondletter = second.chars().next().expect("REASON").to_uppercase().to_string();
        //return with correct formatting
        return format!("{}.{}", firstletter, secondletter);       
    }
    //incorrect input returns empty string
    String::new()