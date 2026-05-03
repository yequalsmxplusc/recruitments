use crate::auth::context::AuthContextHandle;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::models::applicant::Applicant;
use crate::services::api;
use yew::functional::use_effect_with;
use yew::prelude::*;
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
                        if let Some(index) = updated_applicants
                            .iter()
                            .position(|a| a.id == updated_applicant.id)
                        {
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
        <div class="theme-wrapper min-h-screen flex flex-col">
            <Header auth={props.auth.clone()}/>
            <div class="container mx-auto px-4 py-8 flex-grow">
                <h1 class="text-3xl font-bold mb-8 theme-text-primary">{ "Applicants" }</h1>
                if let Some(err) = &*error {
                    if err == "not_admin" {
                        <div class="text-center text-2xl text-red-600 font-semibold">
                            { "404: Page Not Found" }
                        </div>
                    } else {
                        <div class="text-red-500 mb-4">{ err }</div>
                    }
                } else {
                    <div class="overflow-x-auto rounded-lg border theme-card">
                        <table class="min-w-full">
                            <thead>
                                <tr class="border-b theme-card">
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Name" }</th>
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Email" }</th>
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Contact Number" }</th>
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Department" }</th>
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Round" }</th>
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Status" }</th>
                                    <th class="py-3 px-4 text-left font-semibold theme-text-primary">{ "Actions" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for (*applicants).iter().map(|applicant| {
                                    let update_applicant = update_applicant.clone();
                                    let applicant = applicant.clone();
                                    let current_round_str = applicant.round.as_deref().unwrap_or("Round 1");
                                    let is_skill = applicant.skill.as_deref().map_or(false, |s| !s.is_empty());
                                    let stages = if is_skill {
                                        vec!["Round 1", "Case Study 1", "Case Study 2", "Interview"]
                                    } else {
                                        vec!["Round 1", "Case Study 1", "Interview"]
                                    };
                                    html! {
                                        <tr key={applicant.id.clone()} class="border-b hover:opacity-90 transition-opacity">
                                            <td class="py-3 px-4 theme-text-primary">{ &applicant.name }</td>
                                            <td class="py-3 px-4 theme-text-primary text-sm">{ &applicant.email }</td>
                                            <td class="py-3 px-4 theme-text-primary">{ &applicant.mobile.as_deref().unwrap_or("N/A") }</td>
                                            <td class="py-3 px-4 theme-text-primary">{ &applicant.department.as_deref().unwrap_or("N/A") }</td>
                                            <td class="py-3 px-4">
                                                <div class="flex items-center space-x-1">
                                                    { for stages.iter().map(|&stage| {
                                                        let is_active = stage == current_round_str;
                                                        let is_past = stages.iter().position(|&s| s == current_round_str).unwrap_or(0) > stages.iter().position(|&s| s == stage).unwrap_or(0);
                                                        let bg_color = if is_active { "bg-blue-500" } else if is_past { "bg-green-500" } else { "bg-gray-400 opacity-50" };
                                                        html! {
                                                            <div title={stage} class={classes!("h-2", "w-6", "rounded-full", bg_color)}></div>
                                                        }
                                                    })}
                                                    <span class="text-xs ml-2 opacity-75">{ current_round_str }</span>
                                                </div>
                                            </td>
                                            <td class="py-3 px-4 font-semibold theme-text-primary">
                                               { applicant.status.as_deref().unwrap_or("In Consideration") }
                                            </td>
                                            <td class="py-3 px-4 space-x-2">
                                             <button
                                                 onclick={Callback::from({
                                                     let applicant = applicant.clone();
                                                     let update_applicant = update_applicant.clone();
                                                     move |_| {
                                                         let mut updated_applicant = applicant.clone();
                                                         let current_round = applicant.round.as_deref().unwrap_or("Round 1");
                                                         let is_skill = applicant.skill.as_deref().map_or(false, |s| !s.is_empty());

                                                         let next_round = match current_round {
                                                             "Round 1" => "Case Study 1",
                                                             "Case Study 1" => if is_skill { "Case Study 2" } else { "Interview" },
                                                             "Case Study 2" => "Interview",
                                                             _ => current_round, // Reached the end
                                                         };

                                                         if next_round != current_round {
                                                             updated_applicant.round = Some(next_round.to_string());
                                                             update_applicant.emit(updated_applicant);
                                                         }
                                                     }
                                                 })}
                                                 class="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 text-sm rounded transition-colors duration-200">
                                                 { "Advance" }
                                             </button>
                                             <button
                                                 onclick={Callback::from({
                                                     let applicant = applicant.clone();
                                                     let update_applicant = update_applicant.clone();
                                                     move |_| {
                                                         let mut updated_applicant = applicant.clone();
                                                         updated_applicant.status = Some(match applicant.status.as_deref().unwrap_or("In Consideration") {
                                                             "In Consideration" => "Selected".to_string(),
                                                             "Selected" => "Rejected".to_string(),
                                                             "Rejected" => "In Consideration".to_string(),
                                                             _ => "In Consideration".to_string(),
                                                         });
                                                         update_applicant.emit(updated_applicant);
                                                     }
                                                 })}
                                                 class="bg-amber-600 hover:bg-amber-700 text-white px-3 py-1 text-sm rounded transition-colors duration-200">
                                                 { "Status" }
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
        </div>
        </>
    }
}
