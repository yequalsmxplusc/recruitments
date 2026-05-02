use yew::prelude::*;
use yew_router::prelude::*;
use crate::models::applicant::Applicant;
use crate::services::api;
use crate::auth::context::AuthContextHandle;
use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::applicant_form::ApplicantForm;
use crate::components::case_study::CaseStudySubmission;
use crate::components::interview_slots::InterviewSlotBooking;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component(Rounds)]
pub fn rounds(props: &Props) -> Html {
    let applicant = use_state(|| None::<Applicant>);
    let error = use_state(|| None::<String>);
    let token = props.auth.token();

    {
        let applicant = applicant.clone();
        let error = error.clone();
        let token = token.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_applicant(token).await {
                    Ok(data) => applicant.set(Some(data)),
                    Err(e) => error.set(Some(e)),
                }
            });

            || ()
        });
    }

    let on_profile_update = {
        let applicant = applicant.clone();
        Callback::from(move |updated: Applicant| {
            applicant.set(Some(updated));
        })
    };

    let on_slot_booked = {
        let applicant = applicant.clone();
        Callback::from(move |slot: String| {
            if let Some(mut app) = (*applicant).clone() {
                app.interview_slot = Some(slot);
                applicant.set(Some(app));
            }
        })
    };

    html! {
        <>
        <div class="backdrop-blur-sm bg-gray-900/30 min-h-screen z-10 flex flex-col items-center p-4 lg:p-12 space-y-6">
            <Header auth={props.auth.clone()} />
            <div class="w-full max-w-4xl lg:max-w-6xl xl:max-w-7xl bg-white shadow sm:rounded-lg overflow-y-auto max-h-[80vh] lg:max-h-full px-6 py-5">
            {
                if let Some(err) = &*error {
                    html! { <div class="text-red-500 text-lg lg:text-xl">{ err }</div> }
                } else if let Some(app) = &*applicant {
                    let current_round = app.round.as_deref().unwrap_or("Applied");
                    
                    html! {
                        <>
                            <div class="mb-6 px-4 py-5 sm:px-6 border-b border-gray-200">
                                <h3 class="text-xl leading-6 font-medium text-gray-900">
                                    { "Current Round: " } { current_round }
                                </h3>
                            </div>
                            {
                                match current_round {
                                    "Applied" => {
                                        if app.mobile.is_some() && app.grad_year.is_some() {
                                            html! {
                                                <div class="p-6 text-center text-lg text-green-600 font-semibold">
                                                    { "Application Form Submitted successfully! Next rounds soon will be shown." }
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <ApplicantForm 
                                                    applicant={app.clone()} 
                                                    auth={props.auth.clone()} 
                                                    on_update={on_profile_update.clone()}
                                                />
                                            }
                                        }
                                    },
                                    "Case Study 1" => {
                                        if app.submission1_url.is_some() {
                                            html! {
                                                <div class="p-6 text-center text-lg text-green-600 font-semibold">
                                                    { "Case Study 1 submitted successfully! Next rounds soon will be shown." }
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <CaseStudySubmission 
                                                    auth={props.auth.clone()} 
                                                    skill={app.skill.clone()}
                                                    submission1_url={app.submission1_url.clone()}
                                                    submission2_url={app.submission2_url.clone()}
                                                />
                                            }
                                        }
                                    },
                                    "Case Study 2" => {
                                        if app.submission2_url.is_some() {
                                            html! {
                                                <div class="p-6 text-center text-lg text-green-600 font-semibold">
                                                    { "Case Study 2 submitted successfully! Next rounds soon will be shown." }
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <CaseStudySubmission 
                                                    auth={props.auth.clone()} 
                                                    skill={app.skill.clone()}
                                                    submission1_url={app.submission1_url.clone()}
                                                    submission2_url={app.submission2_url.clone()}
                                                />
                                            }
                                        }
                                    },
                                    "Interview" => {
                                        if app.interview_slot.is_some() {
                                            html! {
                                                <div class="p-6 text-center text-lg text-green-600 font-semibold">
                                                    { "Interview Slot booked successfully! We will contact you soon." }
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <InterviewSlotBooking 
                                                    applicant={app.clone()} 
                                                    auth={props.auth.clone()} 
                                                    on_slot_booked={on_slot_booked.clone()}
                                                />
                                            }
                                        }
                                    },
                                    _ => {
                                        html! {
                                            <div class="p-6 text-center text-lg text-gray-600 font-semibold">
                                                { "Your application has been processed. Please wait for further updates." }
                                            </div>
                                        }
                                    }
                                }
                            }
                        </>
                    }
                } else {
                    html! { <p class="text-gray-600 text-lg">{ "Loading rounds data..." }</p> }
                }
            }
            </div>
            <Footer/>
        </div>
        </>
    }
}
