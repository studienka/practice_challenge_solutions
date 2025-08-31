//Take an vector of integers and return a vector of two integer values, the count of positive numbers and sum of all negative values.
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    //if empty input return empty output
    if input.is_empty() {
        return vec![]
    }
    //declaring mutable variables to store the count/sum
    let mut pos_count = 0;
    let mut neg_sum = 0;
    
    for x in input {
        if x > 0 {
            pos_count += 1;
        }
        else if x < 0 {
            neg_sum += x;
        }
    }
    vec![pos_count, neg_sum]
}