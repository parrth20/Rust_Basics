// use core::str;
// use std::env;

// use chrono::prelude::*;//using this external crate 
// use dotenv::dotenv;

// fn main() {

//     let utc= Utc::now();
//     println!("{}", utc);
//     let local = Local::now();
//     print!("{}" ,local);


//     dotenv().ok(); //this reads all the envoirmnet varribles of dotenv file
    
//     let var = env::var("MONGO_URL"); //this return a result orr it can 
//     let unrap = var.unwrap(); // this unwarps is we got url it stores otherwise throws run time error and breaks the process


//     //write a option enum also

//     match var {
//         Ok(str) => print!("correct url found {}" , str),
//         Err(_e)=> print!("something is wrong")
//     }

// // none of this are needed for solana developmenet

// }


// // external pakages - crates in rust
// // cargo.toml is similar to pakage.json 
// // cargo add something to get what dependencies to be added in the file 
 








// genrics in rust
//if we want string and number both output from a function we canuse genrics so that we don't have to  make 2-2 function for same thing
//we have single function which has a genric intendation there

// struct display{
//     username : String
// }

// fn main(){
//     // print!("{}" , sum(1,3));
//     display_elements(3, 4);
//     display_elements(String::from("Paarht"), String::from(" Bandwal"));
//     let u1 = display{
//         username: String::from("Parth12345")
//     };
//     let u2 = display{
//         username: String::from("Bandwal2uytw7q")
//     };

//     // display_elements(u1, u2); // u1 and u2 doesn't follow trait therfore can't be passed under varrible
// }

// fn display_elements<T :std::fmt::Display>(a:T , b:T){ //this follows display trait which puts ristriction
//     //we can call this function only with varribles that has display traits
//     println!("{}", a);
//     println!("{}", b);
//      //this gives as an error cos genric can be any datatype but we cannot add 2 bool therofre we need to add here a trait bound to remove this error
//     print!("{}", add(3, 4));

// }


// fn add<T :std::ops::Add<Output = T >>( a : T , b : T )->T{ //the t varrible should be bound to a specific trait 
//     return a+b;
// }









//genrics over structs


// //define macros 
// #[derive(Clone, Copy)]
// struct rect<T>{  // sturcts can also have genrics advantage 2 alag alag stuct nhi likhne pade for u32 and f32
//     length : T,
//     widht : T
// }

// impl<T:std::ops::Mul<Output = T>+Copy> rect<T>{ //copy trait  this follows 2 traits mul trait and copy trait
//     fn area(&self)->T {
//         return self.length * self.widht;
//     }    
// }



// //enums also have genrics


// fn main(){

//     let r1 = rect{
//         widht : 10,
//         length : 10
//     };
//     let r2 = rect{
//         widht : 10.6,
//         length : 10.4
//     };

//     let ar = r1.area();
//     println!("{}", ar);


//     let x  =Some(23.32);

//     match x {
//         Some(var)=>println!("Some : {}" , var),
//         None=>print!("error in mesage")
//     }
// }

use std::f32::consts::PI;












// Traits
// specifc trait which a struct can follow simliar to interfaces in java/js
trait Shape {
    fn area(&self) -> f32 ;
}

struct rect {
    widht : f32,
    height : f32
}
struct circle {
    radi : f32
}

impl Shape for rect {
    fn area(&self) ->f32 {
        return self.height * self.widht;
    }
}
impl Shape for circle {
    fn area(&self) -> f32 {
        return self.radi*self.radi*PI;
    }
}

fn main(){
    let r1 = rect{
        height:32.4,
        widht:13.2
    };

    let c1 = circle {
        radi : 12337.2
    };

    let area1 = r1.area();
    let area2 = c1.area();
    println!("{} {} " , area1 ,area2);
}
