fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        // Icon
        res.set_icon("assets/icon.ico");

        // Version info
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        res.set("ProductName", "WorkBench");
        res.set("FileDescription", "Developer Workstation Benchmark");
        res.set("LegalCopyright", "Copyright © 2025");
        res.set("CompanyName", "WorkBench");
        res.set("InternalName", "workbench");
        res.set("OriginalFilename", "workbench.exe");

        // Compile the resource
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
        }
    }
}
