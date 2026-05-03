use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::{use_reducer, use_effect_with};
use crate::auth::context::{AuthContext, AuthContextHandle};
use crate::routers::{Route, switch};
use crate::theme::{ThemeContext, ThemeMode};
use std::rc::Rc;

mod routers;
mod components{
    pub mod login;
    pub mod applicant_list;
    pub mod dashboard;
    pub mod reset;
    pub mod header;
    pub mod footer;
    pub mod applicant_form;
    pub mod case_study;
    pub mod interview_slots;
    pub mod rounds;
    pub mod theme_toggle;
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
mod theme;

#[function_component]
fn App() -> Html {
    let auth = use_reducer(AuthContext::default);
    let auth_handle = AuthContextHandle { inner: auth.clone() };

    let dark_mode = use_state(|| {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            storage.get_item("theme_mode").ok().flatten() == Some("dark".to_string())
        } else {
            false
        }
    });

    let toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let new_value = !*dark_mode;
            dark_mode.set(new_value);

            // Save to localStorage
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item(
                    "theme_mode",
                    if new_value { "dark" } else { "light" },
                );
            }

            // Update HTML class
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(html_element) = document.document_element() {
                    let class_list = html_element.class_list();

                    if new_value {
                        let _ = class_list.add_1("dark-mode");
                        let _ = class_list.remove_1("light-mode");
                    } else {
                        let _ = class_list.add_1("light-mode");
                        let _ = class_list.remove_1("dark-mode");
                    }
                }
            }
        })
    };

    // Initialize theme on mount
    {
        let dark_mode = dark_mode.clone();
        use_effect_with(dark_mode.clone(), move |_| {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(html_element) = document.document_element() {
                    let class_list = html_element.class_list();

                    if *dark_mode {
                        let _ = class_list.add_1("dark-mode");
                        let _ = class_list.remove_1("light-mode");
                    } else {
                        let _ = class_list.add_1("light-mode");
                        let _ = class_list.remove_1("dark-mode");
                    }
                }
            }
            || ()
        });
    }

    let theme_mode = if *dark_mode {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };

    let theme_context =
        ThemeContext::new(theme_mode, Rc::new(move || toggle_theme.emit(())));

    html! {
        <ContextProvider<AuthContextHandle> context={auth_handle.clone()}>
            <ContextProvider<ThemeContext> context={theme_context}>
                <div class="theme-wrapper">
                    <BrowserRouter>
                        <Switch<Route>
                            render={move |routes: Route| switch(routes, auth_handle.clone())}
                        />
                    </BrowserRouter>
                </div>
            </ContextProvider<ThemeContext>>
        </ContextProvider<AuthContextHandle>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}