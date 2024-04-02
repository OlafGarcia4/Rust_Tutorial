/**
 * 
 * See Notes for more information regarding this Chapter.
 * 
 */
const SOME_VALUE : u32 = 40*40*5;

fn main() {
    mutability();
    shadowing();
    tttuples();
    arrays();
    function1(7,'h');
    function2(3);
    x_eq_y_eq_five();
    meLoops();

}

 
    
    fn mutability() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }
    fn shadowing(){
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
        }
    }
    fn tttuples(){
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let(x,y,z)= tup;
        let fiveOO = tup.0;
    }
    fn arrays(){
        let a = [1, 2, 3, 4, 5];
        let b: [i32;5] = [1, 2, 3, 4, 5];
        let c = [3,5];
        let first = a[0];
    }

    fn function1(x : i32, unit : char){
        println!("this is x : {x}");
        println!("this is unit lable : {x}");
    }
    fn function2(x : i32) -> i32 {
        x + 3
    }
    fn x_eq_y_eq_five() {
        let y = {
            let x =5;
            println!("The value of x is: {}", x);
            x
        };
        println!("The value of y is: {}", y);
    }
    fn meLoops() {
        let mut a = 0;
        let mut b = 3;
        let c =[1, 2, 3, 4, 5];

        'froot : loop {
            if a == 3 {
                break;
            }
            loop{
                if a ==2 {
                break 'froot;
                }
                a += 1;
            }
        }

        while b != 0{
            println!("{}", b);
            b -= 1;
        }

        for element in c {
            println!("for: {element}");
        }
        for number in (1..5){
            println!("For(range): {number}");
        }
    }

