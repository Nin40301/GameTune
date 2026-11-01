fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && let Err(error) =
            embed_resource::compile("helper.rc", embed_resource::NONE).manifest_required()
    {
        panic!("helper manifest: {error:?}");
    }
}
