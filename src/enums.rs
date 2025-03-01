enum Color {
Yellow,
Red,
Green,
Blue,
}

impl Color {
 fn green_part(&self) -> bool {
    match self {
	Color::Yellow => true,
	Color::Blue => true,
	_ => false,
  
}
}

fn is_green(&self) -> bool{
 if let Color::Green = self {

	return true;		
} 
	return false;
}

}

fn print_color(color:Color){
match color{
  Color::Red => println!("red"),
  Color::Green => println!("Green"),
  Color::Blue => println!("Blue"),
  Color::Yellow => println!("Yellow")
}
}


