use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::use_reducer;
use crate::auth::context::{AuthContext, AuthContextHandle};
use crate::routers::{Route, switch};

mod routers;
mod components{
    pub mod login;
    pub mod applicant_list;
    pub mod dashboard;
    pub mod reset;
    pub mod header;
    pub mod footer;
}
mod models{
    pub mod applicant;
}
mod services{
    pub mod api;
}
mod auth{
    pub mod context;
    pub mod models;
}
mod utils{}

#[function_component]
fn App() -> Html {
    let auth = use_reducer(AuthContext::default);
    let auth_handle = AuthContextHandle { inner: auth.clone() };

    let dark_mode = use_state(|| false);

    html! {
        <ContextProvider<AuthContextHandle> context={auth_handle}>
            <body class={if *dark_mode { "dark" } else { "" }}>
                <BrowserRouter>
                   <Switch<Route> render={move |routes: Route| switch(routes, AuthContextHandle { inner: auth_handle.clone() })} />
                </BrowserRouter>
            </body>
        </ContextProvider<AuthContextHandle>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}