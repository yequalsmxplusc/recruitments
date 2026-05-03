use yew::prelude::*;
use crate::theme::ThemeContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: ThemeContext,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &Props) -> Html {
    let toggle_callback = {
        let toggle_fn = props.theme.toggle_theme.clone();
        Callback::from(move |_: MouseEvent| {
            toggle_fn();
        })
    };

    let label = if props.theme.is_dark() {
        "Switch to Light Mode"
    } else {
        "Switch to Dark Mode"
    };

    html! {
        <button
            onclick={toggle_callback}
            class="theme-toggle-btn p-1.5 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-all duration-200 border border-gray-300 dark:border-gray-600 flex items-center justify-center"
            title={label.to_string()}
        >
            {
                if props.theme.is_dark() {
                    html! {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white hover:text-black">
                            <circle cx="12" cy="12" r="5"></circle>
                            <line x1="12" y1="1" x2="12" y2="3"></line>
                            <line x1="12" y1="21" x2="12" y2="23"></line>
                            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                            <line x1="1" y1="12" x2="3" y2="12"></line>
                            <line x1="21" y1="12" x2="23" y2="12"></line>
                            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                        </svg>
                    }
                } else {
                    html! {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-black">
                            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                        </svg>
                    }
                }
            }
        </button>
    }
}
