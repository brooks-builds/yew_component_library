use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::atoms::icon::image_icon::ImageIconName;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub image_icon: ImageIconName,
    pub children: Children,
}

#[styled_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let stylesheet = create_stylesheet();
    let children = props.children.clone();

    html! {
      <header>
          <div class={classes!("icon", stylesheet)}>
            {children}
          </div>
      </header>
    }
}

fn create_stylesheet() -> Style {
    let css = format!(
        r#"
    display: flex;
    line-height: 25px;
  "#
    );

    Style::new(css).unwrap()
}
