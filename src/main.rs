#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
#![windows_subsystem = "windows"]

use bitceptron_iso::{app::App, ui::asset_manager::AssetManager};
use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};

fn main() {
    let window = WindowBuilder::new()
        .with_title("bitceptron ISO")
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(1200.0, 900.0))
        .with_min_inner_size(LogicalSize::new(650.0, 500.0));
    let head = format!(r#"<link rel="stylesheet" href="{}">"#, AssetManager::get_stylesheet());
    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(
            head,
        )
        .with_window(window);
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
