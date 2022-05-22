use crate::{
    atoms::{page_title::PageTitle, sub_title::SubTitle, yr_link::YrLink},
    demo::router::Route,
};
use yew::prelude::*;

#[function_component(AtomPage)]
pub fn atom_page() -> Html {
    html! {
      <section>
        <PageTitle data_test={"page-title"} center={true}>{"Atom Components"}</PageTitle>
        <div>
          <SubTitle data_test="subtitle">{"Links"}</SubTitle>
          <div>
            <YrLink data_test="yew-router-link" to={Route::Home}>{"Yew Router link to another page"}</YrLink>
          </div>
        </div>
      </section>
    }
}
