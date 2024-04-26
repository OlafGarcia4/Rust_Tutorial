fn main() {
    println!("\n\n");
    let mut vector = vec![1, 2, 3, 4, 5, 5, 3, 4, 4] ;
    let result = median(&mut vector);
    println!("The median is : {}", result);   
    mode(&mut vector);
    print!("Please enter a word or phrase: ");
    io::stdout().flush(); // since print! doe not create a new line the stdout will need to be refreshed as it is buffered. See the documentation https://doc.rust-lang.org/std/macro.print.html 
    letter_counter();
    println!("\n\n");
    
}

use std::io;
use std::io::Write;

fn letter_counter() {

    let mut user_input = String::new();
    
    io::stdin().read_line(&mut user_input).expect("Please enter a UTF-8 string");
    let mut map = HashMap::new();
    for c in user_input.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    println!("The count of letters: {:?}", map);
}

fn median(list : &mut Vec<i32>)-> i32 {
    list.sort();
    let middle = list.len()/2;
    *list.get(middle).unwrap()
    
}

use std::collections::HashMap;

fn mode(list : &mut Vec<i32>) {
    let mut map = HashMap::new();
    let mut max = 0;
    let mut key = 0;
    for num in list.iter() {
        let count = map.entry(num).or_insert(0);
        *count += 1;

        if *count > max {
            max = *count;
            key = *num;
        }

    }
    println!("The mode is [{key}: {max}]");
}