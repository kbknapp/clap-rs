extern crate clap;

use clap::{App, Arg};

#[test]
fn opts_using_short() {
    let r = App::new("opts")
        .args(&mut [
            Arg::from_usage("-f [flag] 'some flag'"),
            Arg::from_usage("-c [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["myprog", "-f", "some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_space() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["myprog", "--flag", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_equals() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["myprog", "--flag=some", "--color=other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["myprog", "-f", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed2() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["myprog", "--flag=some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}
