use glob::glob;

#[test]
fn ui() {
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
