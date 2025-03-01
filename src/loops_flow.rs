fn main(){
//simple_loop();

//while_loop();

for_loop();

//itr_loop();
}

// Simple loop 

fn _simple_loop(){
let mut count = 0;

loop{
println!("count is: {}",count);
count += 1;
if count == 10{
	break;
}}
}
// While loop

fn _while_loop(){
let mut count = 15;
while count <= 100{
  if count % 15 == 0{
	println!("Fizzbuzz");	
  }
  else if count % 3 == 0{
  	println!("Fizz");
  }
  else if count % 5 == 0{
  	println!("Buzz");
  }
  else{
 	println!("Invalid input"); 
 }
	count +=1;
}}

// for loop

fn for_loop(){
for x in 0..1000000{
	println!("number are {} ",x);
}
}

// Iterating loop

fn _itr_loop(){
let a = [10,28,25,26,72,82,52,28,26,5,37];
for element in a.iter(){
	println!("value is {}",element);
}
}
