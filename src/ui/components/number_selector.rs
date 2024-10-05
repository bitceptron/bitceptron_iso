use dioxus::prelude::*;

use crate::ui::asset_manager::AssetManager;

#[component]
pub fn NumberSelector(
    title: String,
    comments: String,
    min: Option<i64>,
    max: Option<i64>,
    local_val_view_hook: Signal<u64>,
    core_val: u64,
    disabled: Signal<bool>,
) -> Element {
    let has_min = min.is_some();
    let has_max = max.is_some();
    let min = if has_min { min.unwrap() } else { i64::MIN };
    let max = if has_max { max.unwrap() } else { i64::MAX };
    rsx! {
        div { class: "flex flex-col flex-grow text-second_color items-start content-start p-3 justify-start self-center h-full w-full",
            div { class: "flex flex-row items-center h-8",
                div { class: "in_page_title_text h-8 text-center items-center justify-center content-center",
                    "{title}:"
                }
                div { class: "flex flex-row h-8",
                    button {
                        class: "flex p-2 h-8 w-10 rounded-l-lg border-second_color border-t-[1px] border-l-[1px] border-b-[1px] items-center content-center self-center active:bg-yellow-950 disabled:bg-disabled_area_color outline-none disabled:border-none",
                        disabled: disabled.read().to_owned(),
                        onclick: move |_| {
                            if local_val_view_hook.read().to_owned() > min as u64 {
                                local_val_view_hook -= 1;
                            }
                        },
                        if !disabled.read().to_owned() {
                            img {
                                class: "flex h-7 object-scale-down items-center content-center self-center justify-center border-none outline-none",
                                src: AssetManager::get_png_minus(),
                            }
                        }
                    }
                    input {
                        class: "h-8 w-14 bg-black text-second_color border-y-[1px] border-second_color text-center [&::-webkit-inner-spin-button]:appearance-none self-center focus:bg-yellow-800 focus:outline-none font-mono disabled:bg-disabled_area_color disabled:text-black disabled:border-none",
                        r#type: "number",
                        disabled: disabled.read().to_owned(),
                        min,
                        max,
                        value: if disabled.read().to_owned() { core_val as i64 } else { local_val_view_hook.read().to_owned() as i64 },
                        oninput: move |new_value| match new_value.value().parse::<i64>() {
                            Ok(value) => {
                                local_val_view_hook.set(value as u64);
                            }
                            Err(_) => {
                                let val = local_val_view_hook.read().to_owned();
                                local_val_view_hook.set(val);
                            }
                        },
                    }
                    button {
                        class: "flex p-2 h-8 w-10 rounded-r-lg border-second_color border-t-[1px] border-r-[1px] border-b-[1px] items-center content-center self-center active:bg-yellow-950 disabled:bg-disabled_area_color outline-none disabled:border-none",
                        disabled: disabled.read().to_owned(),
                        onclick: move |_| {
                            if local_val_view_hook.read().to_owned() < max as u64 {
                                local_val_view_hook += 1;
                            }
                        },
                        if !disabled.read().to_owned() {
                            img {
                                class: "flex h-7 object-scale-down items-center content-center self-center justify-center border-none outline-none",
                                src: AssetManager::get_png_plus(),
                            }
                        }
                    }
                }
            }
            div { class: "font-serif text-sm self-start justify-start w-full py-1 text-slate-400 overflow-y-scroll mt-2",
                "{comments}"
            }
        }
    }
}
