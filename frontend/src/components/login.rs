use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::json;
use crate::auth::context::AuthAction;
use crate::theme::ThemeContext;
use crate::components::theme_toggle::ThemeToggle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: crate::auth::context::AuthContextHandle,
}

#[function_component]
pub fn Login(props: &Props) -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| None::<String>);
    let theme = use_context::<ThemeContext>().expect("ThemeContext not found");

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let auth = props.auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if username.is_empty() || password.is_empty() {
                error.set(Some("Username and password are required".to_string()));
                return;
            }

            let username_val = username.to_string();
            let password_val = password.to_string();
            let error = error.clone();
            let auth = auth.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match Request::post(&format!("{}/login", crate::services::api::get_api_base()))
                    .json(&json!({
                        "username": username_val,
                        "password": password_val,
                    }))
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.ok() {
                            if let Ok(login_response) = response.json::<LoginResponse>().await {
                                auth.dispatch(AuthAction::Login(
                                    login_response.token,
                                    username_val
                                ));
                            }
                        } else {
                            error.set(Some("Invalid credentials".to_string()));
                        }
                    }
                    Err(e) => error.set(Some(format!("Login failed: {}", e))),
                }
            });
        })
    };

    html! {
        <div class="flex flex-col min-h-screen theme-wrapper relative overflow-hidden">
            <main class="flex-grow flex items-center justify-center px-4 py-4">
                <div class="w-full max-w-md p-8 glass-card rounded-3xl shadow-2xl theme-card border">
                    <div class="flex flex-col items-center mb-8">
                      {
                          if theme.is_dark() {
                              html! {
                                  <img src="/static/assets/ecell-w.png" alt="JUECell Logo Dark" class="h-54 w-84 object-contain mb-8" />
                              }
                          } else {
                              html! {
                                  <img src="/static/assets/ecell.png" alt="JUECell Logo" class="h-54 w-84 object-contain mb-8" />
                              }
                          }
                      }
                     <h2 class="text-3xl font-extrabold text-center theme-text-primary">
                      { "Recruitments 2026" }
                     </h2>
                </div>
                    <p class="text-center text-sm mb-6 opacity-75">
                        { "Sign in to your account" }
                    </p>
                    if let Some(err) = &*error {
                        <div class="mb-4 p-3 bg-red-500/20 border border-red-500 rounded text-red-600 text-sm text-center">
                            { err }
                        </div>
                    }
                    <form class="space-y-5" onsubmit={on_submit}>
                        <div>
                            <label for="username" class="sr-only">{"ID or Email address"}</label>
                            <input
                                id="username"
                                type="text"
                                required = true
                                class="theme-input w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-offset-0 focus:outline-none text-sm"
                                placeholder="ID or Email Address"
                                value={(*username).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    username.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                })}
                            />
                        </div>
                        <div>
                            <label for="password" class="sr-only">{"Password"}</label>
                            <input
                                id="password"
                                type="password"
                                required =true
                                class="theme-input w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-offset-0 focus:outline-none text-sm"
                                placeholder="Password"
                                value={(*password).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                })}
                            />
                        </div>
                        <button
                            type="submit"
                            class="w-full py-3 px-4 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors duration-200 shadow-md hover:shadow-lg"
                        >
                            { "Sign In" }
                        </button>
                    </form>
                </div>
            </main>
            <footer class="theme-footer border py-6 px-4">
                <div class="max-w-7xl mx-auto flex flex-col sm:flex-row justify-between items-center space-y-4 sm:space-y-0">
                 <div class="flex flex-col sm:flex-row items-center space-y-4 sm:space-y-0 sm:space-x-6">
                    <p class="text-sm">
                        {"© 2026 "}
                        <a href="https://www.juecell.com/" class="hover:opacity-80 transition-opacity font-semibold">
                            {"Jadavpur University Entrepreneurship Cell"}
                        </a>
                        {". All rights reserved."}
                    </p>
                    <ThemeToggle theme={theme} />
                </div>
                    <ul class="flex space-x-4">
                        <li>
                            <a href="https://linkedin.com/school/juecell/" class="opacity-70 hover:opacity-100 transition-opacity" title="LinkedIn">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-linkedin" viewBox="0 0 16 16">
                                    <path d="M0 1.146C0 .513.526 0 1.175 0h13.65C15.474 0 16 .513 16 1.146v13.708c0 .633-.526 1.146-1.175 1.146H1.175C.526 16 0 15.487 0 14.854V1.146zm4.943 12.248V6.169H2.542v7.225h2.401zm-1.2-8.212c.837 0 1.358-.554 1.358-1.248-.015-.709-.52-1.248-1.342-1.248-.822 0-1.359.54-1.359 1.248 0 .694.521 1.248 1.327 1.248h.016zm4.908 8.212V9.359c0-.216.016-.432.08-.586.173-.431.568-.878 1.232-.878.869 0 1.216.662 1.216 1.634v3.865h2.401V9.25c0-2.22-1.184-3.252-2.764-3.252-1.274 0-1.845.7-2.165 1.193v.025h-.016a5.54 5.54 0 0 1 .016-.025V6.169h-2.4c.03.678 0 7.225 0 7.225h2.4z"/>
                                </svg>
                            </a>
                        </li>
                        <li>
                            <a href="https://twitter.com/ju_ecell" class="opacity-70 hover:opacity-100 transition-opacity" title="Twitter">
                                <svg class="bi" width="20" height="20" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84"/>
                                </svg>
                            </a>
                        </li>
                        <li>
                            <a href="https://instagram.com/juecell" class="opacity-70 hover:opacity-100 transition-opacity" title="Instagram">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-instagram" viewBox="0 0 16 16">
                                    <path d="M8 0C5.829 0 5.556.01 4.703.048 3.85.088 3.269.222 2.76.42a3.917 3.917 0 0 0-1.417.923A3.927 3.927 0 0 0 .42 2.76C.222 3.268.087 3.85.048 4.7.01 5.555 0 5.827 0 8.001c0 2.172.01 2.444.048 3.297.04.852.174 1.433.372 1.942.205.526.478.972.923 1.417.444.445.89.719 1.416.923.51.198 1.09.333 1.942.372C5.555 15.99 5.827 16 8 16s2.444-.01 3.298-.048c.851-.04 1.434-.174 1.943-.372a3.916 3.916 0 0 0 1.416-.923c.445-.445.718-.891.923-1.417.197-.509.332-1.09.372-1.942C15.99 10.445 16 10.173 16 8s-.01-2.445-.048-3.299c-.04-.851-.175-1.433-.372-1.941a3.926 3.926 0 0 0-.923-1.417A3.911 3.911 0 0 0 13.24.42c-.51-.198-1.092-.333-1.943-.372C10.443.01 10.172 0 7.998 0h.003zm-.717 1.442h.718c2.136 0 2.389.007 3.232.046.78.035 1.204.166 1.486.275.373.145.64.319.92.599.28.28.453.546.598.92.11.281.24.705.275 1.485.039.844.047 1.097.047 3.231s-.008 2.389-.047 3.232c-.035.78-.166 1.203-.275 1.485a2.47 2.47 0 0 1-.599.919c-.28.28-.546.453-.92.598-.28.11-.704.24-1.485.276-.843.038-1.096.047-3.232.047s-2.39-.009-3.233-.047c-.78-.036-1.203-.166-1.485-.276a2.478 2.478 0 0 1-.92-.598 2.48 2.48 0 0 1-.6-.92c-.109-.281-.24-.705-.275-1.485-.038-.843-.046-1.096-.046-3.233 0-2.136.008-2.388.046-3.231.036-.78.166-1.204.276-1.486.145-.373.319-.64.599-.92.28-.28.546-.453.92-.598.282-.11.705-.24 1.485-.276.738-.034 1.024-.044 2.515-.045v.002zm4.988 1.328a.96.96 0 1 0 0 1.92.96.96 0 0 0 0-1.92zm-4.27 1.122a4.109 4.109 0 1 0 0 8.217 4.109 4.109 0 0 0 0-8.217zm0 1.441a2.667 2.667 0 1 1 0 5.334 2.667 2.667 0 0 1 0-5.334z"/>
                                </svg>
                            </a>
                        </li>
                        <li>
                            <a href="https://facebook.com/juecell" class="opacity-70 hover:opacity-100 transition-opacity" title="Facebook">
                                <svg class="bi" width="20" height="20" fill="currentColor" viewBox="0 0 24 24">
                                    <path fillRule="evenodd" d="M22 12c0-5.523-4.477-10-10-10S2 6.477 2 12c0 4.991 3.657 9.128 8.438 9.878v-6.987h-2.54V12h2.54V9.797c0-2.506 1.492-3.89 3.777-3.89 1.094 0 2.238.195 2.238.195v2.46h-1.26c-1.243 0-1.63.771-1.63 1.562V12h2.773l-.443 2.89h-2.33v6.988C18.343 21.128 22 16.991 22 12z" clipRule="evenodd" />
                                </svg>
                            </a>
                        </li>
                    </ul>
                </div>
            </footer>
        </div>
    }
}

#[derive(serde::Deserialize)]
struct LoginResponse {
    token: String,
}