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

#[derive(Debug, Fail)]
pub enum ErrorEnum {
    #[fail(display = "Empty file provided - {} bytes", size)]
    EmptyFile {
        size: u32
    },
    #[fail(display = "Small file provided {} bytes", size)]
    SmallFile {
        size: u32
    },
    #[fail(display = "Medium size file provided {} bytes", size)]
    MediumSizeFile {
        size: u32
    },
    #[fail(display = "Large file provided {} bytes", size)]
    LargeFile {
        size: u32
    }
}

pub struct AppErrors {
    code: String,
    reason: String,
    file_size: ErrorEnum
}

impl fmt::Display for AppErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.code, self.reason, self.file_size)
    }
}

impl fmt::Debug for AppErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug ({}, {}, {})", self.code, self.reason, self.file_size)
    }
}

impl Fail for AppErrors {

}


impl From<std::io::Error> for AppErrors {
    fn from(error: std::io::Error) -> Self {
        AppErrors {
            code: "0".to_string(),
            reason: error.to_string(),
            file_size: ErrorEnum::EmptyFile {
                size: 0
            }
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
        (@arg input:  -i --input +takes_value +multiple +required "The input filename to look for mark up. Use space seperate multiple items.")
        (@arg events: -e --events "Sets a custom config to print events. (verbose - try to redirect to file.)")
        (@arg output: -o --output +takes_value "Output file to write html")
        (@arg css: -c --css +takes_value "path to css file")
    ).get_matches();

    let input_filenames = args_handler.values_of("input").unwrap();
    let events_print = args_handler.is_present("events");
    let output_file_print = args_handler.is_present("output");
    let input_css = args_handler.is_present("css");

    for item in input_filenames {
        println!("Processing ... {:?}", item.split('/').last().unwrap());
        let infile = read_file(item.to_string())?;

        let mut outcome = String::new();
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
            outcome = wrap_block_in_html(&outcome, Some(args_handler.value_of("css").unwrap()));
        }

        if output_file_print {
            let output_filename_provided = args_handler.value_of("output").unwrap().to_string();
            let mut output_filename_augmented_for_input = item.split('/').last().unwrap().to_string();
            output_filename_augmented_for_input.push('_');

            output_filename_augmented_for_input.push_str(&output_filename_provided);

            let mut output_file_fd = File::create(output_filename_augmented_for_input).unwrap();
            let _ignore = output_file_fd.write_all(&outcome.into_bytes());
        } else {
            println!("{:?}", outcome);
        }
    }

    Ok(())
}
