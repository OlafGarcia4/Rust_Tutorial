Line 1 calls the io library into the scope of the program
This is how we can use other packages in and out of the standard library

    let mut x = String::new();

The 'let' part will allow you to assing a value to a variable when creating into
In rust all variables after creation will become immutable, meaning we can not change the value
stored.

'mut' allows us to create a variable that we can manipulate the content. 

'String::new();' is a function from the String class that will create a new empty instance of a String


'io::stdin()' function in the io library that will take user input
'.read_line(&mut guess)' method that will handle the input storing it on a string of type mutable
'.expect("Failed to read line");'  This is how we handle possible errors that the readLine method might give with a message

This could be written as 
io::stdin().read_line(&mut guess).expect("Failed to read line");
however this makes it harder to read. Use a whitespace format to keep the method option of function more legiable



This is how we can use varibles with in a String to print. to do so we use the {} signs as placeholders
then we can do the following to set the info

    let x = 4;
    let y = 3;

    printlin("this is {x}");
    printlin("this is {}", x);
    printlin("this is {x}, and this is {}", y);

the '&' works much like how pointer work with C, more on that on Ch4. In rust they are called a reference.


After adding the the rand packege into the dependecied section in the TOML we need to build the project inorder 
for the rand library to be loaded into the project.

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

This line is were we convert the String that we are taking from the user into a interger, in this case a u32.
The variabale as the same name as the previous variable in the project, this is okay due to Shadowing. Shadowing is
this feature is often used when you want to convert a value from one type to another type.

***.trim*** - This trims the whitespaces before and after string

***.parse*** - Does exactly what it sounds, it parses the selected string to convert the 
string to another data type.

**.expect** - This is just error handling

***: u32*** - This is how we can specify the type of data that the variable should stored

loop { } will funtion as a while(1) loop, continueing until the programs is interupted.
For this we added a break command so the game closes when the user guesses correctly

