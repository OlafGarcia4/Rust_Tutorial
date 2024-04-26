Ownership

Ownership is the set of rules that Rust uses in order to control memory. Rust has no garbage collector
nor does it have the user manage it, instead it uses a system of owner and barrower. Ownership mainly 
deals with the heap instead of the stack by keeping track of data, and its use, in the heap, minimizing 
duplicate data on the heap and cleaning unused memory. 

Rules 
-

    1. Each value has a owner
    2. Only 1 owner can exist at a time
    3. When the owner leaves the scope, the value is removed is dropped

We'll be using Strings here to understand ownership since it is stored in the heap. So far we have used 
string literals (let h = "something") but we will use the second one, String, because it calls for allocation
on the heap to store an unknown size of data.
    {
        let mut s = String::from("Hello");
    }
The :: allows us to call the this specific function under the String type and not from the other string type.
Because the String type allocation memory in the heap it allows us to set a variable of this type as a mutable.
As rule 3, the above scope block is the only place that s is an owner, once it end Rust will free the allocated 
space that was called for s.  

    let s1 = String::from("Hello");
    let s2 = s1;

Normaly we can say the  let x = 4; let y = x;, however, when in a using unknown data size variable like s1 we do
not simply copy the data, we copy the pointer to the data on the heap. But if we Rust drops allocations when the 
owner goes out of a scope what happens to the s1 and s2. What Rust will do is no longer consider s1 as valid, meaning 
that s2 is the only way to access the data on the heap, this is called a move.

Clone
-
Clone method will allows us to copy the data of the heap instead of invalidating and coping the pointer,lenght, and 
capacity. This method is very resource heavy since the program will need to allocate space on the heap and copy the data

    let s1 = String::from("Hello");
    let s2 = s1.clone();

Stack data
-
Most data we will use we know the size of it and therefore it is easier to create copies on the stack rather thatn 
on the heap. Copy is a trait the only applicable to types that can be stored in the stack, this includes:
- intergers
- booleans
- floating-points
- Characters
- Tupples*

    **Only if the types in the tuple are also the same types previously mentioned*

Functions and Ownership
-
    {
    let s = String::from("hello");
    takes_ownership(s);
    let s1 = String::from("hello");
    let s2 = gives_ownership(s1);
    }
    fn takes_ownership(some_string: String) { 
        println!("{}", some_string);
    }
    fn gives_ownership(a_string: String) -> String { 
        let some_string = String::from("yours");
        some_string
    }

Ownership will be moved unless especified. Meaning that the ownership of s get moved to the takes_ownership function.
When the scope of that function, whatever allocated space was reserved for s initially gets dropped and the. The same
process repeats when ruturning a value. The ownership would go from the function to the variable either through move,
clone, or copy.

 References and Borrowing 
 -
    {
        let mut s = String::from("hello");
        let len = calc_len(&mut s1);
        println!("length of '{}' is {}.", s, len);
        {
            change(&mut s1);
        }
        let r1 = &s; 
        let r2 = &s; 
        println!("{} and {}", r1, r2);
        
        let r3 = &mut s; // no problem
        println!("{}", r3);
    } 
    

    fn calc_len(s : &mut String)->usize{
        s.len()
    }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

This is how we can use a variable without taking ownership. By using references (&var), we can gaing access to the object's
data without doing a move or having the value dropped from the heap. This borrowing of data also means that we cannot modify
the references unless the variable we reference is mutable.

When we use mutable references we need to remember that we can't have multipe mutable references. This is done to prevent data 
races from happening, so you can't borrow the same reference until its out of a scope. Just like real life, you can't borrow what
someone else is using. If you need to have a mutable reference after an immutable reference, you need to make sure that he immutable reference 
reference is out of a scope or not longer referenced.

Rust makes sure that any reference to the any data, ends before the data is out of the scope. This prevents a dangling reference or a 
reference to a emptied allocation. This is what we will call a lifetime for a refernce.

**Basic rules for references**
1. you can have 1 mutable reference or many immutable references
2. References must always be valid
3. Scope will determine how long a reference exists

Slices
-

Another way we can borrow data from a collection is by using slices. A slice a  nothing more that a reference to a range we can determine

    let x = [1,2,3,4,5,6,7,8]

    let y = &x[1..5]

We use ranges to determine the size of the slice we want out of a collection. This can  be done with Strings but not string litterals as the
String literals are already a slice of original string, this is why we can not mannipulate string literals as they are a immutable reference
to the String in the binary.
 