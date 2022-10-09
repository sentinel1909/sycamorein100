// header.rs

use sycamore::prelude::*;
use crate::components::navigation::Navigation;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Navigation {}
    }
}
