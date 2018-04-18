extern crate getopts;
extern crate csv_to_json_converter;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use getopts::Options;
use getopts::Matches;

use csv_to_json_converter::csv_to_json;
//use csv_to_json_converter::Args;
#[derive(Debug)]
struct Args {
     input: String,
     output: Option<String>,
     is_nulled: bool,
     is_keyed: bool,
}

fn get_file_names(input: String, output: Option<String>) -> (String, String) {
    if !input.contains(".csv") {
        panic!("src file is invalid. Should be specified and should contain the .csv extension!");
    }

    let src_file_name: String = input;
    let dest_file_name: String = {
        match output {
            Some(output_string) => {
                if !output_string.contains(".json") {
                    panic!("destination file is invalid. Should be specified and should contain the .json extension!");
                }
                output_string
            }
            None => {
                let splitted: Vec<&str> = src_file_name.split('.').collect();
                let mut dest_name = splitted[0].to_string();
                dest_name.push_str(".json");
                dest_name.to_owned()
            }
        }
    };

    (src_file_name, dest_file_name)
}

fn get_args(arg_strings: &[String]) -> Option<Args> {
    let mut opts: Options = Options::new();
    opts.optopt(
        "o",
        "",
        "The path of the output file including the file extension.",
        "TARGET_FILE_NAME",
    );
    opts.optflag("n", "null", "Empty strings are set to null.");
    opts.optflag("k", "keyed", "Generate output as keyed JSON.");
    opts.optflag("h", "help", "Prints this help menu.");
    let matches: Matches = match opts.parse(&arg_strings[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let program: String = arg_strings[0].clone();
    let mut is_nulled: bool = false;
    let mut is_keyed: bool = false;

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return None;
    }
    if matches.opt_present("k") {
        is_keyed = true;
    }
    if matches.opt_present("n") {
        is_nulled = true;
    }

    let output: Option<String> = matches.opt_str("o");
    let input: String = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return None;
    };
    Some(Args {
        input,
        output,
        is_nulled,
        is_keyed,
    })
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} SOURCE_FILE_NAME [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let arg_strings: Vec<String> = env::args().collect();
    let args: Args = match get_args(&arg_strings) {
        Some(args) => args,
        None => {
            return;
        }
    };
    println!("{:?}", args);

    let (src_file_name, dest_file_name) =
        get_file_names(args.input.to_owned(), args.output.to_owned());

    println!("src_file_name: {}", src_file_name);
    println!("dest_file_name: {}\n", dest_file_name);

    let mut src_file: File = File::open(src_file_name).expect("File not found");

    let mut contents: String = String::new();
    src_file
        .read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
  
    let json = csv_to_json(&mut contents, args.is_keyed, args.is_nulled);

    let mut dest_file: File = File::create(&dest_file_name)
        .expect(&format!("Error creating the file: {}", dest_file_name)[..]);
    dest_file
        .write_all(json.to_string().as_bytes())
        .expect(&format!("Error writing to file: {}", dest_file_name)[..]);

    println!("Successfully wrote to file {}", dest_file_name);
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_args_test() {
        let arg_strings: Vec<String> = vec![String::from("path"), String::from("csv.csv")];
        let args: super::Args = super::get_args(&arg_strings).unwrap();
        assert_eq!(args.input, "csv.csv");
        assert_eq!(args.output, None);
        assert_eq!(args.is_nulled, false);

        let arg_strings: Vec<String> = vec![
            String::from("path"),
            String::from("csv.csv"),
            String::from("-o"),
            String::from("csv.json"),
        ];
        let args: super::Args = super::get_args(&arg_strings).unwrap();
        assert_eq!(args.input, "csv.csv");
        assert_eq!(args.output, Some(String::from("csv.json")));
        assert_eq!(args.is_nulled, false);

        let arg_strings: Vec<String> = vec![
            String::from("path"),
            String::from("csv.csv"),
            String::from("-n"),
        ];
        let args: super::Args = super::get_args(&arg_strings).unwrap();
        assert_eq!(args.input, "csv.csv");
        assert_eq!(args.output, None);
        assert_eq!(args.is_nulled, true);
    }

 
    #[test]
    fn file_names() {
        let (src, dest) =
            super::get_file_names(String::from("csv.csv"), Some(String::from("csv.json")));
        assert_eq!(src, "csv.csv");
        assert_eq!(dest, "csv.json");

        // If no dest file name is specified
        let (src, dest) = super::get_file_names(String::from("csv.csv"), None);
        assert_eq!(src, "csv.csv");
        assert_eq!(dest, "csv.json");
    }

    #[test]
    #[should_panic]
    fn panic_no_extensions() {
        super::get_file_names(String::from("csv"), Some(String::from("csv")));
    }

    #[test]
    #[should_panic]
    fn panic_diff_file_names() {
        super::get_file_names(String::from("csv.json"), Some(String::from("csv.csv")));
    }
}
