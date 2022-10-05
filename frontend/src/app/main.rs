// main.rs

use sycamore::prelude::*;

use frontend_lib::routes::router::SiteRouter;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            SiteRouter {}
        }
    });
}
