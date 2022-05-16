use clap::Parser;
use pulldown_cmark::{html, Options};
use std::{env, ffi::OsStr, fs, path::Path};

// #[derive(Parser)]
// struct Cli {
//   /// The pattern to look for
//   #[clap(parse(from_os_str))]
//   input_file: std::path::PathBuf,
//   /// The path to the file to read
//   #[clap(parse(from_os_str))]
//   output_file: Option<std::path::PathBuf>,
// }

fn main() {
  for argument in env::args() {
    println!("{:?}", argument);
  }

  // let args = std::env::split_paths(unparsed: &T)
  // let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);

  // let in_file = std::env::args().nth(1).expect("no path given");

  // let in_file = Path::new(&args[1]);
  // println!("Input file path is : {:?}", in_file.display());
  // assert_eq!(Some(OsStr::new("md")), in_file.extension());

  let contents =
    fs::read_to_string(env::args().nth(1).expect("no path given")).expect("Could not read file");

  println!("{}", contents);

  let markdown_input = "# Hello world\nThis is a ~~complicated~~ *very simple* example.";

  // Set up options and parser. Strikethroughs are not part of the CommonMark standard
  // and we therefore must enable it explicitly.
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = pulldown_cmark::Parser::new_ext(markdown_input, options);

  // Write to String buffer.
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  // Check that the output is what we expected.
  let expected_html =
    "<h1>Hello world</h1>\n<p>This is a <del>complicated</del> <em>very simple</em> example.</p>\n";
  assert_eq!(expected_html, &html_output);
  println!("{}", html_output);
}
