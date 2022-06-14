use clap::Parser as clapParser;

use std::{env, fs, path::PathBuf};

#[derive(clapParser, Debug)]
struct Args {
  /// The pattern to look for
  #[clap(
    parse(from_os_str),
    long,
    short = 'i',
    help = "The Markdown file to parse"
  )]
  input: std::path::PathBuf,
  /// The path to the file to read
  #[clap(
    parse(from_os_str),
    long,
    short = 'o',
    help = "The output file, defaults to input file .html"
  )]
  output: Option<std::path::PathBuf>,
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
#[derive(Parser)]
#[grammar = "inctex.pest"]
struct IncDParser;

enum IncNode<'a> {
  // Node(Vec<(&'a str, IncNode<'a>)>),
  Text(&'a str),
  Command(&'a str),
}

struct Command<'a> {
  name: &'a String,
  args: [&'a String],
}
fn serialize_jsonvalue(val: &IncNode) -> String {
  use IncNode::*;

  match val {
    // Node(o) => {
    //   let contents: Vec<_> = o
    //     .iter()
    //     .map(|(name, value)| format!("\"{}\":{}", name, serialize_jsonvalue(value)))
    //     .collect();
    //   format!("{{{}}}", contents.join(","))
    // }
    Text(s) => format!("\"{}\"", s),
    Command(s) => format!("\"{}\"", s),
    // Number(n) => format!("{}", n),
    // Boolean(b) => format!("{}", b),
    // Null => format!("null"),
  }
}
fn main() {
  // Parse arguments
  let args = Args::parse();
  let input_file = args.input;
  println!("Input file: {}", input_file.display());
  // if no output path is given use default
  let mut output_file = args.output.unwrap_or(input_file.clone());
  output_file.set_extension("html");
  println!("output file: {}", output_file.display());
  println!(
    "Relative path from: {}",
    env::current_dir().unwrap().display()
  );
  println!(
    "The generated html can be found here: {}",
    output_file.display()
  );
  let file_contents = fs::read_to_string(input_file).expect("Could not read file");
  fn parse_json_file(file: &str) -> Result<IncDParser, Error<Rule>> {
    let json = IncDParser::parse(Rule::file, file)?.next().unwrap();
    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> IncDParser {
      match pair.as_rule() {
        // Rule::object => IncDParser::Object(
        //     pair.into_inner()
        //         .map(|pair| {
        //             let mut inner_rules = pair.into_inner();
        //             let name = inner_rules
        //                 .next()
        //                 .unwrap()
        //                 .into_inner()
        //                 .next()
        //                 .unwrap()
        //                 .as_str();
        //             let value = parse_value(inner_rules.next().unwrap());
        //             (name, value)
        //         })
        //         .collect(),
        // ),
        // Rule::array => IncDParser::Array(pair.into_inner().map(parse_value).collect()),
        Rule::command => IncDParser::Command(pair.into_inner().next().unwrap().as_str()),
        Rule::text => IncDParser::Text(pair.into_inner().next().unwrap().as_str()),
        // Rule::number => IncDParser::Number(pair.as_str().parse().unwrap()),
        // Rule::boolean => IncDParser::Boolean(pair.as_str().parse().unwrap()),
        // Rule::null => IncDParser::Null,
        // Rule::json
        // | Rule::EOI
        // | Rule::pair
        // | Rule::value
        // | Rule::inner
        // | Rule::char
        // | Rule::WHITESPACE => unreachable!(),
      }
    }
    Ok(parse_value(json))
    // ...
  }
  return;
}
