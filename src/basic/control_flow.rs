fn main(){
    // if
    let number = 15;
    let result = if number < 10 {
        "Less than 10"
    } else if number > 20 {
        "Greater than 20"
    } else {
        "Between 10 and 20"
    };

    println!("Result: {}", result);

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
        println!("Count: {}", count);
    }

    // while
    let mut count_w = 0;
    while count_w < 5 {
        count_w += 1;
        println!("Count_w: {}", count_w);
    }

    // for
    for number in 1..5 {
        println!("Number: {}", number);
    }

    // match
    let number_m = 3;

    match number_m {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}