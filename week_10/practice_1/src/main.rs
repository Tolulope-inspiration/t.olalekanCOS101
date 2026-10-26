fn main() {

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap 

    // Only a single variable owns the heap memory a any given time
    let _v2 = v.clone();
    // here two variables owns heap value,
    // two pointers to the same content is not allowed in rust

    // Rust is very smart in terms of memory access, so it detects a race condition
    // as two variables point to same heap

    println!("{:?}",v);

}
