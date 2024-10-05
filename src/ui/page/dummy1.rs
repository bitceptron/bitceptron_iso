use dioxus::prelude::*;

use crate::ui::{
    components::{flag_bearer_header::FlagBearerHeader, sidebar::Sidebar},
    route::Page,
};

#[component]
pub(crate) fn Dummy1() -> Element {
    rsx! {
        div { class: "bg-black h-screen w-screen",
            FlagBearerHeader {}
            Sidebar { current_page: Page::Dummy1 }
        }
    }
}
