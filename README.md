# rust-notebook
rust notebook with practise

## pulldown-cmark : 
The program would process multiple mark down files and process them to be HTML pages independently. 

### Instructions
#### Build 
cargo +nightly build --release

#### Help
pull_down_cmark 0.1.0
vadhri

USAGE:
    pulldown-cmark [FLAGS] [OPTIONS] --input <input>...

FLAGS:
    -e, --events     Sets a custom config to print events. (verbose - try to redirect to file.)
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --css <css>           path to css file
    -i, --input <input>...    The input filename to look for mark up. Use space seperate multiple items.
    -o, --output <output>     Output file to write html

