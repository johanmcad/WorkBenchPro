fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        // Icon
        res.set_icon("assets/icon.ico");

        // Version info
        res.set("FileVersion", "1.1.0.0");
        res.set("ProductVersion", "1.1.0.0");
        res.set("ProductName", "WorkBench-Pro");
        res.set("FileDescription", "WorkBench-Pro - Developer Workstation Benchmark");
        res.set("LegalCopyright", "Copyright © 2025 Johan Moreau");
        res.set("CompanyName", "Johan Moreau");
        res.set("InternalName", "workbench-pro");
        res.set("OriginalFilename", "workbench-pro.exe");

        // Compile the resource
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
        }
    }
}
