# Collections and more Data structures

We will cover 3 crucial data structure 

    -Vectors
    -Strings
    -Hash Maps

Vectors 
-
    let v : Vec<i32> = Vec::new();

 Above is how we difine a new vectore, we can see that we have the option to specify the type of data that this structre will hold,
    on this case i32, if we are not inserting data otherwise we can just say:

    let v = vex![1,2,3,4];

 Vec<T> comes from the standar library and can hold any type of data, while vec![] is a macro. 
 Like having the a normal varible, inorder to modify the vector we will need to declare it as mutable. After, we can add data to 
    the vector by calling the add() method.

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

 We can also access the data of a vertor by indexing or using the get() method. When we index the start of the vector will be index 0.

    let mut vec_example = vec![1,2,3,4,5]

    let mut third_val_idx = &vec_example[2];

    let mut third_val_get = vec_example.get(2);

For when we call for the index of a value we need to get a reference to the vector as we dont want to change the list. Using the get() method 
    we do not need to expressively sent a reference since it takes care of it and returns the value we ask for. This allows us to interact with 
    the data structure differently. For example if we want to call for a value bigger that the size of the vectore we get diffrent error. If we were 
    to call for &vec_example[100], the program will panic and end with an error, but when we use the get() method instead of the program ending instead 
    we get returned a value of None. This allows us to handle calls beyond the size of the vector with out the program ending in panic.

Rules for borrowing and checking apply to vectors, so if the vector is declared as mutable the values where we store data should also be declared as mut.
    Remember that at compile time, Rust will allocate aveliable memmory in the heap, so adding a new value to the vetor will cause an error as there might not 
    be enough space to keep all the data together.So doing something like this wont work since the size of vec_example is already accounted for.

    let first = &vec_example[0]; // We can assign a the data becuse the data is mutable and an the variable is immutable (See Chapter 3)
    vec_example.push(6)  // We can add a new value to the vetor beacuse we the vector would need to reallocate the values in the heap

Iterating values over a vector is simple, We just use a for loop and use a reference to check for the values

    for i in &mut vec_example{
        println!("{i}");
    }
    for i in &mut vec_example{
        *i += 20;
    }

As we can see if we want to edit the vector as we iterate over it, we need to dereference the value in the list(represented by i in this case) using the * operator before we change
    the data. (We need to obey the borrow rules)
. Because the vector accept any type of data, it means that we can set them to accept a enum structure we create.

    enum Contacts{
        Number(i32),
        Name(String),
        Email(String),
    }

    let mom = {
        Contacts::Number(5558100),
        Contacts::Name(String::from("Mom")),
        Contacts::Email(String::from("mom@email.com")),
    }

We need to express what type of data the enum will hold so Rust can allocate the right amount of memory, so using an a match case and enum will garantee that there wont be any problems at 
    compile time. If we dont know the exhaustive set of types that the program will recieve at run time for the vector to store then this enum technique won't work, and we would need to use traits.

Vec(T) data structure comes with a lot of other methods from the standar library. Most commonly used are push and pop. These act like the push and pop actions on stacks of other languages.
    Keep in mind that due to how Rust memmory handling is, it means that once the vectore is outisde the {} scope it will be dropped from memory along with its contents.


Strings
-

We have talked a bit about strings on a previous chapter (Chapter 3), here we will get more in depth.

There are 3 main issues to get stuck when working with string in Rust.

    1. The propensity for exposing possible errors
    2. Strings high complexity
    3. UTF-8 encoding for strings 

Strings are collections of bytes of data, so we can use multiple methods to creating, updating, and reading data, however it behaves differntly when compared to other data collections. A string in rust 
will normally mean str, or a string slice. We interact with these slices using references like &str. A string literal is the collection of charcter encoded with UTF-8. The String data type is instead provided 
by Rust standard library instead of being coded into the language itself.

    let mut s= String::new();

String is implemented as wrapper around a vector of UTF-8 bytes with some guarantees, restrictions and capabilities. Above we can see how we create a new isntace of String. In this case the String is empty so we 
can store data, if we want to load data as String we need to use the to_string() method or String::from():

    let data = "Some data"
    let s = data.to_string();
    //let s = "Some data".to_string(); this will do the same function

    let s1 = String::from("Some data");

Both ways to add data are achive the same result and wheter to use one method or the other is up to the use case or readability. Additionaly we can use other string APIs to do this and other actions giving us more 
versitility. 

String, like vectors, can grow in size and can change the data with in. We can use the push_str() method,  we can concanote string by using the + operator with a few caviates, and with the format! macro.

    let mut s = String::from("Some data");
    let s1 = " is stored";
    let third = "!";
    // ways to update data in String

    s.push_str(s1) //s.push_str("is stored");
    s.push('!');
    let s2 = s + &s1;
    let big_string = format!("{s} {s1} {third}") //format!("{} {} {}", s, s1, third);

Let break down each of these ways to update a string. push_str() will append a string slice to the string we call the method for, this will allow us to reuse the appended string slice since we are just taking a reference of it and not ownership.
The push() method will instead add a single character to the string we use this method for.

Now lets talk about the + and format!. Using the + operator will concatenate 2 strings, taking ownership of the first string passed and then getting a reference value for the second string. This is beacuse of how the add() operation works in Rust.
 The signature of add is 'fn add(self, s: &str)->String', we see we take the first string in order to change it and return not a reference value but a new String value. So we would not be able to call s later in the program because s2 took ownership
but we can call s1 later since we just refered it. THis is due to the fact that we can not add 2 String data types but we can add a String and str (string slice), having the add method take ownership of self. This means that we only take a copy of s1,
instead of creating a copy of both s and s1 (This is more efficient).

Using the + operator has it's limitiations, it only really works when we just want to add 2 strings. So adding s + s1 + third will look like this:
add(self, add(self, third)), making this process unwieldy.

format!() solves this issue. The macro acts like println!, but instead of printing to screen it just returns a String. The macro uses refernces, meaning that we can reuse the original string later in the code.

Indexing Strings can be bit more difficult. In other languages Strings can be index using some syntax like s[idx], however this will give us an error in Rust due to how Strings are stored in memory. Since String is a wrapper around a vector encoded in UTF-8
it means that certain charcters will take more space in memory, For example the String::from("Hola, mundo!") is 12 bytes long but String::from("Здравствуйте") is 24 bytes despite only being 12 charcters. This is due to the UTF-8 encoding. The Unicode scalar value 
for each charcter takes 2 bytes to store, skewing the index. So when we do try to index the value of a charcter we won't recieve the character but instead it will the value of the charcter based on the encoding.

Because of the encoding we have 3 differnt ways to intrepet words in UTF-8. First we have bytes(the actual value of a character), then Unicode scalar(what Rust recognizes as characters) and lastly grapheme clusters(what we call letters)

    “नमस्ते” //the word we pass to rust 
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] //the actual value to display (bytes)
    ['न', 'म', 'स', '्', 'त', 'े'] //What rust recognizes as charctes (Unicode Scalar Values)
    ["न", "म", "स्", "ते"]  // what we recognize as 'letters' (Grapheme Clusters)

Becuase of the computer only interpreting the bytes, it is impossible to index a String in Rust to recieve the letter/charcter we want. As a side note, another reason why we can index a string is because the indexing operations are expected to run on O(1) time. This is not
because Rust will need to go through the data to see what are the valid characters.

So how we can access the data from a String? If we need to use indices to create string slices, we need to be specific. We could use &s[i..n-1], but this will make a string slice contaning the bytes of the string. However in cases like with the russian or the hindi words, it
    is important that we index the appropite amount of bytes for the corresponding character we want, otherwise the program will panic and crash.

We achive this by specifing what type of data we want with the corresponding method.

    for i in "hello".chars(){...}

    for i in "hello".bytes(){...}

To get a grapheme its more tricky. Rust standard library does not contain a way to get them, but there are plenty of crates online that can achive this. String and &str have multiple methods to manipulate them (Contain, replace, etc).

Hash Maps 
-
    Similar to other languages, has a data structure of (key, Value). This is how we initilize it:

        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);


 As we can see we call HashMap into the scope of the project and then we can call a HashMap. then we can use the insert method that accepts a key and value of any type. Remember that once a type of data is selected the rest of the data to load to the struct should be the same type.
    lets look at how we can interact with the HashMap:

        let team_color = "Blue";
        let score = scores.get(&team_color).copied().unwrap_or(0);

Here we use the get() method to get the score of the team we are looking for, this will return a Option<&Value> type, we use copied() to create a copy of the option into a Option<i32> and not a Option<&i32>, then we take the value with in with unwrap_or, if the Option is None then we just set the score to 0. To iterate over the hash map wo can do so with a for loop:

    for (key, value) in &scores.{
        ...
    }

Here are the ways we can update the data in the hash map

    scores.insert(String::from("Blue"), 10);  //overwrite 
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Yellow")).or_insert(50); // Adding a new Key/Value if it does not exists
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {       // Update based on previous value  
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

The first block of code shows us how we can overwrite a key/value set by just calling another insert with the same key, updating only the value. The second block of code shows us how we can add new Key/Value set when it does not exist on the hashmap already, otherwise the or_insert will check and
    if the key does not exists then it adds the value in its parameter to Value, else, it will not a change the Value. Lastly Here is how we can upadte the Value it self, in this case we fist use the split_whitespace to add the text per word skiping spaces, then we iterate through the map adding each word
    while evealuating if the key we pass with .entry exists or we add 1 to the Value.

 Hashing Functions: The Rust languge uses the hash funnction called SipHash with good protection against DoS attacks at the cost of speed. We'll talk more on hashers and the BuildHasher Trait.

Lets talk ownership. Any type that has the copy trait will be copied to the hashmap, while Strings will need to be owned by the hashmap. Meaning that we could reuse types like i32 later in the code after we passed them to the hashmap, but string declared before the hashmap and then 
    passed to it wont be able to be reused or refereced. 

These are the basic data collections that you'll use in Rust, 