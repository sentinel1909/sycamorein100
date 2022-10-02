// header.rs

use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header {
            h1 { "100 Days of Sycamore" }
        }
    }
}