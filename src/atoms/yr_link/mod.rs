use crate::demo::router::Route;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

const RAW_CSS: &str = include_str!("yr_link.css");

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data_test: Option<String>,
    pub children: Children,
    pub to: Route,
}

///
#[styled_component(YrLink)]
pub fn bb_link(props: &Props) -> Html {
    let data_test = props.data_test.clone().unwrap_or_default();

    html! {
      <span data-test={data_test} class={classes!(Style::new(RAW_CSS).unwrap(), "yr-link")}>
        <Link<Route> to={props.to}>{props.children.clone()}</Link<Route>>
      </span>
    }
}
