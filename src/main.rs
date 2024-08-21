#![allow(non_snake_case)]

use daisy_rsx::components::avatar::Avatar;
use dioxus::prelude::*;

use crate::manganis;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(|| {
        rsx! {
            head::Link { rel: "stylesheet", href: _STYLE }
            Router::<Route> {}
        }
    });
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        // At "/blog", we want to show a list of blog posts
        #[route("/")]
        Home {},

        #[route("/avatar")]
        Avatar {},


}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav { id: "w-full navbar",
            div { class: "w-full flex flex-col justify-start h-screen",
                div { class: "navbar w-full flex flex-row bg-base-100",
                    div { class: "navbar-start",
                        div { class: "dropdown",
                            div {
                                role: "button",
                                tabindex: "0",
                                class: "btn btn-ghost btn-circle",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke: "currentColor",
                                    "viewBox": "0 0 24 24",
                                    fill: "none",
                                    class: "h-5 w-5",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-width": "2",
                                        "stroke-linejoin": "round",
                                        d: "M4 6h16M4 12h16M4 18h7"
                                    }
                                }
                            }
                            ul {
                                tabindex: "0",
                                class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow",
                                li {
                                    a { "Github" }
                                }
                                li {
                                    a { "Dioxus Github" }
                                }
                                li {
                                    a { "Daisy UI Github" }
                                }
                            }
                        }
                    }
                    div { class: "navbar-center",
                        a { class: "btn btn-ghost text-xl", "Dioxus ft. daisyUI" }
                    }
                    div { class: "navbar-end",
                        button { class: "btn btn-ghost btn-circle",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                stroke: "currentColor",
                                class: "h-5 w-5",
                                path {
                                    d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round"
                                }
                            }
                        }
                    }
                }
                div { class: "w-full flex flex-row justify-start items-start space-x-4",
                    ul { class: "flex flex-2 menu bg-base-200 rounded-box w-56 h-auto p-2 shadow-lg",
                        li {
                            // TODO: Will Render a markdown file for the installation guide here also use routing
                            a { "Installation Guide" }
                        }
                        li {
                            a { "Components" }
                            ul {
                                li {
                                    a { "Actions" }
                                    ul {
                                        li {
                                            a { "Button" }
                                        }
                                    }
                                }
                                li {
                                    a { "Data Display" }
                                    ul {
                                        li {
                                            a { "Avatar" }
                                        }
                                    }
                                }
                                li {
                                    a { "Navigation" }
                                    ul {
                                        li {
                                            a { "WIP" }
                                        }
                                    }
                                }
                                li {
                                    a { "Feedback" }
                                    ul {
                                        li {
                                            a { "WIP" }
                                        }
                                    }
                                }
                                li {
                                    a { "Data Input" }
                                    ul {
                                        li {
                                            a { "WIP" }
                                        }
                                    }
                                }
                                li {
                                    a { "Layout" }
                                    ul {
                                        li {
                                            a { "WIP" }
                                        }
                                    }
                                }
                                li {
                                    a { "Mockups" }
                                    ul {
                                        li {
                                            a { "WIP" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-1 flex-col justify-start items-start space-y-4",
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}

pub fn Home() -> Element {
    rsx!(
        div {
            h1 { "Welcome to Dioxus" }
            p { "This is a simple component library for daisyUI." }
        }
    )
}
