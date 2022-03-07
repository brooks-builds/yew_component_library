use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BBLinkProps {}

#[function_component(BBLink)]
pub fn bb_link(props: &BBLinkProps) -> Html {
    html! {
        <a href="/">{"hello"}</a>
    }
}