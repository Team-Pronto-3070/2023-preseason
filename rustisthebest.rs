

enum Lang {
    Java,
    Rust,
}

fn main() {
    println!("rust > java: {}", Lang::Rust > Lang::Java);
}