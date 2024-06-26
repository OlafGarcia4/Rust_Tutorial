# Enums and Mathcing

Like in many other langugaes we can define our own data types, in rust we do so by using enum structs
Enums are similar to how we define a struct except that thery can accept anytype of data and not just one 
type.

    enum IpAddrKing{
        v4,
        v6,
    }

We use the same  let var = enum::val notation to call and set the data with inour code. We can even extend the 
functionality by defining what type of data each value in the enum can fit 'v4(i32,i32,i32,i32), v6(String)'.

    enum mssg{
        Quit,
        Move{x:i32,y:i32},
        Write(String),
        ChngColor(i32,i32,i32),
    }

Enums are so versitily because we can fit any data type and this includes other structs of enum with in the structure.
We can see that her we define mssg as having 4 structs (Quit, Move, Write, ChngColor), of course we still need to define these. 
We can still use teh same impl blocks to further extend the enums with methods.

Enums are great for creating our own data types however we can also further extend it by using Rust "null".
In Rust the  Null type does not exist in the language, instead we use the Option<T> to define something as null.

    enum Option<T> {
        None,
        Some(T),
    }

Above we see how Option<T> is expressed, we see that when using this the data can be None or Some(T), this eliminates the guess workw e usually have with Null.
As we can specify if a variable is truly empty with None or if its waiting for some data to fill its place. This gives us more control over assigning a variable
We dont need to build the Option<T> struct everytime, instead we can call and assign them directly

    let some_var : i8 = Some(5);
    let no_var : Option<i32> = None;

Despite beign able to fill data into the Option<T> struct we can interact with it like we would normally with other variables.
So if we did let x :i8 =5; let sum = x + some_var, we would get an error as some_var is not of type i8 but instead some_var is of type
is of type Option<i8>. Thankfully the standar library in rust gives us other methods to extract the data from the Option<T> struct.

Match and control Flow
-
    #[derive(Debug)]
    enum USstate {
        Alabama,
        Alaska,
        ...
    }

    Enum Coin{
        Penny,
        Nickle,
        Dime,
        Quarter(USstate),
    }

    fn value_in_cents(coin : Coin) -> u8{
        match coin{
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter =>{
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

The match expression work like a switch statement would. It exhausts all cases set until it finds someting that matches or exists the function.
Match can also evaluate Option<T> 

    fn plus_one(x : Option<i32>)->Option<i32>{
        match x{
            None => None,
            Some(i) => Some(i + 1)
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

Match block are exhaustive, meaning that a the match will take the evaluted value through all the options aveliable in the block, this mean that if we are evaluating 
struct. So if we use a match case to evluate a Option<T> we can not just evaluate for Some(T) but also have a case for the None option, rust will trhow an error because
this is not handeled.

**The _ placeholder** 

Sometimes we need to just evaluate certain values without much care for the other values. Using the _ option we can catch all other patterns to evaluate. 

    let dice_rolled = 9;
    match dice_rolled{
        3 = println("You got a 3")
        _ = (),
    }

We can see that the _ catches all other possible values for dice_rolled with () just being an empty placeholder.

**If let cases**

The above code can be condese like much other matches where we are only concerned with evaluating only 1 value. This allows us to condense match cases.

    match config_max {
        Some(max)=>println("this is the maximum {}", max);
        _ = (),
    }
    to 
    if let Some(max)= config_max{
        println("this is the maximum {}", max);
    }
    else{
        count += 1;
    }

While this can be beneficial as it eliminates some boiler plate code we also lose the exhaustiveness provided by match. Think of the if let syntax as a way
to check for a single value and the match syntax as a way to check mulitple values. We can also add a else case to the if let syntax to provide a functionality for if the 
the value does not match what we are looking for.

