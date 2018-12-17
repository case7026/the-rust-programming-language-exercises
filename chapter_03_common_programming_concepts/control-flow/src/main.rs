fn main() {
    rev_for_loop();
}

// fn infinite_loop() {
//     loop {
//         println!("again!");
//     }
// }

// fn infinite_loop_return() {
//     let mut counter = 0;

//     let result = loop {
//         counter +=1;

//         println!("counter equals: {}", counter);
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("result equals: {}", result);
//     println!("we are out of the loop");
//     assert_eq!(result, 20);
// }

// fn while_loop() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}", number);

//         number = number - 1;
//     }

//     println!("WE OUT!")
// }

// fn for_loop() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("The value is: {}", element);
//     }
// }

fn rev_for_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}