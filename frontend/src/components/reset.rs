use yew::prelude::*;
use crate::services::api;
use crate::auth::context::AuthContextHandle;
use crate::components::footer::Footer;
use crate::components::header::Header;
use web_sys::{HtmlInputElement, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component(Reset)]
pub fn reset(props: &Props) -> Html {
    let password = use_state(|| "".to_string());
    let contact = use_state(|| "".to_string());
    let message = use_state(|| None::<String>);
    let auth = props.auth.clone();
    let token = auth.token();

    let on_submit = {
        let password = password.clone();
        let contact = contact.clone();
        let message = message.clone();
        let token = token.clone();
        let _auth = auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let password = (*password).clone();
            let contact = (*contact).clone();
            let message = message.clone();
            let token = token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Fetch the current applicant (assuming it's a self-reset flow)
                match api::fetch_applicant(token.clone()).await {
                    Ok(mut applicant) => {
                        applicant.password = password.clone(); // ensure backend allows this
                        applicant.mobile = Some(contact.clone());

                        match api::update_applicant(&applicant, token).await {
                            Ok(_) => message.set(Some("Details updated successfully.".into())),
                            Err(e) => message.set(Some(format!("Failed to update: {}", e))),
                        }
                    }
                    Err(e) => message.set(Some(format!("Failed to load user: {}", e))),
                }
            });
        })
    };

    html! {
        <div class="flex flex-col items-center justify-center min-h-screen theme-wrapper">
            <Header auth={props.auth.clone()}/>
            <div class="w-full max-w-md p-8 glass-card rounded-2xl shadow-2xl mt-8">
                <h2 class="text-2xl font-bold mb-6 theme-text-primary text-center">{"Reset Password & Contact"}</h2>

                if let Some(msg) = &*message {
                    <div class="mb-4 p-3 bg-green-500/20 border border-green-500 rounded text-green-600 dark:text-green-400 font-semibold text-center text-sm">
                        { msg }
                    </div>
                }
                <form onsubmit={on_submit}>
                    <div class="mb-5">
                        <label class="block theme-text-primary font-medium mb-2 text-sm">{"New Password"}</label>
                        <input
                            type="password"
                            required=true
                            value={(*password).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                password.set(input.value());
                            })}
                            class="theme-input w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-offset-0 focus:outline-none text-sm"
                            placeholder="Enter new password"
                        />
                    </div>
                    <div class="mb-6">
                        <label class="block theme-text-primary font-medium mb-2 text-sm">{"Contact Number"}</label>
                        <input
                            type="text"
                            required=true
                            value={(*contact).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                contact.set(input.value());
                            })}
                            class="theme-input w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-offset-0 focus:outline-none text-sm"
                            placeholder="Enter contact number"
                        />
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-600 hover:bg-blue-700 text-white py-3 rounded-lg font-semibold transition-colors duration-200 shadow-md hover:shadow-lg"
                    >
                        { "Update Details" }
                    </button>
                </form>
            </div>
            <Footer/>
        </div>
    }
}