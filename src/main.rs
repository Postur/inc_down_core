use clap::Parser;
use pulldown_cmark::{html, Options};
use std::io::Write as _;
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
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
fn parse(input: PathBuf) -> String {
  let file_contents = fs::read_to_string(input).expect("Could not read file");
  // parse file_contents info front_matter and content
  // register plugins from markdown front_matter

  // run content through plugins
  let contents = file_contents;
  // return contents
  contents
}
fn main() {
  let args = Args::parse();
  let input_file = args.input;
  println!("Input file: {}", input_file.display());
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
  let content = parse(input_file);
  let mut parser_options = Options::empty();
  parser_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

  let parser = pulldown_cmark::Parser::new_ext(&content, parser_options).map(|event| match event {
    pulldown_cmark::Event::Text(text) => {
      pulldown_cmark::Event::Text(text.replace("Peter", "John").into())
    }
    pulldown_cmark::Event::Start => {
      println!("Heading found")
    }
    _ => event,
  });

  // for idk in parser.into_offset_iter().collect::<Vec<_>>() {
  //   println!("{:#?}", &idk);
  // }

  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);
  fs::write(output_file, html_output).expect("Failed to write file");
  return;
}
