fn main(){
 println!("This code is only about functions"); another_fn(2,3);


let sp = { // scopes is like objects in javascript and interface like in typescript let x = 3; // this is 
   let x = 6;	//statement it must closed by semicolon (;). x + 4 
    x + 4	// this is expresstion it must be without semicolon.
};
println!("{:?}",sp); // for printing the values of scope/objects we need to write :? in curly brackets like this {:?}

  
let result = subtraction_fun(3,4); println!("subtraction value is: {}",result);


let closure = |a:i32,b:i32| a + b; 
println!("the closure sum value is {}", closure(4,16));


}


fn another_fn(x:i32,y:i32){ println!("Addition of above params is {} + {} : {result}",x,y,result = x + y);
}


//closure method
fn subtraction_fun(x:i32, y:i32) -> i32 { x - y
}


