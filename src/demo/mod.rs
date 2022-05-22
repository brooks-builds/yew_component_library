pub mod pages;
pub mod router;

use crate::atoms::yr_link::YrLink;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ExampleApp)]
pub fn example_app() -> Hmtl {
    html! {
      <div>
        <BrowserRouter>
          <YrLink data_test="link-to-atoms" to={Route::Atoms}>{"Atom Components"}</YrLink>
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      </div>
    }
}
