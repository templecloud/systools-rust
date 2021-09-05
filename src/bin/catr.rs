fn main() {
    if let Err(e) = systools_rust::get_args().and_then(systools_rust::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
