mod authenticator_export {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    #![allow(clippy::nursery)]
    include!(concat!(env!("OUT_DIR"), "/authenticator.export.rs"));
}

fn main() {
    println!("Hello, world!");
}
