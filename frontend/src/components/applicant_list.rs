use yew::prelude::*;
use yew::functional::use_effect_with;
use crate::models::applicant::Applicant;
use crate::services::api;
use crate::auth::context::AuthContextHandle;
use crate::components::header::Header;
use crate::components::footer::Footer;
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
                match api::fetch_all_applicants(token).await {
                    Ok(fetched_applicants) => {
                        applicants.set(fetched_applicants);
                    }
                    Err(e) => {
                        if e == "not_admin" {
                            error.set(Some("not_admin".to_string()));
                        } else {
                            error.set(Some(e));
                        }
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
        <>
        <Header auth={props.auth.clone()}/>
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-8 text-gray-900 dark:text-white">{ "Applicants" }</h1>
            if let Some(err) = &*error {
                if err == "not_admin" {
                    <div class="text-center text-2xl text-red-600 font-semibold">
                        { "404: Page Not Found" }
                    </div>
                } else {
                    <div class="text-red-500 mb-4">{ err }</div>
                }
            } else {
                <div class="overflow-x-auto">
                    <table class="min-w-full bg-white dark:bg-gray-800">
                        <thead>
                            <tr>
                                <th class="py-2 px-4 border-b text-left text-gray-900 dark:text-white">{ "Name" }</th>
                                <th class="py-2 px-4 border-b text-left text-gray-900 dark:text-white">{ "Email" }</th>
                                <th class="py-2 px-4 border-b text-left text-gray-900 dark:text-white">{ "Contact Number" }</th>
                                <th class="py-2 px-4 border-b text-left text-gray-900 dark:text-white">{ "Department" }</th>
                                <th class="py-2 px-4 border-b text-left text-gray-900 dark:text-white">{ "Status" }</th>
                                <th class="py-2 px-4 border-b text-left text-gray-900 dark:text-white">{ "Actions" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for (*applicants).iter().map(|applicant| {
                                let update_applicant = update_applicant.clone();
                                let applicant = applicant.clone();
                                html! {
                                    <tr key={applicant.id.clone()}>
                                        <td class="py-2 px-4 border-b text-gray-900 dark:text-white">{ &applicant.name }</td>
                                        <td class="py-2 px-4 border-b text-gray-900 dark:text-white">{ &applicant.email }</td>
                                        <td class="py-2 px-4 border-b text-gray-900 dark:text-white">{ &applicant.contact_number }</td>
                                        <td class="py-2 px-4 border-b text-gray-900 dark:text-white">{ &applicant.department }</td>
                                        <td class="py-2 px-4 border-b text-gray-900 dark:text-white">
                                           {
                                               match applicant.status.as_str() {
                                                   "Selected" => "Yes",
                                                   "No longer in Consideration" => "No",
                                                   _ => "In Consideration",
                                               }
                                           }
                                    </td>
                                        <td class="py-2 px-4 border-b space-x-2">
                                         <button
                                             onclick={Callback::from({
                                                 let applicant = applicant.clone();
                                                 let update_applicant = update_applicant.clone();
                                                 move |_| {
                                                     let mut updated_applicant = applicant.clone();
                                                     updated_applicant.status = match applicant.status.as_str() {
                                                         "In Consideration" => "Selected".to_string(),
                                                         "Selected" => "No longer in Consideration".to_string(),
                                                         "No longer in Consideration" => "Selected".to_string(),
                                                         _ => "In Consideration".to_string(),
                                                     };
                                                     update_applicant.emit(updated_applicant);
                                                 }
                                             })}
                                             class="bg-primary text-white px-4 py-2 rounded hover:bg-primary-dark">
                                             { "Toggle" }
                                         </button>
                                         <button
                                             onclick={Callback::from({
                                                 let applicant = applicant.clone();
                                                 let update_applicant = update_applicant.clone();
                                                 move |_| {
                                                     let mut updated_applicant = applicant.clone();
                                                     updated_applicant.status = "In Consideration".to_string();
                                                     updated_applicant.is_selected = false; // Optional: if used
                                                     update_applicant.emit(updated_applicant);
                                                 }
                                             })}
                                             class="bg-gray-300 text-gray-800 px-4 py-2 rounded hover:bg-gray-400">
                                             { "Reset" }
                                         </button>
                                     </td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </div>
            }
        </div>
        <Footer/>
        </>
    }
}
