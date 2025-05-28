use yew::prelude::*;
use crate::services::api;
use crate::auth::context::AuthContextHandle;
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

        Callback::from(move |_| {
            let password = (*password).clone();
            let contact = (*contact).clone();
            let message = message.clone();
            let token = token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Fetch the current applicant (assuming it's a self-reset flow)
                match api::fetch_applicant(token.clone()).await {
                    Ok(mut applicant) => {
                        applicant.password = password.clone(); // ensure backend allows this
                        applicant.contact_number = contact.clone();

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
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-6">
            <div class="w-full max-w-md bg-white p-8 rounded shadow">
                <h2 class="text-xl font-bold mb-6 text-gray-800">{"Reset Password & Contact Number"}</h2>

                if let Some(msg) = &*message {
                    <div class="mb-4 text-center text-blue-600 font-semibold">{ msg }</div>
                }
                <form onsubmit={on_submit.reform(|e: SubmitEvent| {e.prevent_default();})}>
                    <div class="mb-4">
                        <label class="block text-gray-700 mb-2">{"New Password"}</label>
                        <input
                            type="password"
                            value={(*password).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                password.set(input.value());
                            })}
                            class="w-full px-4 py-2 border rounded"
                        />
                    </div>
                    <div class="mb-6">
                        <label class="block text-gray-700 mb-2">{"New Contact Number"}</label>
                        <input
                            type="text"
                            value={(*contact).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                contact.set(input.value());
                            })}
                            class="w-full px-4 py-2 border rounded"
                        />
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700"
                    >
                        { "Update Details" }
                    </button>
                </form>
            </div>
        </div>
    }
}