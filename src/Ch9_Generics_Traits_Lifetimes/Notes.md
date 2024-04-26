# Generic Types, Traits, and Lifetimes

Generics Data Types
-

In this chapter we will examine how to use generics types in functions,stucts and enums; how we can define behaviors for these generics we create; lastly, well see how we can give the compiler information about how these references behave and how to ensure they remain valid for. 

Generics Data Types
-
Well begin to disscuss Generic types. We have seen the funtionality of Generics like in Option<T>, Result<T,E>, or HashMap<K,V>. We can see that these generics eliminate the need to write a function for each type.

When declaring a function signature, we need to specify that it take a generic:

    fn any_type<T> (param) -> <T>{...}

We can also use generics with stucts and impl blocks.

    struct Point<T, U>{
        x: T,
        y: U,
    } 
    impl<T, U> Point<T, U>{ 
        fn x(&self)->&T{
            &self.x
        }
        
    }
Has we can see in order to declare generic acceptace all we need to do is declare a generic variable with **<>**. There are a couple things that we can do with generics. As seen, we can declare multiple generics with in a function, We could declare just one generics but this will lock the function to only have one type through the function/object we create. Using T and U will allow us to get 2 different data types with out running into this issue. Although you can declare multiple generics at once, this is not recommended, instead try to add more abstraction to keep code more readable.

 Additionaly, with using generics we can give specificity for the type of data we would like to interacc with. For example, if we wanted to add a method that will only interact with the **f64** data type, we can just declare the type with in the angle bracketts **\<f64\>**. This along with the implementation blocks allows for specifity even when using generics by the data type (So if our Point struct had and impl block for f64, then we could only call for methods on that impl block if there is data of type f64 being passed).

 We can also use objects with generics as parameters. It is important to note that we need to use diffrent generic variables if we expect the incoming data to be of different type than the current object.

    fn accepts_Diff<T,U>(&self,other , Point<V,W>)-> Point(T,W){...}

    fn accepts_same<T,U>(&self,other , Point<T,U>)-> Point<T,U>{...}

The accepts_Diff will accpet another point with a possible different data types, represented by V and W, with the return type specifing that it will retrun type T and type W. The accepts_same method will only return the same data type T and U, as we expect the data types to be the same for the current and passed objects.

Generics are great to create better abstraction and eliminate duplication of code.

Traits
-

Traits is how we can define behaviors for generics we create. 

    pub trait Type_behavior(){
        someMethod(&self)->String;
        //methods signatures
    }

This it the basic decalration of the methods will implement. It is declared as public so it can be used through the projects. We don't have to define the methods here, but instaed we will use impl blocks to specify for the data types. For example:

    impl Type_behavior for Point{
        format("{} , {}", self.x, self.y)
    }

As we can see, when we can create methods that will interact witht he struct Point and return its value. If we where to define this method with in the trait block, this this will become the default behavior, meaning that we can call it regardless of the data type, however this can cause issues depending on the what the code is doing. With this info we can set up default and special behavior for each class. If there is only a signature delcared on Type_behavior then we will need to declare the definition in the impl block for the each impl created.

We can use these traits as parameters as well

    pub fn cords(p : &impl Type_behavior){...}

    pub fn cords<T : Type_behavior>(p: &T)

The parameters here are a object that implements the trait of Type_behavior. The second example is what we call a trait bound.

Trait bound is a more specific way to use data that shares a behavior.  Both examples do the same actions, however, using a trait bound will allow us to specify that the data coming in is of the same type while the other function will not. Additionally, we can further the define the behavior of the data coming in by adding the traits onto the signature:

    pub fn cords(p : &(impl Type_behavior + Behavior){...}

    pub fn cords<T : Type_behavior + Behavior>(p: &T)

This is how we can define that the data coming in will need to have the Type_behaviors and Behaviors traits. We can make it less confusing on the trait bound by using the ***where*** clause:

    fn some_func<T, U>(t: T, u: U){
        where T : Behavior1 + Behavior 2,
              U : Behavior3 + Behavior 4
        // ... 
    }

Now lets talk about return a type with a trait. 

    pub fn cords<T : Type_behavior>(p: &T) -> impl Type_behavior{...}

We define the return type to be a type that will implement the trait Type_behavior. This will have issues as we can only return one data type,we'll see how we can retrun the data type with out this error later on.

With having traits, it means that we can call methods from these traits to manipulate the data. This will allow us more precise access and control over the behavior of types or generics.

Life Times
- 

Like we have seen before, life times, or how long a value remains valid before its dropped, is defined by the scope of a project. We can better understand the lifetimes by using generic lifetime annotations

    fn ttl<'a>(x: &'a str,y: &'a str)-> &'a str{...}

As we can see from the signature above to decalre a lifetime relationship. It is important to remember that the lifetime annotion does not chages the actual lifetime of a varaible but instead created a relationship. So in the signature we are saying that there is a relationship between x,y and the return value, this relationship being that they share the smallest lifetime with in the function 

    fn main(){
        let x =...
        {
            let y = ...
            let result = ttl(x,y); 
            // 1
        }
        // 2
    }

In this main function we see that there is 2 scopes. If we were to interact with result at 1 then there is no problem, this object would live for as long as the scope is open, if we use the data at 2 then we have an issue as result lifetime is tied to the smallest lifetime, in this case y's lifetime. This can help the compiler better understand our code and also recognize dangling references. It is important that when declaring lifetimes that the return lifetime is tied to atleast 1 parameter lifetime. This is beacuse the data inside the function will only be available until that function's scope is closed, which happens at the time of return.

    struct example<'a>{
        data: &'a str,
    }

    fn main() {
        let x =...
        let i = example {
            data: x,
            //1
        }
        // 2

Looking at the example above we can see how this applies. The example function lifetime is tied to the lifetime of data. So at 1 the reference of data remains valid until the end of the scope, however, if we were to call i from spot 2 then we would get an error as the example struct's lifetime ends on the same line where we assign its value to i

here are the 3 rules reagrding lifetimes:

1. Each parameter - that is a reference - gets its own lifetime
1. If there is one referenced parameter, then the outputs will share the same lifetime
1. If there is a self reference (&self || &mut self), then the outputs will be assigned to the same lifetime as self.

This is how the compiler creates the lifetime relationships.But how can we just create a varible that will remain valid until the end of our program? We use static lifetimes:

let x: & 'static str = "Hello, world!";

Strings have a static lifetime since the data for the string in store in the binary. 