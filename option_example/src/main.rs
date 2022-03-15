fn main() {
    println!("Hello, world!");

    let name = Some("Pankaj");
    let number: Option<&str> = None;

    println!("Value of name : {:?}", name);
    println!("Value of name : {:?}", number);

    let ans = divide(10, 2);
    println!("Value of ans {:?}", ans);

    // get value
    match ans {
        Some(x) => println!("Answer is {}", x),
        None => println!("cannot divide by zero"),
    }

    //shadowing the ans variable
    let ans = divide(10, 0);

    match ans {
        Some(x) => println!("Answer is {}", x),
        None => println!("cannot divide by zero"),
    }

    let num = Some(5);
    println!("Value on unwrap {} ", num.unwrap());
    let num: Option<i32> = None;
    // panic
    // println!("Value on unwrap for None {}", num.unwrap());

    // panic with message
    // println!("Value {}", num.expect("hello panic"));

    println!("Handle none value with unwrap_or {}", num.unwrap_or(6));
    println!(
        "Handle none wiit unwrap_or_else {}",
        num.unwrap_or_else(|| 5)
    );
    println!(
        "Option Map some value with map {:?}",
        Some(5).map(|x| x + 2)
    );

    println!(
        "Handle none value with map_or {}",
        num.map_or(10, |x| x + 3)
    );

    println!("Handle none value with map_or {:?}", num.ok_or(0));
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
