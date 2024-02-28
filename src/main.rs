use crate::dns::lookup;

mod dns;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <address>", args[0]);
    }
    let address = &args[1];

    println!("Opening: {}", address);

    let ip = lookup(address).unwrap();
    println!("IP: {}", ip);
}
