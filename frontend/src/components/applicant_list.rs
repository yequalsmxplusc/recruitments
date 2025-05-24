use yew::prelude::*;
use yew::functional::use_effect_with;
use crate::models::applicant::Applicant;
use crate::services::api;
use crate::auth::context::AuthContextHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component]
pub fn ApplicantList(props: &Props) -> Html {
    let applicants = use_state(|| Vec::<Applicant>::new());
    let error = use_state(|| None::<String>);

    {
        let applicants = applicants.clone();
        let error = error.clone();
        let token = props.auth.token();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_applicants(token).await {
                    Ok(fetched_applicants) => {
                        applicants.set(fetched_applicants);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
            });
        });
    }

    let update_applicant = {
        let applicants = applicants.clone();
        let error = error.clone();
        let token = props.auth.token();

        Callback::from(move |applicant: Applicant| {
            let applicants = applicants.clone();
            let error = error.clone();
            let token = token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::update_applicant(&applicant, token).await {
                    Ok(updated_applicant) => {
                        let mut updated_applicants = (*applicants).clone();
                        if let Some(index) = updated_applicants.iter().position(|a| a.id == updated_applicant.id) {
                            updated_applicants[index] = updated_applicant;
                            applicants.set(updated_applicants);
                        }
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
            });
        })
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-8 text-gray-900 dark:text-white">{ "Applicants" }</h1>
            if let Some(err) = &*error {
                <div class="text-red-500 mb-4">{ err }</div>
            }
            <div class="overflow-x-auto">
                <table class="min-w-full bg-white dark:bg-gray-800">
                    <thead>
                        <tr>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "Name" }</th>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "id" }</th>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "Contact Number" }</th>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "Department" }</th>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "Date" }</th>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "Selected" }</th>
                            <th class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-left text-gray-900 dark:text-white">{ "Actions" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*applicants).iter().map(|applicant| {
                            let update_applicant = update_applicant.clone();
                            let applicant = applicant.clone();
                            html! {
                                <tr key={applicant.id.clone()}>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-gray-900 dark:text-white">{ &applicant.name }</td>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-gray-900 dark:text-white">{ &applicant.email }</td>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-gray-900 dark:text-white">{ &applicant.contact_number }</td>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-gray-900 dark:text-white">{ &applicant.department }</td>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-gray-900 dark:text-white">{ &applicant.date }</td>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700 text-gray-900 dark:text-white">
                                        { if applicant.is_selected { "Yes" } else { "No" } }
                                    </td>
                                    <td class="py-2 px-4 border-b border-gray-200 dark:border-gray-700">
                                        <button
                                            onclick={Callback::from(move |_| {
                                                let mut updated_applicant = applicant.clone();
                                                updated_applicant.is_selected = !updated_applicant.is_selected;
                                                update_applicant.emit(updated_applicant);
                                            })}
                                            class="bg-primary text-white px-4 py-2 rounded hover:bg-primary-dark"
                                        >
                                            { "Toggle Selection" }
                                        </button>
                                    </td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
