use yew::prelude::*;
use yew_router::{navigator, prelude::*};
use crate::components::login::Login;
use crate::components::applicant_list::ApplicantList;
use crate::components::dashboard::Dashboard;
use crate::components::reset::Reset;
use crate::auth::context::AuthContextHandle;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/reset")]
    Reset,
}

pub fn switch(routes: Route, auth: AuthContextHandle) -> Html {
    match routes {
        Route::Login => {
            if auth.is_authenticated() {
                html! { <Redirect<Route> to={Route::Home} /> }
            } else {
                html! { <Login auth={auth} /> }
            }
        }
        Route::Home => {
            if !auth.is_authenticated() {
                html! { <Redirect<Route> to={Route::Login} /> }
            } else {
                html! { <Dashboard auth={auth} /> }
            }
        }
        Route::Admin => {
            if !auth.is_authenticated() {
                html! { <Redirect<Route> to={Route::Login} /> }
            } else {
                html! { <ApplicantList auth={auth} /> }
            }
        }
        Route::NotFound => html! {
            <div class="flex items-center justify-center h-screen bg-gray-100 dark:bg-gray-900">
                <h1 class="text-4xl font-bold text-gray-800 dark:text-white">{ "404 Not Found" }</h1>
            </div>
        },
        Route::Reset=> {
            if !auth.is_authenticated() {
                html! { <Redirect<Route> to={Route::Login} /> }
            } else {
                html! {<Reset auth={auth} /> } 
            }
        }
    }
}