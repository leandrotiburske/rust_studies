// create a function to append a value to the beginning and to the end of a vector
fn append_vector(main_vec: &mut Vec<i32>, value: i32) -> Vec<i32> {
    main_vec.push(value); // push the value to the end of the vector
    main_vec.insert(0, value); // append a value as the first element of the vector
    main_vec.to_vec() // convert to vector
}

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    v = append_vector(&mut v, 90);
    println!("{:?}", v)
}