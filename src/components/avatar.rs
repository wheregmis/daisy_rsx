#![allow(non_snake_case)]
#![allow(unused_braces)]
use dioxus::prelude::*;

pub fn Avatar() -> Element {
    let mut active_tab = use_signal(|| "Preview".to_string());
    let code: String = "test".to_string();

    rsx!(
        div {
            h1 { "Welcome to Dioxus" }
            p { "This is a simple component library for daisyUI." }
        }
        div { role: "tablist", class: "tabs tabs-lifted",
            a {
                role: "tab",
                class: if active_tab() == "Preview" { "tab tab-active" } else { "tab" },
                onclick: move |_| active_tab.set("Preview".to_string()),
                "Preview"
            }
            a {
                role: "tab",
                class: if active_tab() == "Code" { "tab tab-active" } else { "tab" },
                onclick: move |_| active_tab.set("Code".to_string()),
                "Code"
            }
        }
        if active_tab() == "Preview" {
            div { class: "avatar",
                div { class: "ring-primary ring-offset-base-100 w-24 rounded-full ring ring-offset-2",
                    img { src: "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" }
                }
            }
        } else {
            div { "This is the code content." }
        }
    )
}
