// footer.rs

use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        footer {
            p { "Copyright 2022 Jeffery D Mitchell All Rights Reserved" }
        }
    }
}
