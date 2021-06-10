// Hi, future me (or whoever you are)!
//
// Yes, we do need this attr.
// No, the warnings cannot be fixed otherwise.
// Accept and endure. Do not touch.
#![allow(unused)]

use std::io::{Cursor, Write};

use regex::Regex;

use clap::{App, IntoApp};

pub fn get_help<T: IntoApp>() -> String {
    let mut output = Vec::new();
    <T as IntoApp>::into_app().write_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!("\n%%% HELP %%%:=====\n{}\n=====\n", output);
    eprintln!("\n%%% HELP (DEBUG) %%%:=====\n{:?}\n=====\n", output);

    output
}

pub fn get_long_help<T: IntoApp>() -> String {
    let mut output = Vec::new();
    <T as IntoApp>::into_app()
        .write_long_help(&mut output)
        .unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!("\n%%% LONG_HELP %%%:=====\n{}\n=====\n", output);
    eprintln!("\n%%% LONG_HELP (DEBUG) %%%:=====\n{:?}\n=====\n", output);

    output
}

pub fn get_subcommand_long_help<T: IntoApp>(subcmd: &str) -> String {
    let mut output = Vec::new();
    <T as IntoApp>::into_app()
        .get_subcommands_mut()
        .find(|s| s.get_name() == subcmd)
        .unwrap()
        .write_long_help(&mut output)
        .unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!(
        "\n%%% SUBCOMMAND `{}` HELP %%%:=====\n{}\n=====\n",
        subcmd, output
    );
    eprintln!(
        "\n%%% SUBCOMMAND `{}` HELP (DEBUG) %%%:=====\n{:?}\n=====\n",
        subcmd, output
    );

    output
}

fn compare<S, S2>(l: S, r: S2) -> bool
where
    S: AsRef<str>,
    S2: AsRef<str>,
{
    let re = Regex::new("\x1b[^m]*m").unwrap();
    // Strip out any mismatching \r character on windows that might sneak in on either side
    let ls = l.as_ref().trim().replace("\r", "");
    let rs = r.as_ref().trim().replace("\r", "");
    let left = re.replace_all(&*ls, "");
    let right = re.replace_all(&*rs, "");
    let b = left == right;
    if !b {
        println!();
        println!("--> left");
        println!("{}", left);
        println!("--> right");
        println!("{}", right);
        println!("--")
    }
    b
}

pub fn compare_output(l: App, args: &str, right: &str, stderr: bool) -> bool {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    let res = l.try_get_matches_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    write!(&mut buf, "{}", err).unwrap();
    let content = buf.into_inner();
    let left = String::from_utf8(content).unwrap();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    compare(left, right)
}
