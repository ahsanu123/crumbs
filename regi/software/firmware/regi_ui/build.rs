fn main() {
    unsafe {
        std::env::set_var("SLINT_FONT_SIZES", "14,18,24,30");
    }
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);

    slint_build::compile_with_config("./slints/regi.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();

    unsafe {
        std::env::remove_var("SLINT_FONT_SIZES");
    }
}
