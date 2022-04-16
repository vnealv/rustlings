// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let mut vec0 = Vec::new();

    // one solution
    // let mut vec1 = fill_vec(vec0.clone());
    

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    // Or move it to after the immutable ref to vec0 above.
    // and instead of clonning it, use mutable ref
    let mut vec1 = fill_vec(&mut vec0);
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    //let mut vec = vec;

    //let mut s = vec.clone();
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
