mod test;

fn main() {
    println!("Hello, world!");
}

pub fn add(chr: String) -> u32{
    return if chr == "" { 0 } else { 1 }
}
