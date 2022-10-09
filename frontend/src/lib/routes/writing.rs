// writing.rs

use sycamore::prelude::*;

use crate::components::header::Header;
use crate::components::footer::Footer;

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