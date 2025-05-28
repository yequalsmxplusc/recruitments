use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::auth::context::AuthContextHandle;
use crate::routers::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let auth = props.auth.clone();
    let navigator = use_navigator().unwrap();

    let on_logout = Callback::from(move |_| {
        auth.logout(); // Clear token and username
        navigator.push(&Route::Login); // Redirect to login page
    });

    html! {
        <header class="w-full bg-gray-300 shadow-sm py-4 px-6 flex justify-between items-center rounded-lg">
    <div class="text-xl font-semibold text-gray-800 flex items-center space-x-2">
        <img src="https://www.juecell.com/images/iicjuecell-logo.png" alt="Logo" class="h-10 w-10" />
        <span>{ "Recruitments 2025" }</span>
    </div>
    <div class="flex space-x-3">
        <button onclick={on_logout} class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-lg">
            { "Logout" }
        </button>
        <button class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg">
            { "Reset" }
        </button>
    </div>
    </header>
    }
}