extern crate selog;
extern crate clap;

selog::opts! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Opts {
        #[clap(about = "The input file.")]
        pub input: Option<String>,
        #[clap(short, long, about = "The server port.", default_value = "3000")]
        pub port: u16
    }
}

pub fn init() -> Opts {
    let opts = Opts::parse();
    opts.init_log().expect("Failed to initialize the logger.");

    opts
}
