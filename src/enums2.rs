struct Custom{
name:String,
age:usize,
}

enum Item{
 Foo(String),
 Bar(usize),
 Baz(Custom),	

}

let foo = Item::Foo(String::from("hello"));

if let Item::Foo(s) = foo{
 println!{"{}",s};
}

let bar = Item::Bar(usize::from(42));
if let Item::Bar(b) = bar {
println!{"{}",b}
}

let baz = Item::Baz(Custom {name:String::from("Deago",age:34)});
if let Item::Baz(ba) = baz{
println!("{}",ba.age);
}

fn append(arr:&mut Vec<Item>){
 arr.push(Item::Bar(86))

}


//let mut arr = vec!["hello","bro"];
//append(&mut arr)

// this is not work in rust it only work in typescipt 
