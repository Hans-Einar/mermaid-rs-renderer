fn main() {
    #[cfg(feature = "libavoid")]
    {
        let mut sources: Vec<_> = std::fs::read_dir("vendor/libavoid")
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x == "cpp"))
            .collect();
        sources.sort();
        println!("cargo:rerun-if-changed=vendor/libavoid");
        println!("cargo:rerun-if-changed=native/avoid.cpp");
        cc::Build::new()
            .cpp(true)
            .std("c++17")
            .include("vendor")
            .define("NDEBUG", None)
            .warnings(false)
            .files(sources)
            .file("native/avoid.cpp")
            .compile("mermaid_avoid");
    }
}
