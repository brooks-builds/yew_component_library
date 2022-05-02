use yew::prelude::*;
use yew_component_library::atoms::bb_text::{text_type::BBTextType, BBText};

#[function_component(ExampleApp)]
pub fn example_app() -> Hmtl {
    html! {
      <div>
        <BBText text="title text" data_test="title-text" text_type={BBTextType::Title} />
        <BBText text="paragraph text" data_test="normal-paragraph" />

      </div>
    }
}

fn main() {
    yew::start_app::<ExampleApp>();
}
