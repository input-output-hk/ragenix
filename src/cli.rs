use std::ffi::OsString;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgGroup, ValueHint,
};

#[allow(dead_code)] // False positive
#[derive(Debug, Clone)]
pub(crate) struct Opts {
    pub edit: Option<String>,
    pub editor: Option<String>,
    pub identities: Option<Vec<String>>,
    pub rekey: bool,
    pub generate: Option<String>,
    pub rules: String,
    pub schema: bool,
    pub verbose: bool,
}

fn build() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("edit")
                .help("edits the age-encrypted FILE using $EDITOR")
                .long("edit")
                .short('e')
                .takes_value(true)
                .value_name("FILE")
                .requires("editor")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("rekey")
                .help("re-encrypts all secrets with specified recipients")
                .long("rekey")
                .short('r')
                .takes_value(false),
        )
        .arg(
            Arg::new("identity")
                .help("private key to use when decrypting")
                .long("identity")
                .short('i')
                .takes_value(true)
                .value_name("PRIVATE_KEY")
                .required(false)
                .multiple_values(true)
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose output")
                .long("verbose")
                .short('v')
                .takes_value(false),
        )
        .arg(
            Arg::new("schema")
                .help("Prints the JSON schema Agenix rules have to conform to")
                .long("schema")
                .short('s')
                .takes_value(false),
        )
        .arg(
            Arg::new("generate")
                .help("(re-)generates the age-encrypted FILE using the specified generate script")
                .long("generate")
                .short('g')
                .takes_value(true)
                .value_name("FILE")
                .value_hint(ValueHint::FilePath),
        )
        .group(
            ArgGroup::new("action")
                .args(&["edit", "rekey", "generate", "schema"])
                .required(true),
        )
        .arg(
            Arg::new("editor")
                .help("editor to use when editing FILE")
                .long("editor")
                .takes_value(true)
                .env("EDITOR")
                .value_name("EDITOR")
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("rules")
                .help("path to Nix file specifying recipient public keys")
                .long("rules")
                .takes_value(true)
                .env("RULES")
                .value_name("RULES")
                .required_unless_present_any(&["schema"])
                .default_value("./secrets.nix")
                .value_hint(ValueHint::FilePath),
        )
}

/// Parse the command line arguments using Clap
#[allow(dead_code)] // False positive
pub(crate) fn parse_args<I, T>(itr: I) -> Opts
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let app = build();

    let matches = app.get_matches_from(itr);

    Opts {
        edit: matches.value_of("edit").map(str::to_string),
        editor: matches.value_of("editor").map(str::to_string),
        identities: matches
            .values_of("identity")
            .map(|vals| vals.map(str::to_string).collect::<Vec<_>>()),
        rekey: matches.is_present("rekey"),
        generate: matches.value_of("generate").map(str::to_string),
        rules: matches
            .value_of("rules")
            .expect("Should never happen")
            .to_string(),
        schema: matches.is_present("schema"),
        verbose: matches.is_present("verbose"),
    }
}
