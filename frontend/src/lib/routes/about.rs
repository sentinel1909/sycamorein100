// about.rs

use sycamore::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view! {cx, 
        Header {}
        Navigation {}
        article {
            h2 { "About Me" }
            p { "This will eventually have some information about me." }
        }    
        Footer {}
    }
}