# Error Handeling 

Errors are just a part of programming, Rust comiles these errors into recoverable and inrecoverable errors. Recoverable error(rec) are errors that we expect to report some message with out the programm,
ending. Unrecoverable errors (urec) are error where the programs stops, usually this is because of a bug in the program. So for this we have type Result<T, E> for rec. and panic! macro for urec.

Unrecoverable errors 
- 

We can use the panic macro by calling it directly or by our code calling it as it starts to unwind.
        
        panic!("crash and burn");

This will print an error message where we can  that the fisrt line will give us the message we told to print along with where in our code the call was made from. However not all the time we would be 
able to reference the location of the call as it could have been called by a part of our code and not us directly. Thankfully we can use backtrace to see what caused the panic! call.

Setting the enviroment varaible RUST_BACKTRACE=1 we can see the list of functions that our program has called. The first few lines consists of code that our code called, at the first instace related to the files
in our project, that is where the error first originated, below that point is code that we have called for our program. Set the enviroment variable to 0 to turn off RUST_BACKTRACE.

Recoverable error   
-
    
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        let greeting_file_result = File::open("hello.txt");

As metioned, recs are only used when we just want to be notified of some error without halting the program. This is achived with the type Result<T, E>.  Both T and E are generics with T representing the action that
need to be taken if there is a successful call and E will be the error that will be returned when we have an error.

If we ran the greeting_file_result declaration we see we dont get halting error, we actually dont get any, this is becuase we need to set up rec case using a match block 

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error)=> panic!("Problem opening file {:?}", error),
        } ;

Doing this will allows us to be notified of the error that is happening. Additionally we can add more cases to inform us of different errors,

        use std::io::ErrorKind;

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };

Using a second match case with the error and its kind we can account for diffrent errors with one success output. The ErrorKind::NotFound executes if the file we are
looking for does not exists, while the other_error with informs us if the file was not able to be opened, we are checking for the value returned from error.kind(). As we can see writing our own rec cases can get complicated, however we can use unwrap and and expect helper methods to make this easier.

We have previously used the unwrap and expect helper methods. The unwrap method takes the retruning type of Result<T,E>, unwrapping the information behind it, so if its Ok() then take T. 

        expect(message) is a way to easily retrun a message and call panic. 
            let greeting_file = File::open("hello.txt").unwrap();
            or
            let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
           
 We can also propagate errors to add more control over errors. We achive this by using match cases to retrun the error up to the calling function, for it to decide what to do. However, this can add
 a lot of biolerplate code, instead we can use the ? operator

            use std::fs::File;
            use std::io::{self, Read};

            fn read_username_from_file() -> Result<String, io::Error> {
                let mut username_file = File::open("hello.txt")?;
                let mut username = String::new();
                username_file.read_to_string(&mut username)?;
                Ok(username)
            }
        
        from :

            fn read_username_from_file() -> Result<String, io::Error> {
                let username_file_result = File::open("hello.txt");

                let mut username_file = match username_file_result {
                    Ok(file) => file,
                    Err(e) => return Err(e),
                };

                let mut username = String::new();

                match username_file.read_to_string(&mut username) {
                    Ok(_) => Ok(username),
                    Err(e) => Err(e),
                }
            }

    The ? is placed after the Result value, equal to the respective match cases on the function below. The ? retruns the value of result, weather that be the Ok value, or propagating the error by retruning it.
    It is important to know where we can use this operator. We can only use it if the function on which it lives will retrun a Result, Option, or other type that implements the trait FromResidual. So if we want to
    use ? on our main function, we will need to change main's return to Result<(), Box<dyn Error>>

        fn main() -> Result<(), Box<dyn Error>>{    // The Box<dyn Error> is a Trait Object, we'll disscuss more about this later
            ...
        }
    
So how can we decide when to use panic! and when to return Result?, well it depends. Having errors that clearly halt the program at the point of an error are great whe writing prototype code and even the methods for 
unwrap and expect can help with drafting error handeling. So we need to use panic calls as way to show an error has happened and unwrap and expect as a way to diagnose what out code is doing. 

So here are the general guides behind when to use panic!:

1. Panic if the code could end in bad state. 
2. A bad state is  when something unexpected happens outside of what is expected when using the code 
3. If after a bad state has been identified, the code needs to not be in this state.

If failure is expected then don't panic and instead retrun a Result. After all, we want to use type Results to properly handle errors we are expecting using propagation 
