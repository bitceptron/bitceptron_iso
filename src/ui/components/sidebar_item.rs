use dioxus::prelude::*;

use crate::ui::route::Route;

#[component]
pub fn SidebarItem(
    text: String,
    link: Route,
    icon_not_selected: String,
    icon_selected: String,
    is_selected: bool,
) -> Element {
    rsx! {
        Link { class: "w-56 h-12", to: link,
            div { class: if is_selected { "flex flex-row bg-yellow-950 text-third_color font-bold text-lg w-full h-full hover:bg-yellow-950 font-serif content-center items-center px-2 border-r-4 border-third_color" } else { "flex flex-row text-second_color font-semibold text-base w-full h-full hover:bg-yellow-950 font-serif content-center items-center px-2" },
                img {
                    class: if is_selected { "h-10" } else { "h-9" },
                    src: if is_selected { icon_selected } else { icon_not_selected },
                }
                div { class: "pl-2", "{text}" }
            }
        }
    }
}
