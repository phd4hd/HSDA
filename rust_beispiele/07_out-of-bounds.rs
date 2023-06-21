fn main() {
    println!("Buffer overflow!");
    let array: [u32; 5] = [0;5];
    for index in 0..array.len() + 1 {
        println!("Index {}: {}", index, array[index])
    }
}
