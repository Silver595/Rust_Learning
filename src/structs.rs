	// Traditional structs
struct Color{
    red:u8,
    green:u8,
    blue:u8,
 }
// Tuple struct

struct Color2(u8,u8,u8);

fn main(){
// Structs are similar as tuples but with more flexablity 

 //traditional structs 

  let mut color_option1 = Color{ // Color is a instance of struct Color
   red:0,
   green:255,
   blue:0,   
};

//changing values/properties

color_option1.blue = 200;

//access values 
println!("Color: {} , {} , {}", 
color_option1.red,
color_option1.blue,
color_option1.green,);


//Tuple Struct
let mut color_option2 = Color2(0,14,235);
println!("Color: {}, {} , {}",
color_option2.0,
color_option2.1,
color_option2.2,
);
}
