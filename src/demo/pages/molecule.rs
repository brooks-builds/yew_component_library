use crate::{
    atoms::{
        icon::image_icon::{ImageIcon, ImageIconName},
        page_title::PageTitle,
        sub_title::SubTitle,
        yr_link::YrLink,
    },
    demo::router::Route,
    molecules::navbar::NavBar,
};
use yew::prelude::*;

#[function_component(MoleculePage)]
pub fn molecule_page() -> Html {
    html! {
      <main>
        <PageTitle data_test="page-title" center={true}>{"Molecule Components"}</PageTitle>
        <section>
          <SubTitle data_test="navbar-title">{"Navbar"}</SubTitle>
          <div>
            <NavBar image_icon={ImageIconName::Brookzerker}>
              <YrLink data_test="navbar-icon" to={Route::Home}>
                <ImageIcon image={ImageIconName::Brookzerker} />
              </YrLink>
            </NavBar>
          </div>
        </section>
      </main>
    }
}
