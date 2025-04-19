
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


// //slice
// fn main(){
//   let s:String = String::from("hanji kya haal chaal");

//   let _s1: &str = &s[0..5];
//   let _s2: &str = &s[6..10];
//   let _s3: &str = &s[11..16];
//   let _s4: &str = &s[17..22];
//   println!("{}", _s2);


//   // String literal, known at compile time
//   let x: &str = "hello world";

//   // Dynamic string
//   let hello: String = String::from("hello world");
//   // String slice, references the entire string
//   let y: &str = &hello[..];
//   // String slice, references a part of the string
//   let z: &str = &hello[0..3];


// }


//dangling refernce error
// fn main(){
//   let x;
//   {
//     let y:i32 = 10;
//     x = &y;
//   }
//   println!("{}", x)
// }








//========================================================================================================================


//declarative macro 

// macro_rules! double{
//   ($x:expr) => {
//     $x * 2
//   }
// }

// fn to_uppercase(s: &str) -> String{
//   s.to_uppercase()
// }

// macro_rules! uppercase{
//   ($s:expr) => {
//     $s.to_uppercase()
//   }
// }

// fn main() {
//  let x:i32 = 10;
//   let y:&str = "I am te Best";
//   println!("{}", double!(x));
//   println!("{}", to_uppercase(y));
//   println!("{}", uppercase!(y));
// }




// macro_rules! add{
//   // Match a single parameter
//   ($a:expr)=>{
//      $a
//   };
//  // Match 2 parameters
//   ($a:expr,$b:expr)=>{
//      {
//          $a+$b
//      }
//   };
//  // Recursive call
//   ($a:expr,$($b:tt)*)=>{
//       {
//           $a+add!($($b)*)
//       }
//   }
// }

// fn main() {
//   println!("{}", add!(1, 2, 3, 4));
// }

// The repeated token type is enclosed in $() and is followed by a * or + indicating the number of times the token will berepeated. $($b:tt)* denotes parameters of type tt that can be repeated 0 to N times. The add!($($b)*) statement recursively calls the add! macro, achieving the capability of handling a non-fixed number of parameters.





//========================================================================================================================

//procedural macro ==> derive macro
// // This is a generic trait
// trait HelloMacro {
//     fn hello_macro();
// }

// // Custom struct MyStruct, implementing the above trait
// struct MyStruct;
// impl HelloMacro for MyStruct {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is MyStruct!");
//     }
// }

// // Custom struct YourStruct, implementing the above trait
// struct YourStruct;
// impl HelloMacro for YourStruct {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is YourStruct!");
//     }
// }

// fn main() {
//     MyStruct::hello_macro();
//     YourStruct::hello_macro();
// }
//========================================================================================================================

// //procedural macro ==> function like macro
// vec! Macro for creating Vec.

// fn main(){
// let my_vector = vec![1, 2, 3];

// // println! and format! macros for formatting strings.
// let name = "World";
// println!("Hello, {}!", name);

// let formatted_string = format!("Hello, {}!", name);

// // assert! and assert_eq! macros for writing assertions.
// assert!(true);
// assert_eq!(2 + 2, 4);


// // panic! Macro used to cause Panic exceptions in the program.
// panic!("Something went wrong!");

// // env! Macro for obtaining environment variables at compile time.
// let current_user = env!("USER");
// println!("Current user: {}", current_user);


// // declare_id! is a macro used in the anchor framework to declare program IDs
// declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");
// }
// //========================================================================================================================


// struct User{
//   username:String,
//   password:String
// }

// impl User{
//   deserialize_from_str(){
  
//   }
//   to_string(){}
// }


//Serde
// a popular library for serializing and deserializing data in Rust. It allows you to easily convert between Rust data structures and various data formats like JSON, YAML, and more.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct User{
  username:String,
  password:String
}

fn main(){
  let u: User = User{
    username:String::from("Anurag"),
    password:String::from("12345")
  };
  //now we can use serde to convert this struct to json or string without having to implement the to_string method

  let serialized_string = serde_json::to_string(&u);
  match serialized_string {
    
    Ok(s) => println!("{}", s),
    _err => println!("Error")
  }
  
}

//========================================================================================================================
