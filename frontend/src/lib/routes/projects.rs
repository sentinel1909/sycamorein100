// writing.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[component]
pub fn Projects<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        Header {}
        article {
            h2 { "Projects" }
            p { "My projects..." }
        }
        Footer {}
    }
}
