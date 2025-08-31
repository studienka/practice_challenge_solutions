//Build a function that returns an array of integers from n to 1 where n>0.

//Example : n=5 --> [5,4,3,2,1]

fn reverse_seq(n: u32) -> Vec<u32> {
    //creating a mutable empty vector
    let mut numbers = vec![];
    //iterating between 1 and n in reverse order
    for i in (1..=n).rev() {
        // if i is larger than 0 add it to the vector 
        if i > 0 {
            numbers.push(i);
        }
    }
    return numbers
}