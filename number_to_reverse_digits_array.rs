//Given a random non-negative number, return the digits of this number within an array in reverse order.
//function taking a u64 input n and producing a u8 vector as output
fn digitize(mut n: u64) -> Vec<u8> {
    let mut digit_vector = vec![];
    
    if n == 0 {
        return vec![0]
    }
    
    while n > 0 {
        //arithmetic digitization formula
        digit_vector.push((n % 10) as u8); 
        n /= 10;                      
    }
    digit_vector
}