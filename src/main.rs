use lsd::log::key::deserilize;

const HELP: &str = "LSD
usage: LSD --input <input> --schema <schema>

INPUT
the path for log file to be deserilized
options: file (none for /var/log/secure)

SCHEMA
File type for export
option: JSON, TOML, YAML (none for json)";

type File = String;

#[derive(Debug, Default)]
struct Cmd {
    schema: Option<Schema>,
    file: Option<File>,
    pretty: bool,
}
#[derive(Debug, Default)]
enum Schema {
    #[default]
    Json,
    Yaml,
}

impl Schema {
    pub fn from_str(input: &str) -> Self {
        match input {
            "json" | "JSON" => Schema::Json,
            "yaml" | "YAML" => Schema::Yaml,
            _ => Schema::Json,
        }
    }
}

fn main() {
    let args = idontlikeargs();

    let mut cmd = Cmd::default();

    if args.contains(&String::from("--help")) {
        println!("{HELP}");
        return;
    }

    for x in 0..args.len() {
        match args[x].as_str() {
            "--input" | "-i" => cmd.file = Some(args[x + 1].clone()),
            "--schema" | "-s" => cmd.schema = Some(Schema::from_str(&args[x + 1])),
            "--pretty" | "-p" => cmd.pretty = true,
            _ => {}
        };
    }

    let directory = cmd.file.unwrap_or(String::from("/var/log/secure"));

    if let Err(error) = std::fs::File::open(&directory) {
        println!("File Error\n{error}\nCould not access {directory}");
        return;
    };

    let raw = deserilize(&directory);

    let converted = match (cmd.schema.unwrap_or_default(), cmd.pretty) {
        (Schema::Json, true) => serde_json::to_string_pretty(&raw).unwrap_or_default(),
        (Schema::Json, false) => serde_json::to_string(&raw).unwrap_or_default(),

        (Schema::Yaml, _) => serde_yaml::to_string(&raw).unwrap(),
    };
    println!("{converted}");
}

fn idontlikeargs() -> Vec<String> {
    let mut vec = Vec::new();
    for x in std::env::args() {
        vec.push(x);
    }
    vec
}
