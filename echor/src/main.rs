use clap::{App, Arg};

//fn main() {
//    println!("{:?}", std::env::args());
//}

//fn main() {
//    let _matches = App::new("echor")
//        .version("0.1.0")
//        .author("Raidel Almeida <contact@raidel.dev")
//        .about("Rust echo")
//        .get_matches();
//}

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Raidel Almeida <contact@raidel.dev")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
            .value_name("Text")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .long("nline")
            .help("Do not print newline")
            .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", matches);
}
