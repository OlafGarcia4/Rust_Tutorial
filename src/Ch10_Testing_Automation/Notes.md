# Test Automation
    #[cfg(test)]
    mod test {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }

Test are a quick way to check for errors while developing. Any test that we create will be defined witht the tag ***#[test]***.
We can execute these test by using:

    $ cago test 

Each of these test will run on its own thread, and its decalred failed if the thread dies. The first bit of code is the default test created by calling 'cargo new project --lib'. If we want to test out code, we will need to bring it into scope. So if we have a stuct Rectangle with a width and height and a implementation method of can_hold(&self, other: &Rectangle)-> bool, this is how we would write its tests.

    #[cfg(test)]
    mod tests {
        use super::*; // this is if test module exists with in our modeule otherwise we need to specify the path

        #[test]
        fn can_hold_rec(){
            let large =  Rectangle{width:8, height:7};
            let small = Rectangle{width:5, height:2};

            assert!(large.can_hold(&small));
            //assert!(!small.can_hold(&large));

        }
    }

As we can see the assert check for boolean to test the method can_hold. Similarly we can also use assert_eq!:

    assert_eq!( left , right);
    

Assert will compare the left and right arguments, in which one will be the expected result and the other will be the result that is returned.  It is important to note that when we compare the values using any assert macro that the data types being passed implement both the partial equal and debug traits. The assert macro also comes with a ***assert_ne!()*** which will take 2 arguments and check that they are not equal.

To create our own failure messages using equal we just define the failure message as a parameter and in the assesrt call:

    assert(something, "Something did not exists {}", param)

As we can see, the assert call will first test for something and if it fails then it will return the error message, any other parameters define after wont be tested and can be used on as part of the message.

We can also test for error from panic as well as for retruning Results.

    #[test]
    #[should panic]
    fn function_one(){...}

    #[test]
    fn function_two()-> Result<(), String>{
        if (condition){
            Ok( () );
        }else{
            Err(string)
        }
    }

The function_one will test for if the function has ended with a problem. This is convinient to check for the error case we create, however, with the current definition, the test will pass for any error that ends with a panic. To test for specific errors we need to take into account how we define the error cases on the function to test and then specify for the error message we expect to get:

    fn func_panic(){
        let a = ...
        let b = ...

        if a > b {
            panic!("A is bigger than B: {}", a)
        }
        else{
            panic("B is bigger than A: {}", b)
        }
    }
    //....

    #[test]
    #[should panic](expected ="B is bigger than A:" )
    fn function_one(){...}

By defining the expected return error message on the attribute tag we can pin point the error we want to test. To test for Result type we just set up the same syntacx we have seen for Result, where we return the Result type with () or we return a message if it does not pass the condition.(its important to note that any message to retrun won't be printed to screen but it will captured in the test binary)

To control the tests we have a couple of options. First we have the flags that are in test comand (--) and the flags for the resulting test binary (-- --). 

    cargo test --help       //command options
    cargo test -- --help    //binary options

These will show us ways we can set up the behavior from the tests. For example we can set the amount of threads to use which can help as for example testing test that edit a file will benifit from running on 1 thread as the file will be changed by each test. 

If we have a suite of tests we can choose what test to run using 
    
    $ cargo test name_of_test

    //...
    #[test]
    fn add_one(){...}
    #[test]
    fn add_many(){...}
    #[test]
    fn multiple_add(){...}

In this case if we want to run only multiple_add we would do ***cargo test multiple_add***, this will work for test that share some of the name convention, so if we want to run the first 2 test then we just need to say ***cargo test add***. This will run any test that begin with 'add' on its declaration. We can also ignore test by adding a ***#[ignore]*** attribute tag, then we can run these test by using this test binary option
    
    cargo test -- --ingored
This will run only the test cases that have the attribute, this can help decrease the time on testing as we can assing the tag to test that are more costly to test.

So far we have disscussed unit test. These are define by test that live in the same file as the code it tests, run when we use 'cargo test'. It is common to have test live in the same file as the code but if we wanted to move these test to another file we will need to use intgration test:

    root
        -src
            -main
        -tests
            - common
                - mod.rs
            -Integration_test.rs

We first need to creat a folder called tests on the root directory of our project, after we can create the intergation test file where our test will live in. Each of these test file will be converted to creates by Rust, meaning that we will need to bring objects into the scopes of the project. We do not need to declare the test with the ***#[test]*** attribute as rust knows that these are tests.

Beacuse Rust converts any file under tests/ to a crate this can be a issue when we want to share some data across multiple test files. To solve this we can create a directory called common with a file mod.rs. This will prevent the common/mod.rs file to be converted to a crate since files in directories in test wont be converted. We can then go to our test file and define 'mod common' to use the data within.


