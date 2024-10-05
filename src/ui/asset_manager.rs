use dioxus::prelude::*;

pub struct AssetManager;

impl AssetManager {
    pub fn get_stylesheet() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/tailwind.css");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/tailwind.css".to_string()
        } else {
            asset!("assets/tailwind.css").to_string()
        }
    }

    pub fn get_stylesheet_href() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/tailwind.css");
            path
        } else if cfg!(target_os = "windows") {
            asset!("./assets/tailwind.css").to_string()
        } else {
            asset!("assets/tailwind.css").to_string()
        }
    }

    pub fn get_bitceptron_logo() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/logo_bitceptron.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/logo_bitceptron.png".to_string()
        } else {
            asset!("assets/logo_bitceptron.png").to_string()
        }
    }

    pub fn get_iso_logo() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/logo_iso.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/logo_iso.png".to_string()
        } else {
            asset!("assets/logo_iso.png").to_string()
        }
    }

    pub fn get_png_key_not_selected() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_key_not_selected.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_key_not_selected.png".to_string()
        } else {
            asset!("assets/app_png_key_not_selected.png").to_string()
        }
    }

    pub fn get_png_key_selected() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_key_selected.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_key_selected.png".to_string()
        } else {
            asset!("assets/app_png_key_selected.png").to_string()
        }
    }

    pub fn get_png_minus() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_minus.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_minus.png".to_string()
        } else {
            asset!("assets/app_png_minus.png").to_string()
        }
    }

    pub fn get_png_plus() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_plus.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_plus.png".to_string()
        } else {
            asset!("assets/app_png_plus.png").to_string()
        }
    }

    pub fn get_png_sign_not_selected() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_sign_not_selected.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_sign_not_selected.png".to_string()
        } else {
            asset!("assets/app_png_sign_not_selected.png").to_string()
        }
    }

    pub fn get_png_sign_selected() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_sign_selected.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_sign_selected.png".to_string()
        } else {
            asset!("assets/app_png_sign_selected.png").to_string()
        }
    }

    pub fn get_png_tychentropy_not_selected() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_tychentropy_not_selected.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_tychentropy_not_selected.png".to_string()
        } else {
            asset!("assets/app_png_tychentropy_not_selected.png").to_string()
        }
    }

    pub fn get_png_tychentropy_selected() -> String {
        if cfg!(target_os = "linux") {
            let mut path = env!("CARGO_MANIFEST_DIR").to_string();
            path.push_str("/assets/app_png_tychentropy_selected.png");
            path
        } else if cfg!(target_os = "windows") {
            "./assets/app_png_tychentropy_selected.png".to_string()
        } else {
            asset!("assets/app_png_tychentropy_selected.png").to_string()
        }
    }
}
