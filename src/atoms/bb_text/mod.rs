pub mod text_type;

use yew::prelude::*;

use self::text_type::BBTextType;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub text_type: Option<BBTextType>,
}

#[function_component(BBText)]
pub fn bb_text(props: &Props) -> Html {
    let text_type = props.text_type.clone().unwrap_or_default();

    match text_type {
        BBTextType::Normal => html! { <p>{&props.text}</p> },
        BBTextType::Title => html! { <h1>{&props.text}</h1> },
    }
}
