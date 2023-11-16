# egui_reqwest_loader

Adds support for loading images via HTTP using reqwest.

⚠ It shall be treated as mutually exclusive with `egui_extras/http`!

Use this crate over `egui_extras/http` if you use `reqwest`+`tokio` in the rest of your project (reduces dependencies).

First image loaded using this loader shall be created in thread that is in `tokio` runtime context
(, as GUI thread is rather not async this means ie. calling `let _rt_guard = RUNTIME.enter();` before running eframe app)

```toml
egui_reqwest_loader = { git="https://github.com/PingPongun/egui_reqwest_loader.git" }
```

```rs
egui_extras::install_image_loaders(egui_ctx);
egui_reqwest_loader::install(egui_ctx);
```
