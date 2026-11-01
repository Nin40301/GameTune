fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_resource::compile("monitor.rc", embed_resource::NONE)
            .manifest_required()
            .unwrap_or_else(|e| panic!("monitor resource: {e}"));
    }
}
