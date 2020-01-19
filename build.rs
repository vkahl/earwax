fn main() {
    //gcc::compile_library("libearwax.a", &["src/libearwax.c"]);
    cc::Build::new()
        .file("src/libearwax.c")
        .compile("libearwax.a");
}
