fn main() {
    println!( "{}", defancify::allocs::defancify_lossy(
        &std::env::args().skip(1).next().unwrap_or("No arguments passed in!".to_string()).to_owned(),
        None
    ))
}
