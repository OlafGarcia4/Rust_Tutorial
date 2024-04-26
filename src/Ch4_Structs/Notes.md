# Structs

Structs are the way we can create our own data types. This followes the same patterns as creating a a struct on C. 

    struct Foo {
        key: type,
        key2: type,
    }

    let mut bar = Foo{
            key: value1,
            key2: value2,
        };
     bar.key = value3;

Above we can see how we can create a struct, how we can initilize a instace of Foo ini bar, and how we can interact with
the data inside the struct. At the time of  creating a Struct we need to name the value and its type using a (key:val) notation. 
Wee do not need to declare the struct as a mutable value, this is done when we creat an instance. The same rules apply for instance of struct 
as they do for the all other value, the declaration of mutable will allow you to change the values.
We can also use functions to build the structs and return a filled structure.

    fn buildStruct(key : Type, key2: type )-> Foo {
        Foo{
            key,
            key2
        }
    }

As we can see we can Field Init shorthand, where the parameters of a function share the same name as the variables in the struct.
We can futher manipulate a struct by using the upadate syntax. Sometimes we jsut want to update a single data point with in the struct
especially if we acre creating a new instace by simply using '..[instancename]', where the first '.'  points all the values not that are
initilized. THis way we can pull information out of the first instace we created to fill out the values in the  struct that we did not
specied when we delcaread a second instance. 

    let example = Foo {
        key: val4,
        ..bar
    };

Structs, once declared become its own data type, that means that if we where to copy Foo and rename it as FooClone, Foo and FooClone will not 
be the same type despite sharing the same data types internally, so no function that takes Foo clone will not take Foo.

**Debug and Printing**

Often times we would like to have representation of the values with in a struct, however callilng the macro function of println! woudl not work on this case 
We need to the following, first we need to declared the struct as with the trait "#[derive(Debug)]" right before we begin to state out struct. At the time to 
using the println! function, we will need to also add the ':?' specifier when we pass the info forom pur struct.

    #[derive(Debug)]
    struct Rectangle {
        width: i32,
        height: i32,
    }

    fn main () {
        println!("{:?}", Rectangle);
    }

Simimlarly we can increase the functionality of the structs by using methods. Methods in rust work the same as in other languages, just that in this case we place
them on the impl block. The impl block will contain methods that we want the stuct to be associated with. Once the we have the methods we can use the '.mehtod(param)'
to call any method.

    impl Rectangle{
        fn area(&self)-> i32 {...}
        fn perimeter(&self)-> i32 {...}
    }

We use a references to the  struct as we do not want to take ownership form the object, we just want to interact with the data within. we could also interact with the 
instace of the struct by calling (self) and (&mut self) in the parameters of an object to either consume the object or to change the paramenters with in. We use the &self referenced 
to interact with objects the object however we can also add other parameters for the  methods.  

    impl Rectangle{
        fn can_hold(&self, other : &Rectangle)-> bool {...}
    }

    let a = Rectangle(30,30);
    let b = Rectangle(20,20);
    a.can_hold(&b);

When we call the method the we do not need to specify the first object as a refernce of itself '&a.can_hold(&b)', Rust will add the appropiete &, &mut, or * depending on the 
signature of the method. Additionally, we can add more functions to impl block that do not take a &self as a paramenters, these are known as associeted functions,

    impl Rectangle{
        fn square(size : i32) -> Rectangle{...}
    }
    let sqr = Rectangle::square(3);

As we can see we use the 'struct::Fn();' to use the assicieted functions. Further throught these notes we can see that we used multiple impl Rectrangle blocks, this is a valid 
way to extend the functionality of struct with no problems other than it just making it harder to read.
