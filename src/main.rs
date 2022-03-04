use std::io::BufRead;

use anyhow::Context;
use clap::Parser;
use crypto_hash::{hex_digest, Algorithm};
use uuid::Uuid;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    tool_type: ToolType,
}

#[derive(clap::Subcommand, Debug)]
enum ToolType {
    /// Minify or unminify html
    Html(HtmlArg),

    /// Minify or unminify json
    Json(JsonArg),

    /// Base64 Encoding and Decoding
    B64(Base64Arg),

    /// Popular hash functions (Blake3, SHA1, SHA256, SHA512)
    Hash(HashArg),

    /// Generate an UUID
    Uuid,
}

#[derive(clap::Args, Debug)]
struct HashArg {
    #[clap(subcommand)]
    action: HashAction,
}

#[derive(clap::Subcommand, Debug)]
enum HashAction {
    Md5(InputSource),
    Sha1(InputSource),
    Sha256(InputSource),
    Sha512(InputSource),
    Blake3(InputSource),
}

#[derive(clap::Args, Debug)]
struct HtmlArg {
    #[clap(subcommand)]
    action: HtmlAction,
}

#[derive(clap::Args, Debug)]
struct JsonArg {
    #[clap(subcommand)]
    action: JsonAction,
}

#[derive(clap::Args, Debug)]
struct Base64Arg {
    #[clap(subcommand)]
    action: Base64Action,
}

#[derive(clap::Subcommand, Debug)]
enum HtmlAction {
    Minify(InputSource),
}

#[derive(clap::Subcommand, Debug)]
enum JsonAction {
    Minify(InputSource),
    Unminify(InputSource),
}

#[derive(clap::Subcommand, Debug)]
enum Base64Action {
    Encode(InputSource),
    Decode(InputSource),
}

#[derive(clap::Args, Debug)]
struct InputSource {
    /// Filename to read from or raw input (must specify --raw)
    input: Option<String>,

    /// must be provided if raw input is provided
    #[clap(long)]
    raw: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.tool_type {
        ToolType::Html(h) => match h.action {
            HtmlAction::Minify(is) => for_input(is, |input| {
                println!("{}", minify::html::minify(&input));
                Ok(())
            })
            .context("Minify HTML"),
        },
        ToolType::Json(j) => match j.action {
            JsonAction::Minify(is) => for_input(is, |input| {
                println!("{}", minify::json::minify(&input));
                Ok(())
            })
            .context("Minify JSON"),
            JsonAction::Unminify(is) => for_input(is, |input| {
                let s: serde_json::Value =
                    serde_json::from_str(&input).context("Parse Valid JSON")?;
                println!("{}", serde_json::to_string_pretty(&s).unwrap());
                Ok(())
            })
            .context("Minify JSON"),
        },
        ToolType::B64(b) => match b.action {
            Base64Action::Encode(is) => for_input(is, |input| {
                println!("{}", base64::encode(input));
                Ok(())
            })
            .context("Base64 Encoding"),
            Base64Action::Decode(is) => for_input(is, |input| {
                println!("{:}", String::from_utf8(base64::decode(input)?)?);
                Ok(())
            })
            .context("Base64 Decoding"),
        },
        ToolType::Hash(h) => match h.action {
            HashAction::Md5(is) => hash(is, Algorithm::MD5, "MD5"),
            HashAction::Sha1(is) => hash(is, Algorithm::SHA1, "SHA1"),
            HashAction::Sha256(is) => hash(is, Algorithm::SHA256, "SHA256"),
            HashAction::Sha512(is) => hash(is, Algorithm::SHA512, "SHA512"),
            HashAction::Blake3(is) => for_input(is, |input| {
                println!("{}", blake3::hash(input.as_bytes()).to_hex());
                Ok(())
            })
            .context("Blake3 Hash"),
        },
        ToolType::Uuid => {
            println!("{}", Uuid::new_v4());
            Ok(())
        }
    }
}

fn hash(is: InputSource, algo: Algorithm, name: &str) -> anyhow::Result<()> {
    for_input(is, |input| {
        println!("{}", hex_digest(algo, input.as_bytes()));
        Ok(())
    })
    .context(format!("{} Hash", name))
}

fn for_input(is: InputSource, f: impl Fn(String) -> anyhow::Result<()>) -> anyhow::Result<()> {
    if atty::is(atty::Stream::Stdin) {
        if let Some(input) = is.input {
            if is.raw {
                return f(input);
            }
            let lines = std::fs::read_to_string(input.clone()).context(format!(
                "Reading from file '{}', if this is raw input then specify --raw flag",
                input
            ))?;
            return f(lines);
        } else {
            println!("nothing; to do here!");
            return Err(anyhow::anyhow!(
                "Not input source found. You can either pipe the input or specify a file or plaintext"
            ));
        }
    }

    // prefer piped data
    let stdin = std::io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        lines.push(line);
    }

    f(lines.join("\n"))
}
