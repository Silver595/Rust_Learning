fn main(){
let array1 = [1,3,4,6,7];
let array2 = array1;
println!("{:?}",(array1,array2));

let vector1 = vec![1,2,3,5];
let vector2 = &vector1;
println!("{:?}",(&vector1,vector2));
}
