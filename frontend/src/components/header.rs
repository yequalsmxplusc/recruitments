use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::auth::context::AuthContextHandle;
use crate::routers::Route;
use crate::theme::ThemeContext;
// use crate::components::theme_toggle::ThemeToggle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let auth = props.auth.clone();
    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone(); //clones navigator to allow rerouting
    let theme = use_context::<ThemeContext>().expect("ThemeContext not found");

    let on_logout = Callback::from(move |_| {
        auth.logout(); // Clear token and username
        navigator.push(&Route::Login); // Redirect to login page
    });
    let on_reset = Callback::from(move |_| {
        navigator_clone.push(&Route::Reset); // Use the cloned navigator
    });

    let logo_src = if theme.is_dark() {
        "/static/assets/wimage.png"
    } else {
        "/static/assets/heroicon.png"
    };

    html! {
        <header class="theme-header p-4">
            <div class="max-w-7xl mx-auto flex justify-between items-center">
                <div class="text-xl font-bold theme-text-primary flex items-center space-x-2 sm:space-x-4">
                    <img src={logo_src} alt="Logo" class="h-12 sm:h-16 w-auto" />
                    <span>{"Recruitments 2026"}</span>
                </div>
                <div class="flex space-x-3 items-center">
                    <button onclick={on_reset} class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium whitespace-nowrap transition-all duration-200 shadow-sm hover:shadow-md">
                        {"Reset"}
                    </button>
                    <button onclick={on_logout} class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-lg text-sm font-medium whitespace-nowrap transition-all duration-200 shadow-sm hover:shadow-md">
                        {"Logout"}
                    </button>
                </div>
            </div>
        </header>
    }
}