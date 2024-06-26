
# Concepts of Rust 

Undestanding Mutability 
= 

    e.g. 2.1.1
        fn main() {
            let x = 5;
            println!("The value of x is: {x}");
            x = 6;
            println!("The value of x is: {x}");
        }
This function when compiled will throw a error because of what we are doing to x.
    We first assign x =5 and then we want to chage it to 6, however rust does not allow 
    for variable to be re-assined after they are decalred like other languges.
    By doing 'let mut x =', we tell the the rest of the code that the variables value can be changed.
    This can be beneficials as we can rely on the values not changing.

Constants
-

So we have seen that any variable created in rust will be immutable but this doesnt make them
constants. To decalare we constat we do:

e.g. 2.1.2

    const SOME_VALUE : u32 = 40*40*5;

Constants can be assinged as 'mut', the data type must be declared everytime, they can be used in any scope,
    and the value stored can not come from the code, it needs to be declared before the code runs.

Shadowing
-
    e.g. 2.1.3
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
        }

When we reuse the name of a varaible when we have already previously declared it is called shadowing. What the 
    compiler will do at run time is that it will first assign x =5. Then the complier will over write the value (let x = x + 1)
    and any future references to this value will use this new value to we store until the end of the scope. For example, in the 
    curly-braces block x will take on the new value of x =x*2, and when the block ends the value of x will return to be x = x + 1. 

 Mutability and shadowing seem to be similar, however, Shadowing will allow for the reasignment of the value by using 
    'let' but this is like creating a new varible each time just reusing the name. 'mut' will allow for that specific varible and its type to be
    reassigned at any point through a scope. 

Data type
=

Rust has 4 scalar types : Intergers,Floating Point Numbers, Booleans, and Strings.

Intergers
-

        e.g. 2.2.1
            Length	|Signed	|Unsigned
            8-bit	|   i8  |   u8
            16-bit	|  i16  |   u16
            32-bit	|  i32  |   u32
            64-bit	|  i64  |   u64
            128-bit	|  i128 |   u128
            arch	| isize |   usize

Rust will use the i32 interger type as default. isize and usize will only realy be used 
when you index a collection.

Floating-Points
-
Rust uses f32 and f64 as a it's primitives for floats. Default is f64, with f32 being a single-precision and f34 being double-precision.

Booleans
-
Booleans operate the same way as in other langugaes, specified by ': bool =' or assigning a value 'let t = true;'



Tuples
-
    e.g. 2.2.2

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let(x,y,z)= tup;
        let fiveOO = tup.0;
        
Tuples are created as seen. you will need to specify the data types and their order when creating the tuple
We can then use another other variable to interact with the data in the tuple, this is called Destructuring.

We can also use the x.n approach to access the data in the tuple

Arrays
-
    e.g. 2.2.3

        let a = [1, 2, 3, 4, 5];
        let b: [i32;5] = [1, 2, 3, 4, 5];
        let c = [3,5];
        let first = a[0];

These arrays will be stored on the stack, therefore they should be used for a data whose size is known and the data wont change.[Vectors operate differently]. We can also specify the data type and size for array with ': [type;size]='. We can also fill a array by invoking similar syntax '= [value, size]'. To access the value we do x.[idx]

Functions
-
    e.g. 2.3.1
    
    fn main(){
        function1(7,h);
        function2(4);
    }
    fn function1(x : i32, unit : char){
        println!(this is x : {x});
        println!(this is unit lable : {x});
    }
    fn function2(x : i32) -> i32 {
        x + 3
    }

Functions work like in most other langugaes. Parameters types need to be specified. Returning 
values from a function is done throught the 'function(param) -> type' notation, it is crucial to remember that
if you have a return function it ends with a expression and not a statement. 
    
Statments vs Expressions
-

In Rust statments are instructions with no return value and expressions give a returning value.
Statments are things like 'let x = 4;', and the function definitions. This means that logically, the statment
'let x = (let y = 4)' would make sense [x=y=4], in Rust this is not the case. To understand why, If we extrapulate 
the 'let y = 4' and let it be A then we can see that if A is a statment, it will have no value since it can't return 
a value by its definition leaving nothing for x to be binded to.

    e.g. 2.3.2

        let x =(                
            let y = 4   ------>   let x = (A)      A has no value since its a statement
        )
    
Expression is what most of the Rust will be. Things that evaluate or return a value are expressions, meaning that 
3 + 4 is an expression since it evaluates to 7 and even the 4 in 'let y = 4' is an expression since it evaluates to 4.
Scope blocks are expressions as well, meaning that we can some what make it so x = y = 5:
    
    e.g. 2.3.3

    let x = {
       let y = 5;
        y
    };
    
This will cause no error to appear at run time becuase although we do create and initialize y, its okay becuase the x is being
assinged the value that the scope returns. (In this case, calling 'y' is evaluating its value which is then returned as the scope block
ends and assigned to x). However, the variable y will only exists for as long as the scope block is being active.

***[Quick note: in a scope, staments are always closed by a ; and expressions are not. If you add a semicolon to a expression you'll make it 
into a statement.If you need to evaluate the scope to something else it will fail since you are evaluating statements]***

    {
        statement1;
        statement2;
        expression1 
        statement3;
        expression2 
    };

Ifs and Loops
-

If statements work the same as a in other languges. You can assing a value to a if expression, however keep in mind that Rust will 
expect for both evaluating values to be of the same type at complie time instead of figuring out the type on run time. 
    
    e.g. 2.3.4

    let temp = if(condition){...} else {...};
  
    if [condition1]{
        expression
    }else if [condition2]{
        expression
    }else{
        expression
    }

Loops come in the 3 styles of loop,while, and for. The loop will execute a block of code until it is given the break command. If you
    need multiple nested loop loops then you can name/lable them like  this " 'name : loop {...} ". While Loops are the same as other languge,
     checks the condition at runtime making it slower. For loops are the best to iterate through a collection and can be used with the a range.

    e.g. 2.3.5
        
    'froot : loop {
         if a = 3 {
            break;
        }
        loop{
            if a =2 {
            break 'froot;
            }
            a += 1;
        }
    }

    while b != 0{
        println("{}", b);
        b -= 1;
    }
    for element in c {
        println("for: {element}");
    }
    for number in (1..5){
        println("For(range): {number}");
    }