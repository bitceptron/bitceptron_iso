use dioxus::prelude::*;

use crate::ui::{
    asset_manager::AssetManager, components::sidebar_item::SidebarItem, route::{Page, Route}
};

// static _TAILWIND_CSS_STYLE: &str = manganis::mg!(file("public/tailwind.css"));

#[component]
pub(crate) fn Sidebar(current_page: Page) -> Element {
    let tychentropy_is_active = current_page == Page::Tychentropy;
    let _dummy1_is_active = current_page == Page::Dummy1;
    let _dummy2_is_active = current_page == Page::Dummy2;
    rsx! {
        div { class: "flex flex-col gap-8 w-60 p-4 justify-start h-fit",
            SidebarItem {
                text: String::from("Tychentropy"),
                link: Route::TychentropyPage {},
                icon_selected: AssetManager::get_png_tychentropy_selected(),
                icon_not_selected: AssetManager::get_png_tychentropy_not_selected(),
                is_selected: tychentropy_is_active,
            }
        }
    }
}
