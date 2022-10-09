// writing.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[component]
pub fn Writing<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        Header {}
        article {
            h2 { "Writing" }
            p { "My writings..." }
        }
        Footer {}
    }
}
