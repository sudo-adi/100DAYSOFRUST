fn main() {

#[allow(unused)] // this will allow the compiler to ignore the warnings for unused variables

{

// Defning a variable   

let name = "Rust";  // Datatype automatically inferred by compiler to String Data type "&str"
let age = 20; // Datatype automatically inferred by compiler to int 32 bit Data type "i32"
let marks = 95.5; // Datatype automatically inferred by compiler to float 64 bit Data type "f64"
let is_rust_awesome = true; // Datatype automatically inferred by compiler to boolean Data type "bool"
let char = 'A'; // Datatype automatically inferred by compiler to char Data type "char"


// 1 Scaler Data Sturectures.



// 1.1 Integer Types
let i8_num: i8 = 42; // Declares a signed 8-bit integer variable
let u8_num: u8 = 42; // Declares an unsigned 8-bit integer variable

// ... (similar explanations for other integer types)


// 1.2 Floating-Point Types
let f32_num: f32 = 42.42; // Declares a 32-bit floating-point variable
let f64_num: f64 = 42.42; // Declares a 64-bit floating-point variable


// 1.3 Boolean Type

let bool_true: bool = true; // Declares a boolean variable with value true


// 1.4 Character Type

let char_a: char = 'a'; // Declares a character variable with value 'a'


// 1.5 The Unit Type

let unit: () = (); // Declares a variable of type unit



// Compound Data Structures.



// 2.1 Tuple Type
let tup: (i32, f64, char) = (500, 6.4, 'A'); // Declares a tuple with elements of different types


// 2.2 Array Type
let arr = [1, 2, 3, 4, 5]; // Declares an array with elements of the same type


// 2.3 String Type
let str_literal = "Hello World!"; // Declares a string literal

let name: String = "Rust".to_string(); // Datatype explicitly defined to String Data type "String"



// 2.4 Slice Type
let slice = &str_literal[0..5]; // Declares a slice of a string


// 2.5 Pointer Type
let ptr = &slice; // Declares a reference to a slice


// 2.6 Reference Type
let reference = &ptr; // Declares a reference to a pointer


// 3 Other Data Types (not explicitly categorized as scalar or compound)


// 3.1 Function Type
fn func() {
    println!("Hello World!");
} // Declares a function


// 3.2 Closure Type
let closure = || println!("Hello World!"); // Declares an anonymous function (closure)


// 3.3 Struct Type
struct MyStruct {
    name: String,
    age: i32,
}

let my_struct_instance = MyStruct {
    name: String::from("John"),
    age: 30,
}; // Declares a struct and an instance of it


// 3.4 Enum Type
enum MyEnum {
    A,
    B,
    C,
}

let my_enum_instance = MyEnum::B; // Declares an enum and an instance of it


// 3.4 Union Type
union MyUnion {
    a: i32,
    b: f64,
} // Declares a union


// 3.5 Trait Type
trait MyTrait {
    fn func();
} // Declares a trait


// 3.6 Module Type
mod my_module {
    pub fn func() {
        println!("Hello World!");
    }
} // Declares a module and a function within it


// 3.7 Type Alias Type
type MyType = i32; // Declares a type alias


// 3.8 Never Type
fn never() -> ! {
    panic!("This function never returns!");
} // Declares a function that never returns


// 3.9 Abstract Type
impl MyTrait for MyStruct {
    fn func() {
        println!("Hello World!");
    }
} // Declares a trait implementation for a struct

// ... (similar explanations for other types)


// 3.10 Static Type
static MY_STATIC: i32 = 0; // Declares a static variable


// 3.11 Macro Type
macro_rules! my_macro {
    () => {
        println!("Hello World!");
    };
} // Declares a macro


// 3.12 Pin Type
let my_pin = Box::pin(0); // Declares a pinned value



// Variable Shadowing

let x = 5; // Declares a variable x with value 5
let x = x + 1; // Declares a new variable x with value 6, shadows the previous value of x
let x = x * 2; // Declares a new variable x with value 12, shadows the previous value of x
println!("The value of x is: {}", x); // Prints "The value of x is: 12"


// Mutable Variables

let mut x = 5; // Declares a mutable variable x with value 5
x = x + 1; // Modifies the value of x to 6
x = x * 2; // Modifies the value of x to 12
println!("The value of x is: {}", x); // Prints "The value of x is: 12"


// Constants

const MAX_POINTS: u32 = 100_000; // Declares a constant with value 100000


// Shadowing vs. Mutability vs. Constants

let x = 5; // Declares a variable x with value 5
let x = x + 1; // Declares a new variable x with value 6, shadows the previous value of x
const MAX_NUM: u32 = 100_000; // Declares a constant with value 100000
let mut y = 5; // Declares a mutable variable x with value 5
y = y + 1; // Modifies the value of x to 6
// MAX_NUM = MAX_NUM + 1; // This will cause a compiler error
println!("The value of x is: {}", x); // Prints "The value of x is: 6"
println!("The value of y is: {}", y); // Prints "The value of y is: 6"


}
}