fn main() {
    cc::Build::new()
        .file("blocklib/block.c")
        .include("blocklib")
        .compile("block");
}
