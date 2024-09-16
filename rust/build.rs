// build.rs
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_language(0x0804);
        res.set("FileDescription", "Giwifi - github.com/mcitem");
        res.set("ProductName", "Giwifi");
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        res.set("LegalCopyright", "github.com/mcitem");
        res.compile().unwrap();
    }
}
