// writing.rs

use sycamore::prelude::*;

use crate::components::header::Header;
use crate::components::footer::Footer;

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