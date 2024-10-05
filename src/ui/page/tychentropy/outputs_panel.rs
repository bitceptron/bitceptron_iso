use std::str::FromStr;

use dioxus::prelude::*;
use tychentropy::{
    domain::statistics::statistical_tests_results::StatisticalTestsOverview, Tychentropy,
};

use crate::ui::{
    components::stats_test_result_indicator::StatsTestResultsIndicator,
    view_model::tychentropy_page::TychentropyPageViewModel,
};

#[component]
pub fn TychentropyPageOutputsPanel() -> Element {
    let range_and_target_entropy_bytes_accepted_in_ui =
        use_context::<TychentropyPageViewModel>().range_and_target_entropy_bytes_accepted;
    let sequence = use_context::<Signal<Tychentropy>>()
        .read()
        .get_sequence()
        .to_owned();
    let sequence_display = match sequence.is_empty() {
        false => format!("{sequence:?}"),
        true => String::from_str("None").unwrap(),
    };
    let final_entropy_bytes = use_context::<Signal<Tychentropy>>()
        .read()
        .get_final_entropy_bytes_vector()
        .to_owned();
    let final_entropy_bytes_display = match final_entropy_bytes.is_empty() {
        false => format!("{final_entropy_bytes:?}"),
        true => String::from_str("None").unwrap(),
    };
    let mnemonic = match use_context::<Signal<Tychentropy>>().read().get_mnemonic() {
        Some(mnemonic) => mnemonic.to_string(),
        None => String::from_str("None").unwrap(),
    };
    let overview: StatisticalTestsOverview = use_context::<Signal<Tychentropy>>()
        .read()
        .get_statistical_test_results()
        .into();
    let approximate_entropy_test_m2_result = overview.get_approximate_entropy_test_m2_result();
    let approximate_entropy_test_m3_result = overview.get_approximate_entropy_test_m3_result();
    let block_frequency_test_result = overview.get_block_frequency_test_result();
    let cumulative_sums_test_result = overview.get_cumulative_sums_test_result();
    let frequency_test_result = overview.get_frequency_test_result();
    let longest_run_of_ones_test_result = overview.get_longest_run_of_ones_test_result();
    let runs_test_result = overview.get_runs_test_result();
    let serial_test_pattern_size_6_result = overview.get_serial_test_pattern_size_6_result();
    rsx! {
        div { class: "flex flex-col content-start justify-start items-start w-full h-full mt-6 gap-2",
            div { class: "px-3 justify-items-stretch w-full",
                div { class: "flex flex-row",
                    div { class: "in_page_title_text w-48", "Random data sequence:" }
                    div { class: "px-3 text-second_color bg-black h-16 w-full border-second_color border-[1px] rounded-lg blur-[4px] hover:blur-none overflow-y-scroll transition",
                        "{sequence_display}"
                    }
                }
            }
            div { class: "px-3 w-full",
                div { class: "flex flex-row w-full",
                    div { class: "in_page_title_text w-48", "Final entropy bytes:" }
                    div { class: "flex flex-row w-full justify-items-stretch",
                        div { class: "px-3 text-second_color w-full bg-black h-16 border-second_color border-[1px] rounded-tl-lg rounded-bl-lg blur-[5px] hover:blur-none overflow-y-scroll transition",
                            "{final_entropy_bytes_display}"
                        }
                        button {
                            class: "bg-button_color_enabled text-black hover:bg-button_color_hover disabled:bg-button_color_disabled disabled:text-slate-400 font-serif text-center p-1 transition border-r-[1px] border-black h-16 w-56",
                            disabled: !*use_context::<Signal<Tychentropy>>().read().get_is_entropy_ready()
                                || !range_and_target_entropy_bytes_accepted_in_ui.read().to_owned(),
                            onclick: move |_| {
                                let _ = use_context::<Signal<Tychentropy>>().write().mix_with_rng();
                            },
                            "Mix sequence with RNG"
                        }
                        button {
                            class: "bg-button_color_enabled text-black hover:bg-button_color_hover disabled:bg-button_color_disabled disabled:text-slate-400 font-serif text-center p-1 transition rounded-tr-lg rounded-br-lg  h-16 w-56",
                            disabled: use_context::<Signal<Tychentropy>>()
                                .read()
                                .get_rng_entropy_bytes_vector()
                                .is_empty(),
                            onclick: move |_| {
                                use_context::<Signal<Tychentropy>>()
                                    .write()
                                    .recover_original_entropy_bytes_after_mix()
                            },
                            "Recover Original Sequence"
                        }
                    }
                }
            }
            div { class: "px-3 justify-items-stretch w-full",
                div { class: "flex flex-row",
                    div { class: "in_page_title_text w-48", "Mnemonic:" }
                    div { class: "px-3 text-second_color bg-black h-16 w-full border-second_color border-[1px] rounded-lg blur-[7px] hover:blur-none overflow-y-scroll transition",
                        "{mnemonic}"
                    }
                }
            }
            div { class: "px-3 justify-items-stretch w-full h-full",
                div { class: "flex flex-row",
                    div { class: "in_page_title_text w-48", "Statistical tests results:" }
                    div { class: "px-3 text-second_color bg-black h-full w-full overflow-y-scroll transition",
                        div { class: "flex flex-wrap gap-5 w-full h-full",
                            StatsTestResultsIndicator {
                                test_name: "Approximate Entropy m2",
                                test_state: approximate_entropy_test_m2_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Approximate Entropy m3",
                                test_state: approximate_entropy_test_m3_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Block Frequency",
                                test_state: block_frequency_test_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Cumulative Sums",
                                test_state: cumulative_sums_test_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Frequency",
                                test_state: frequency_test_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Longest Runs of Ones",
                                test_state: longest_run_of_ones_test_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Runs",
                                test_state: runs_test_result.clone(),
                            }
                            StatsTestResultsIndicator {
                                test_name: "Serial",
                                test_state: serial_test_pattern_size_6_result.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
