use cc;

fn main() {
    cc::Build::new()
        .file("src/example.c")
        .compile("foo");
}