//! Custom argument parser.
//!
//! `djr` is a very simple CLI, so building clap seemed a bit redundant. Simple argument parsing
//! like shown below should do the trick.

#[derive(Debug)]
pub(crate) enum Arg {
    File(String),
    Output(String),
    Format(String),
    Fmt,
}

pub(crate) fn args() -> Vec<Arg> {
    let mut fields = Vec::new();

    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|a| a.to_string_lossy().to_string())
        .collect();

    let mut enumerated = args.iter().enumerate();
    while let Some((i, arg)) = enumerated.next() {
        match (arg.as_str(), args.get(i + 1)) {
            ("-o" | "--output", Some(opt)) => {
                fields.push(Arg::Output(opt.to_owned()));
                enumerated.next();
            },
            ("-f" | "--format", Some(opt)) => {
                fields.push(Arg::Format(opt.to_owned()));
                enumerated.next();
            },
            ("--fmt", _) => fields.push(Arg::Fmt),
            (file, _) if !file.starts_with("-") => fields.push(Arg::File(file.to_owned())),
            _ => {},
        }
    }

    fields
}

/// This creates a function that checks all the args, and returns true if one of them matches the
/// pattern given. It does not care about the context around it, so it is well suited for toggles
/// that override all other arguments.
macro_rules! toggle {
    ($name:ident, $( $pattern:pat_param )|+) => {
        pub(crate) fn $name() -> bool {
            for arg in std::env::args_os().skip(1) {
                if matches!(arg.to_string_lossy().as_ref(), $( $pattern )|+) {
                    return true
                }
            }

            false
        }
    };
}

toggle!(help, "-h" | "--help" | "help");
toggle!(version, "-v" | "--version");
