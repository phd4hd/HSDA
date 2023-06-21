fn main() {
    println!("Wem gehört der String?");
    
    let s1: String = "Hallo".to_string();
    let s2: String = s1;

    println!("{} =?= {}", s1, s2);
}

