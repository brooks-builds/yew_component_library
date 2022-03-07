use yew::prelude::*;
use yew_component_library::atoms::bb_link::BBLink;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <BBLink />
        </div>
    }
}