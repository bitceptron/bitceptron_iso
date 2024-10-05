use dioxus::prelude::*;

use crate::ui::{
    asset_manager::AssetManager, components::{flag_bearer_header::FlagBearerHeader, sidebar::Sidebar}, route::Page
};

#[component]
pub(crate) fn HomePage() -> Element {
    rsx! {
        div { class: "flex flex-col bg-black w-screen h-screen",
            FlagBearerHeader {}
            div { class: "flex flex-row w-full h-full",
                Sidebar { current_page: Page::Home }
                div { class: "flex flex-col text-second_color text-center h-full w-full font-serif text-xl justify-center items-center content-center overflow-clip",
                    div { class: "flex flex-row items-center content-center",
                        "This is ISO by"
                        img {
                            class: "px-1 object-scale-down h-6 self-center items-center content-center",
                            src: AssetManager::get_bitceptron_logo(),
                        }
                    }
                    div { "Designed for air-gapped machines to keep your bitcoins safe." }
                }
            }
        }
    }
}
