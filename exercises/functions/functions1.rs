// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)


pub fn call_me() -> &'static str {
    return "foo"
}

fn main() {
    println!("{}", call_me());
}
