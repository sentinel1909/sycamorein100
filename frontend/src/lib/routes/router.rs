// router.rs

use sycamore::prelude::*;
use sycamore_router::{Route, Router, HistoryIntegration};

use super::home::Home;
use super::about::About;
use super::writing::Writing;
use super::projects::Projects;

#[derive(Route)]
enum SiteRoutes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[to("/writing")]
    Writing,
    #[to("/projects")]
    Projects,
    #[not_found]
    NotFound,
}

#[component]
pub fn SiteRouter<G: Html>(cx: Scope) -> View<G> {
    view! {cx, 
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<SiteRoutes>| {
                view! {cx, 
                    div {
                        (match route.get().as_ref() {
                            SiteRoutes::Home => view! { cx,
                                Home {}
                            },
                            SiteRoutes::About => view! { cx,
                                About {}
                            },
                            SiteRoutes::Writing => view! { cx,
                                Writing {}
                            },
                            SiteRoutes::Projects => view! { cx,
                                Projects {}
                            },
                            SiteRoutes::NotFound => view! { cx, 
                                "404 Not Found"
                            },
                        })
                    }
                }
            }
        )}
}
