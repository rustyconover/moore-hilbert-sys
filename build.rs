fn main() {
    cc::Build::new()
        .file("src/hilbert.c")
        .compile("moore-hilbert");
}
