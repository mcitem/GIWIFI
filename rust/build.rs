// build.rs
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("main.manifest");
        res.compile().unwrap();
    }
}
