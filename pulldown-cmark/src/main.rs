#![feature(proc_macro_hygiene)]

extern crate failure;

use std::io::Write;
use clap::{clap_app, crate_version};
use pulldown_cmark::{Parser, Options, html};
use std::fs;
use std::fmt;
use maud::html;
use std::fs::File;
use failure::{Fail};

pub struct AppErrors {
    code: String,
    reason: String
}

impl fmt::Display for AppErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.code, self.reason)
    }
}

impl fmt::Debug for AppErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug ({}, {})", self.code, self.reason)
    }
}

impl Fail for AppErrors {

}


impl From<std::io::Error> for AppErrors {
    fn from(error: std::io::Error) -> Self {
        AppErrors {
            code: "0".to_string(),
            reason: error.to_string()
        }
    }
}

fn wrap_block_in_html(partial_html: &str, css: Option<&str>) -> String {
    let res = html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";

                @if let Some(css_link) = css {
                    link rel="stylesheet" type="text/css" href=(css_link) {}
                }
            }
            body {
                (maud::PreEscaped(partial_html))

            }
        }
    };

    res.into_string()
}

pub fn read_file (input_filename: String) -> Result<String, AppErrors> {
    let string = fs::read_to_string(input_filename)?;
    Ok(string)
}

fn main() -> Result<(), AppErrors> {
    let args_handler = clap_app!(pull_down_cmark =>
        (version: crate_version!())
        (author: "vadhri")
        (@arg input:  -i --input +takes_value +required "Filename to look for mark up.")
        (@arg events: -e --events "Sets a custom config file for events.")
        (@arg output: -o --output +takes_value "Output file to write html")
        (@arg css: -c --css +takes_value "path to css file")
    ).get_matches();

    let input_filename = args_handler.value_of("input").unwrap();
    let events_print = args_handler.is_present("events");
    let output_file_print = args_handler.is_present("output");
    let input_css = args_handler.is_present("css");

    let infile = read_file(input_filename.to_string())?;

    let mut outcome = String::new();
    let mut css: Option<&str> = None;
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(&infile, options);

    if events_print {
        for item in parser.clone().into_iter() {
            println!("Event {:?}", item);
        }
    }

    html::push_html(&mut outcome, parser);

    if input_css {
        css = Some(args_handler.value_of("css").unwrap());
    }

    outcome = wrap_block_in_html(&outcome, css);

    if output_file_print {
        let output_filename = args_handler.value_of("output").unwrap();
        let mut output_file_fd = File::create(output_filename).unwrap();
        let _ignore = output_file_fd.write_all(&outcome.into_bytes());
    } else {
        println!("{:?}", outcome);
    }

    Ok(())
}
