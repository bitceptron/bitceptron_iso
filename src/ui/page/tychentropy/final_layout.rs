use crate::ui::{
    components::{flag_bearer_header::FlagBearerHeader, sidebar::Sidebar},
    page::tychentropy::{
        data_entry_panel::TychentropyPageDataEntryPanel,
        outputs_panel::TychentropyPageOutputsPanel, settings_panel::TychentropyPageSettingsPanel,
    },
    route::Page,
};
use dioxus::prelude::*;

#[component]
pub(crate) fn TychentropyPage() -> Element {
    rsx! {
        div { class: "flex flex-col bg-black w-screen h-screen",
            FlagBearerHeader {}
            div { class: "flex flex-row items-start h-full w-full",
                Sidebar { current_page: Page::Tychentropy }
                div { class: "flex flex-col ml-3 mb-3 overflow-y-scroll w-full h-full",
                    TychentropyPageSettingsPanel {}
                    TychentropyPageDataEntryPanel {}
                    TychentropyPageOutputsPanel {}
                }
            }
        }
    }
}
