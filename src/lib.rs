#![crate_type = "lib"]
#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_results,
    unused_must_use,
    unused_macros
)]

/// rust compiler get version
/// 
/// 
fn rustc_get_version() -> String {
    let cmd_output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Unable to execute command");

    let v = String::from_utf8(cmd_output.stdout).unwrap_or_else(|_| String::new());
    let re = regex::Regex::new(
        r"rustc (?P<version>\d+\.\d+\.\d+) \((?P<commit>[^ ]+) (?P<date>[^\)]+)\)\n?$",
    )
    .unwrap();
    let captures = re.captures(&v).unwrap();

    String::from(
        captures
            .name("version")
            .map(|m| m.as_str())
            .unwrap_or_else(|| "0.0.0"),
    )
}

#[test]
fn rustc_get_version_test() {
    let v = rustc_get_version();

    println!("{}", &v);

    assert_ne!("0.0.0", &v)
}