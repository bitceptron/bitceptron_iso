use dioxus::prelude::*;
use tychentropy::domain::statistics::statistical_tests_results::StatsTestState;

#[component]
pub fn StatsTestResultsIndicator(
    test_name: String,
    test_state: StatsTestState,
) -> Element {
    let (value, color) = match test_state {
        StatsTestState::NotAvailable => ("N/A", "bg-slate-500"),
        StatsTestState::Passed => ("Passed", " bg-green-500"),
        StatsTestState::Failed => ("Failed", "bg-red-700"),
    };

    rsx! {
        div { class: "h-12 w-56 flex flex-row items-center content-start justify-center",
            div { class: "font-serif mb-1 text-second_color self-center text-start h-12 w-28 justify-center items-center place-items-center content-center",
                "{test_name}:"
            }
            div { class: "{color} text-black text-center font-serif w-28 h-8 justify-center self-center items-center place-items-center content-center rounded-full",
                "{value}"
            }
        }
    }
}
