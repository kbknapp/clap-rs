extern crate clap;
#[macro_use]
extern crate clap_macros;

use clap::code_gen::*;

// Create an application with 5 possible arguments (2 auto generated) and 2 subcommands (1 auto generated)
//    - A config file
//        + Uses "-c filename" or "--config filename"
//    - An output file
//        + A positional argument (i.e. "$ myapp output_filename")
//    - A debug flag
//        + Uses "-d" or "--debug"
//        + Allows multiple occurrences of such as "-dd" (for vary levels of debugging, as an example)
//    - A help flag (automatically generated by clap)
//        + Uses "-h" or "--help" (Only autogenerated if you do NOT specify your own "-h" or "--help")
//    - A version flag (automatically generated by clap)
//        + Uses "-V" or "--version" (Only autogenerated if you do NOT specify your own "-V" or "--version")
//    - A subcommand "test" (subcommands behave like their own apps, with their own arguments
//        + Used by "$ myapp test" with the following arguments
//            > A list flag
//                = Uses "-l" (usage is "$ myapp test -l"
//            > A help flag (automatically generated by clap
//                = Uses "-h" or "--help" (full usage "$ myapp test -h" or "$ myapp test --help")
//            > A version flag (automatically generated by clap
//                = Uses "-V" or "--version" (full usage "$ myapp test -V" or "$ myapp test --version")
//    - A subcommand "help" (automatically generated by clap because we specified a subcommand of our own)
//        + Used by "$ myapp help" (same functionality as "-h" or "--help")

/// Does awesome things.
#[derive(App, FromArgMatches)]
#[clap(name = "MyApp", version = "1.0")]
#[clap(author = "Nemo157 <clap@nemo157.com>")]
pub struct MyApp {
    /// Sets a custom config file
    #[clap(short = "c", value_name = "FILE")]
    config: Option<String>,

    /// Sets an optional output file
    #[clap(index = "1")]
    output: Option<String>,

    /// Turn debugging information on
    #[clap(counted, short = "d", long = "debug")]
    debug_level: u64,

    #[clap(subcommand)]
    subcommand: Option<Commands>,
}

#[derive(SubCommands)]
pub enum Commands {
    Test(Test),
}

/// does testing things
#[derive(App, FromArgMatches)]
pub struct Test {
    /// lists test values
    #[clap(short = "l")]
    list: bool,
}

fn main() {
    let app = MyApp::from(&MyApp::app().get_matches());

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = app.output {
        println!("Value for output: {}", o);
    }

    if let Some(c) = app.config {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match app.debug_level {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match app.subcommand {
        Some(Commands::Test(test)) => {
            if test.list {
                // "$ myapp test -l" was run
                println!("Printing testing lists...");
            } else {
                // "$ myapp test" was run
                println!("Not printing testing lists...");
            }
        }
        None => {
        }
    }

    // Continued program logic goes here...
}
