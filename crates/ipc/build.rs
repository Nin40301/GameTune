fn main() {
    println!("cargo:rerun-if-env-changed=AMETISTA_HELPER_SHA256");
    if let Ok(hash) = std::env::var("AMETISTA_HELPER_SHA256") {
        assert!(
            hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()),
            "Invalid helper build hash"
        );
        println!(
            "cargo:rustc-env=AMETISTA_HELPER_SHA256={}",
            hash.to_ascii_lowercase()
        );
    }
}
