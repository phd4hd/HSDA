
fn pointer() -> &i32 {
    let N : i32 = 42;
    return N;
}

fn main() {
    println!("pointer = {}", pointer() );
}

// Lösung:
fn solved_pointer() -> &'static i32 {
    static N : i32 = 42;
    return &N;
}
