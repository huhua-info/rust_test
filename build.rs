fn main() {
    slint_build::compile("ui/app.slint")
        .expect("無法編譯 Slint UI");
}