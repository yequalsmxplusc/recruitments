use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::login::Login;
use crate::components::applicant_list::ApplicantList;
use crate::components::dashboard::Dashboard;
use crate::components::rounds::Rounds;
use crate::components::reset::Reset;
use crate::auth::context::AuthContextHandle;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("/sudoadmin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/reset")]
    Reset,
    #[at("/rounds")]
    Rounds,
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
            <div class="flex items-center justify-center h-screen theme-wrapper">
                <div class="text-center">
                    <h1 class="text-6xl font-bold theme-text-primary mb-4">{ "404" }</h1>
                    <p class="text-2xl theme-text-primary opacity-75 mb-6">{ "Page Not Found" }</p>
                    <Link<Route> to={Route::Home} classes="bg-blue-600 hover:bg-blue-700 text-white py-3 px-6 rounded-lg transition-colors duration-200 inline-block">
                        { "Return to Home" }
                    </Link<Route>>
                </div>
            </div>
        },
        Route::Reset=> {
            if !auth.is_authenticated() {
                html! { <Redirect<Route> to={Route::Login} /> }
            } else {
                html! {<Reset auth={auth} /> } 
            }
        }
        Route::Rounds => {
            if !auth.is_authenticated() {
                html! { <Redirect<Route> to={Route::Login} /> }
            } else {
                html! { <Rounds auth={auth} /> }
            }
        }
    }
}