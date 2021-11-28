use regex::Regex;
use std::env;
use std::process;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]

pub struct Release {
    pub owner: String,
    pub repo: String,
    pub tag: String,
    pub name: String,
}

pub struct Options {
    pub release: Release,
    pub file: String,
    pub token: Option<String>,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} URL [FILE]", program);
    print!("{}", opts.usage(&brief));
}

pub fn parse_opts() -> Options {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    let mut opts = getopts::Options::new();

    opts.optflag("v", "version", "Print version and exit");
    opts.optflag("h", "help", "Print usage and exit");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        process::exit(0);
    }

    if matches.free.len() < 1 || matches.free.len() > 2 {
        print_usage(&program, opts);
        process::exit(1);
    }

    let url_str = &matches.free[0];

    let url = if let Ok(u) = reqwest::Url::parse(url_str) {
        u
    } else {
        eprintln!("Unable to parse URL: {}", url_str);
        process::exit(1);
    };

    let re = Regex::new(r"\A/([^/]+)/([^/]+)/releases/download/([^/]+)/([^/]+)\z").unwrap();

    let (owner, repo, tag, name) = if let Some(m) = re.captures(url.path()) {
        (
            m.get(1).unwrap().as_str(),
            m.get(2).unwrap().as_str(),
            m.get(3).unwrap().as_str(),
            m.get(4).unwrap().as_str(),
        )
    } else {
        eprintln!("Invalid URL: {}", url_str);
        process::exit(1);
    };

    let file = if matches.free.len() == 2 {
        &matches.free[1]
    } else {
        name
    };

    let token = if let Ok(t) = env::var("GITHUB_TOKEN") {
        Some(t)
    } else {
        None
    };

    Options {
        release: Release {
            owner: owner.to_string(),
            repo: repo.to_string(),
            tag: tag.to_string(),
            name: name.to_string(),
        },
        token: token,
        file: file.to_string(),
    }
}
