use crate::demo::pages::{atom::AtomPage, home::HomePage, molecule::MoleculePage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/atoms")]
    Atoms,
    #[at("/molecules")]
    Molecules,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Atoms => html! { <AtomPage /> },
        Route::Molecules => html! { <MoleculePage /> },
    }
}
