use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data_test: Option<String>,
    pub children: Children,
}

#[styled_component(SubTitle)]
pub fn sub_title(props: &Props) -> Html {
    let stylesheet = create_stylesheet();

    html! {
      <h2 data-test={props.data_test.clone()} class={stylesheet}>{props.children.clone()}</h2>
    }
}

fn create_stylesheet() -> Style {
    let css = format!(
        r#"
        font-size: 35px;
        margin: 10px 0;
    "#
    );

    Style::new(css).unwrap()
}
