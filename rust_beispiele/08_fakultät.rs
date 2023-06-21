fn fakultät(i: u64) -> Option<u64> {
    match i {
        0 => Some(1),
        n => match fakultät(n - 1) {
            Some(m) => n.checked_mul(m),
            None => None
        }
    }
}

fn main() {
    print!("{:?}", fakultät(20) );
}
