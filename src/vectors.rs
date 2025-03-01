fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 4, 5, 24, 7];
    let _vector1: Vec<i32> = Vec::new();

    vector[0] = 20;
    println!("{:?}", vector); // Output: [20, 2, 4, 5, 24, 7]

    println!("{}", vector[1]); // Output: 2

    println!("{}", vector.len()); // Output: 6

    vector.push(10);
    println!("{:?}", vector); // Output: [20, 2, 4, 5, 24, 7, 10]

    vector.pop();
    println!("{:?}", vector); // Output: [20, 2, 4, 5, 24, 7]

    // iterating each element in vector
    for element in vector.iter() {
        println!("Element is {}", element);
    }

    // in below case it is mutable iteration (iter_mut())
    for item in vector.iter_mut() {
        *item += 10; // --> * is used to dereference the mutable reference and modify the value it points to
        println!("{}", item);
    }
}
