use dioxus::prelude::*;
use tychentropy::Tychentropy;

use crate::ui::{asset_manager::AssetManager, route::Route, view_model::tychentropy_page::TychentropyPageViewModel};

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(Tychentropy::default()));
    use_context_provider(|| TychentropyPageViewModel::default());
    rsx! {
        head::Link { rel: "stylesheet", href: AssetManager::get_stylesheet_href() }
        div { class: "flex flex-grow bg-black h-screen w-screen", Router::<Route> {} }
    }
}
