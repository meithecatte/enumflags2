#[cfg(unix)]
#[test]
fn ui() {
    use glob::glob;
    use std::os::unix::ffi::OsStrExt;
    let t = trybuild::TestCases::new();
    for test in glob("ui/*.rs").unwrap() {
        let path = test.unwrap();
        match path.as_os_str().as_bytes() {
            b"ui/must_use_warning.rs" => t.pass(path),
            _ => t.compile_fail(path),
        }
    }
}

#[cfg(windows)]
#[test]
fn ui() {
    use glob::glob;
    let t = trybuild::TestCases::new();
    for test in glob("ui/*.rs").unwrap() {
        let path = test.unwrap();
        if path == std::path::Path::new("ui/must_use_warning.rs") {
            t.pass(path)
        } else {
            t.compile_fail(path)
        }
    }
}