use rand::Rng;


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x : Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1)
    }
}
fn rolled(x : i32) -> i32 {
    match x {
        3 => {
            println!("You rolled a 3\n");
            x
        }
        _ => 9
    }
}
fn main() {
 let coin = Coin::Penny;
 let value = value_in_cents(&coin);
 println!("\n\nThe value of {:?} is {} cents", coin, value);

 let five = Some(5);
 let six = plus_one(five);
 let _none = plus_one(None);

 println!("{:?}", six);

 let mut rng = rand::thread_rng();
 let dice = rng.gen_range(1..=6);
 if let out = rolled(dice){
    if out != 3 {
     println!("You did not rolled a 3\n");
    }
 };
}
