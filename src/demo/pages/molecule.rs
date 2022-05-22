use crate::atoms::{page_title::PageTitle, sub_title::SubTitle};
use yew::prelude::*;

#[function_component(MoleculePage)]
pub fn molecule_page() -> Html {
    html! {
      <main>
        <PageTitle data_test="page-title" center={true}>{"Molecule Components"}</PageTitle>
        <section>
          <SubTitle data_test="navbar-title">{"Navbar"}</SubTitle>
        </section>
      </main>
    }
}
