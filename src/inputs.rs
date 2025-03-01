use std::io; //io its like Scanner in java
// :: it is a path separator 
fn main(){

let mut a = String::new(); // --> output of a is an empty string

println!("Enter the input: {}",a);

io::stdin() // -->stdio handles user input 
	  .read_line(&mut a) // --> take input and analyis it 
	  .expect("error to read input"); // --> if input failed in format /wrong input is entered then expect
					  //     this throws error as our choice

println!("This is the value of user input {}",a);
}
