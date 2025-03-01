use std::io;

fn main(){
// all control flow means if else conditions statementes 
// sample code for syantax

let mut a = String::new();
let mut b = String::new();

println!("Enter input for a: ");
io::stdin().read_line(&mut a).expect("Error");
println!("Enter input for b: ");
io::stdin().read_line(&mut b).expect("Error");

let a:i32 = a.trim().parse().expect("Error enter valid input");

let b:i32 = b.trim().parse().expect("Error enter valid input");

if a > b {
println!("nigga are you such a dumnass");
}
else if b > a {
println!("head are you such a nerd");
}
else if b == a {
println!("bro are you such a speaker");
}
else {
println!("yo yo yo bitch ");
}
short_form();
}

fn short_form(){

let mut age = String::new();
println!("Enter a valid true age: ");
io::stdin().read_line(&mut age).expect("their is an error to gain input try vaild no");

let age:i32 = age.trim().parse().expect("Enter valid input"); 
let teenager = if age > 8 && age < 18 {"yeah you are teenager"} else {"shut the fuck off"};
println!("{}",teenager);
}
  
