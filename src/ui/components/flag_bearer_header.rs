use dioxus::prelude::*;

use crate::ui::{asset_manager::AssetManager, route::Route};

#[component]
pub fn FlagBearerHeader() -> Element {
    rsx! {
        div { class: "bg-black flex w-full h-fit p-4 content-center items-start justify-start object-center",
            Link { to: Route::HomePage {},
                img {
                    class: "object-scale-down h-10",
                    src: AssetManager::get_iso_logo(),
                }
            }
        }
    }
}
