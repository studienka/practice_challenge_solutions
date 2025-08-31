//remove the first and the last character of a string
pub fn remove_char(s: &str) -> String {
    //if the string has less than 2 characters, return a new empty string
    if s.chars().count() < 2 {
        return String::new()    //return a new empty string and exit the program
    }
    //otherwise run the code below
    let mut characters = s.chars();
    characters.next();       // drop first
    characters.next_back();  // drop last
    characters.collect()
}