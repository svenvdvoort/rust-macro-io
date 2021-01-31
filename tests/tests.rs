use macro_io::read_file;

#[test]
fn hello_world_file() {
    assert_eq!(read_file!("tests/testfiles/hello_world.txt"), "Hello world!");
}

#[test]
fn fmt_file() {
    assert_eq!(format!(read_file!("tests/testfiles/fmt.txt"), "foo", "bar"), "foo: bar");
}

#[test]
fn spaces_in_filename() {
    assert_eq!(read_file!("tests/testfiles/spaces filename.txt"), "Hello spaces!");
}

#[test]
fn compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_failures/*.rs");
}
