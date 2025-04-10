
// trait Shape{
//   fn area(&self) -> f32;
// }

// struct Circle{
//   radius: f32
// }

// struct Reactangle{
//   width: u32,
//   height: u32
// }

// impl Shape for Circle{
//   fn area(&self) -> f32{
//     3.14 *( self.radius * self.radius)
//   }
// }


// fn get_area(aaakaar: impl Shape) -> f32{
//     return aaakaar.area()
// }

// fn main() {
//     let c:Circle = Circle{
//         radius: 3.0
//     };

//   let r:Reactangle = Rectangle{
//     width:3,
//     height: 4
//   };
//     get_area(c);
  
// }


//traits are similar to interfaces in java/javascript


//Macros are a way of writing code that writes other code, which is known as metaprogramming.
// #[derive(Debug)]

// struct User{
//   name: String,
//   age: u32,
// }


// fn main() {
//   let u:User = User{
//     name: String::from("Anurag"),
//     age: 21
//   };

//   println!("{:?}", u.name);
//   println!("{:?}", u.age);
//   println!("{:?}", u);
// }


//========================================================================================================================

//borrowing and references
// fn main(){
//   let x:String = String::from("hanji");

//   let y:&String = &x;

//   println!("x ={},y = {}", x, *y)
// }

//ownership
// fn main(){
//   let x:String = String::from("hanji");

//   let y:String = x;
//  //cannot use x after this point
//   // println!("{}", x)
//   println!("{}", y)
  
// }



//========================================================================================================================


//slice
fn main(){
  let s:String = String::from("hanji kya haal chaal");

  let _s1: &str = &s[0..5];
  let _s2: &str = &s[6..10];
  let _s3: &str = &s[11..16];
  let _s4: &str = &s[17..22];
  println!("{}", _s2);


  // String literal, known at compile time
  let x: &str = "hello world";

  // Dynamic string
  let hello: String = String::from("hello world");
  // String slice, references the entire string
  let y: &str = &hello[..];
  // String slice, references a part of the string
  let z: &str = &hello[0..3];


}






















//========================================================================================================================

//========================================================================================================================

//========================================================================================================================

//========================================================================================================================

//========================================================================================================================
