// navigation.rs

use sycamore::prelude::*;

#[component]
pub fn Navigation<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        nav {
            ul {
                li {
                    a(href="/") { "Home" }
                }
                li {
                    a(href="/about") { "About" }
                }
            }
        }
    }
}