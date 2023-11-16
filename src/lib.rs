mod reqwest_loader;

pub fn install(ctx: &egui::Context) {
    if !ctx.is_loader_installed(self::reqwest_loader::ReqwestLoader::ID) {
        ctx.add_bytes_loader(std::sync::Arc::new(
            self::reqwest_loader::ReqwestLoader::default(),
        ));
        log::trace!("installed ReqwestLoader");
    }
    let _ = ctx;
}
