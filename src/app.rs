use clap::{Arg, App, crate_name, crate_version, crate_authors, crate_description};

pub struct Params {
    pub input: String,
}

pub fn params() -> Params {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    // note(unwrap): assured by required(true)
    // todo: multiple files
    let input = matches.value_of("INPUT").unwrap();
    Params {
        input: input.to_string()
    }
}
