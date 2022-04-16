// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    let word = match optional_word {
        Some(n) => println!("The word is: {}", n),
        None => println!("The optional word doesn't contain anything"),
    };

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    //integer = optional_integers_vec.pop() {
    //    println!("current value: {}", integer);
    //}
    /*
     * Infinite loop
    let mut integer :Option<i8>;
    loop {
        integer = match optional_integers_vec.pop() {
            Some(x) => {
                println!("current value: {:?}", x);
                x
            },
            None => {
                println!("NONE");
                None
            },
        }
    }
    */
    while let Some(integer) = optional_integers_vec.pop() {
        println!("Final value: {}", integer.unwrap());
    }
}
