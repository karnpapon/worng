use std::io::{ self, Read, Write};
use std::io::LineWriter;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()>{
  let args: Vec<String> = env::args().collect();
  let output = match args.len() {
    2 => &args[1],
    _ => { 
      writeln!(io::stdout(), "Usage: generate_ast <output directory>")?;
      std::process::exit(64);
    }
  };

  define_ast(
    output, 
    String::from("Expr"), 
    vec![
      "Binary   : Expr left, Token operator, Expr right",
      "Grouping : Expr expression",
      "Literal  : Object value",
      "Unary    : Token operator, Expr right"
    ]
  );

  Ok(())
}

fn define_ast(output_dir: &str, base_name: String, types: Vec<&str> ) -> Result<()> {
  let road_not_taken = b"struct 
  Somewhere ages and ages hence:";
  
  let path_name: String = format!("{}/{}.rs", output_dir , base_name);
  let path = Path::new(&path_name);
  let display = path.display();

  let mut file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", display, why),
    Ok(file) => file,
  };

  let mut file = LineWriter::new(file);

  file.write_fmt(format_args!("struct {} {{\n", String::from("Test")))?;
  file.write_fmt(format_args!(" fn test_function() {{\n"))?;
  file.write_fmt(format_args!(" }}\n"))?;
  file.write_fmt(format_args!("}} \n"))?;

  Ok(())

  // PrintWriter writer = new PrintWriter(path, "UTF-8");

  // writer.println("package com.craftinginterpreters.worng;");
  // writer.println();
  // writer.println("import java.util.List;");
  // writer.println();
  // writer.println("abstract class " + baseName + " {");

  // writer.println("}");
  // writer.close();
}