use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::json;
use crate::auth::context::AuthAction;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: crate::auth::context::AuthContextHandle,
}

#[function_component]
pub fn Login(props: &Props) -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| None::<String>);

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
                match Request::post("http://localhost:8000/login")
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
        <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900">
            <div class="max-w-md w-full space-y-8 p-8 bg-white dark:bg-gray-800 rounded-lg shadow">
                <h2 class="text-3xl font-extrabold text-center text-gray-900 dark:text-white">
                    { "Coordinator Recruitment Portal 2025" }
                </h2>
                if let Some(err) = &*error {
                    <div class="text-red-500 text-center">{ err }</div>
                }
                <form class="mt-8 space-y-6" onsubmit={on_submit}>
                    <div class="rounded-md shadow-sm space-y-4">
                        <div>
                            <label for="username" class="sr-only">{"ID or Email address"}</label>
                            <input
                                id="username"
                                type="text"
                                required = true
                                class="appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-t-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm bg-white dark:bg-gray-700"
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
                                class="appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white rounded-b-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm bg-white dark:bg-gray-700"
                                placeholder="Password"
                                value={(*password).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                })}
                            />
                        </div>
                    </div>
                    <div>
                        <button
                            type="submit"
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:bg-primary-dark focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary"
                        >
                            { "Sign in" }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[derive(serde::Deserialize)]
struct LoginResponse {
    token: String,
}