// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.


use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();// TODO: declare your hash map here.
    let names = vec![String::from("Mango"), String::from("Durian")];
    let counts = vec![20, 4];

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    //basket.insert(names.into_iter().zip(counts.into_iter()).collect());
    //basket.insert(names.into_iter().collect(), counts.into_iter().collect());
    for (k, v) in names.into_iter().zip(counts.into_iter()) {
        println!("Dddd {} {}", k, v);
        basket.insert(k, v);
    }


    // TODO: Put more fruits in your basket here.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
