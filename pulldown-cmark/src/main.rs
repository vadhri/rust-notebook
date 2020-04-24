#![feature(proc_macro_hygiene)]

use std::io::Write;
use clap::{clap_app, crate_version};
use pulldown_cmark::{Parser, Options, html};
use std::fs;
use maud::html;
use std::fs::File;

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

fn main() {
    let args_handler = clap_app!(pull_down_cmark =>
        (version: crate_version!())
        (author: "vadhri")
        (@arg input:  -i --input +takes_value +required "Filename to look for mark up")
        (@arg events: -e --events "Sets a custom config file")
        (@arg output: -o --output +takes_value "output file to write html")
        (@arg css: -c --css +takes_value "path to css file")
    ).get_matches();

    let input_filename = args_handler.value_of("input").unwrap();
    let events_print = args_handler.is_present("events");
    let output_file_print = args_handler.is_present("output");
    let input_css = args_handler.is_present("css");

    let infile = fs::read_to_string(input_filename).expect("File not found !");
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

    let mut css: Option<&str> = None;

    if input_css {
        css = Some(args_handler.value_of("css").unwrap());
    }

    outcome = wrap_block_in_html(&outcome, css);

    if output_file_print {
        let output_filename = args_handler.value_of("output").unwrap();
        let mut output_file_fd = File::create(output_filename).unwrap();
        let _ignore = output_file_fd.write_all(&outcome.into_bytes());
    } else {
        println!("HTML -> {:?}", outcome);
    }
}
