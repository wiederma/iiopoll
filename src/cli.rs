use std::path::PathBuf;

struct Args {
    location: Option<String>,
    iioroot: PathBuf,
}

impl Args {
    fn parse() -> Self {
        Self {
            let matches = App::new("myapp")
                            .version(crate_version!())
                            .author(crate_authors!())
                            .about("Does awesome things")
        }
    }
}

fn parse_args() {
    unimplemented!()
}
