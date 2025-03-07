// fn main() {
//     let name = String::from("parth");
    
//     let (ans , name) = get_len(name);

//     println!("{}",ans);
//     println!("{}",name); //this passes ownership error as only one can be owner but this passes toh danggling pointer error toh nhi ayega idhr

// }

// fn get_len(s:String)->(usize,String){
//     return (s.len(),s);
// }
//this is an ugly way of writing passing ownership rules 







//this was an ugly was to remove error we can do the same by borrow rather then giving ownership give borrow(refrence) to value
//we can borrow the varrible by not just passing value just passing refrence to that value like

// fn main(){
//     let name = String::from("Parth");
//     let length = get_len(&name); // this is passing refrence to varrible not giving ownership to varrible or borrowing varrible

//     println!("{} {}" ,name,length);

// }
// fn get_len(str:&String)->usize{
//     return str.len();
// }










// when you pass varrible by refrence varrible is still owned by the first function but it is only bowwred by ref

//browwing also have some rules - 
//there can be only one mutable refrence , after that we cannot have any mutable or immutable refrence
// there can be multiple imutable refrences

//eg of mutable and immutable refrences 
// fn main(){

//     //immutabe
//     // let name = String::from("Parth");
//     // name.push_str("Bandwal");

//     //immutable refrnces
//     // let s2  = &name;
//     // let s3  = &name;
//     // let s4  = &name;
//     // let s5  = &name;

//     // //we can have multiple immutable refrences but we can have only one mutable refrence 
//     // println!(" {} {} {} {} ",name,s2,s3,s4);


//     //mutable refrence 
//     let mut name1 = String::from("Ramesh");
//     let ref1 = &mut name1;
//     // let s3 = &mut name1; // this cannot be done we can only have one mutable refrences in rust
//     ref1.push_str(" Lodwal");
//     println!("{}",ref1);


// }





// // now if mutable refrence is not used beyond a certain point then we can have immutable refrences
// fn main(){
//     let mut name = String::from("Parth");
//     let s2 = &mut name;
//     s2.push_str(" Bandwal");

//     //--------------> after this mutable ref not used so then we can create any no. of immutable refs
//     let s4 = &name;
//     let s5 = &name;
//     // print!("{}" , s2); //this create a compilation error for all
//     let s7 = &name;

//     println!("{} {} {} " , s4,s5,s7); //full name gets printed as we are not using that mutable value if we have used it above this print statement then this all immutable refrences throws error

// }




//struct which fairly similar to class/object in javascript
// struct Rect{
//     length : f32,
//     breadth : f32
// }

// impl Rect {
//     fn area(&self)->f32{ // doing this it gets access to varribles in struct
//         return self.breadth *self.breadth
//     }
//     fn perimeter(&self)->f32{
//         return 2.0*(self.length+self.breadth);
//     }
// }
// fn main(){
//     let r1 = Rect{
//         length :10.4,
//         breadth: 14.4
//     };

//     println!(" area is : {} perimeter is : {} ",r1.area() , r1.perimeter());
//     // let area = r1.length*r1.breadth;
//     // println!("lenght: {} Breadth : {} Area of rectangle is {}", r1.length , r1.breadth , area);
// }




//enums
//annything which is meant to one of some values then it is better to define a enum


// enum Direction {
//     north,
//     south,
//     east,
//     west
// }

// fn main(){
//     let direction = Direction::east;

//     steer(direction);
// }
// fn steer(dir : Direction){ //this passing enum as input to function

//     match dir {
//         Direction::north => println!("north direction") ,
//         Direction::south => println!("south direction"),
//         _ => println!("horizontal direction") //for rest directions it runs this
//     }

// }




// Enum can also store values 

// use std::{f32::consts::PI, iter::Map};

// enum Shape {
//     circle(f32),
//     square(f32),
//     rectangle(f32,f32)
// }

// impl Shape{
//     fn area(&self)-> f32{
//         return match self {
//             Shape::circle(x) => PI * x * x,
//             Shape::rectangle(x,y ) => x*y,
//             Shape::square(x)=> x*x
            
//         }
    
//     }
// }

// fn main(){
//     let Circle = Shape::circle(10.4);
//     let Square = Shape::square(22.3);
//     let rect = Shape::rectangle(10.4,11.4);
//     println!(" {} ", Circle.area());
//     area(Shape::rectangle(4.3,11.4, ));


// }

// //write a fn that take shape as input and print area
// fn area(shape:Shape){
//     match shape {
//         Shape::circle(x) => print!("{}",3.14 * x*x),
//         Shape::rectangle(x, y) => print!("Area of reactangle is {} " , x*y),
//         Shape::square(x) => print!(" Area of Square is : {}"  , x*x),
//     }

// }

use std::{fs, ops::Index};

// enum Option{
//     None,
//     Some(u32)
// }


// error handling in enums
fn main(){
    let contents =  fs::read_to_string("a.txt");

    let ans :Option<u32> = find_first_a(String::from("hdfbjsdjakjsd"));


    match contents {
        Ok(contents) => println!("{}", contents),
        Err(e)=> println!("error while reading file")
    }

    match ans {
        None => print!("value not found"),
        Some(val) => println!("a found at index :{} ", val)
    }
}




//Option Enum to handle concept of nullablity in safe and simple way null varrible 
// this helps in genrating null varrible
fn find_first_a(s:String)->Option<u32>{
    let mut index = 0;
    for c in s.chars(){
        if c =='a'{
            return Some(index);
        }
        index = index+1;
    }

    None
}
