use stylist::Style;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub data_test: Option<String>,
    pub children: Children,
    pub center: Option<bool>,
}

#[function_component(PageTitle)]
pub fn page_title(props: &Props) -> Html {
    html! {
        <h1 data-test={props.data_test.clone()} class={create_stylesheet(props.center.unwrap_or_default())}>{props.children.clone()}</h1>
    }
}

fn create_stylesheet(center: bool) -> Style {
    let text_align = if center { "center" } else { "left" };

    let css = format!(
        r#"
            font-size: 50px;
            text-align: {text_align};
        "#
    );
    Style::new(css).unwrap()
}
