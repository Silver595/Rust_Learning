use std::env;

fn main(){
let args: Vec<String> = env::args().collect();
let input = args[1].clone();

println!("{:?}",args);
println!("{}",input);

if input == "hello" {
	println!{"hey what's buddy! How are you"};
}
}
