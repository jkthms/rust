// if condition changes control flow

fn main() {
    let a = 99;

    // if check
    if a > 1 {
        println!("a is greater than 1");
    } else {
        println!("a is less than or equal to 1");
    }

    // infinite loop
    // let b be a mutable variable
    let mut b = 0;

    loop {
        if b == 5 {
            break;
        }
        println!("b: {}", b);
        b += 1;
    }

    // while loop
    
    let mut c = 0;

    while c != 5 {
        println!("c: {}", c);
        c += 1;
    }
}