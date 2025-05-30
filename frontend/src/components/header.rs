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
    let navigator_clone = navigator.clone(); //clones navigator to allow rerouting

    let on_logout = Callback::from(move |_| {
        auth.logout(); // Clear token and username
        navigator.push(&Route::Login); // Redirect to login page
    });
    let on_reset = Callback::from(move |_| {
        navigator_clone.push(&Route::Reset); // Use the cloned navigator
    });

    html! {
        <header class="w-full bg-gray-300 shadow-sm p-4 rounded-lg">
        <div class="flex justify-between items-center">
            <div class="text-xl font-semibold text-gray-800 flex items-center space-x-2">
                <img src="https://www.juecell.com/images/iicjuecell-logo.png" alt="Logo" class="h-10 w-10 hidden sm:inline" />
                <span>{"Recruitments 2025"}</span>
            </div>
            <div class="flex space-x-2">
                <button onclick={on_logout} class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-lg whitespace-nowrap">
                    {"Logout"}
                </button>
                <button onclick={on_reset} class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg whitespace-nowrap">
                    {"Reset"}
                </button>
            </div>
        </div>
    </header>
    }
}