use yew::prelude::*;
use crate::models::applicant::Applicant;
use crate::services::api;
use crate::auth::context::AuthContextHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub applicant: Applicant,
    pub auth: AuthContextHandle,
    pub on_update: Callback<Applicant>,
}

#[function_component]
pub fn ApplicantForm(props: &Props) -> Html {
    let applicant = use_state(|| props.applicant.clone());
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let is_loading = use_state(|| false);

    let grad_year = {
        let app = applicant.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let mut updated = (*app).clone();
            updated.grad_year = Some(input.value());
            app.set(updated);
        })
    };

    let mobile = {
        let app = applicant.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let mut updated = (*app).clone();
            updated.mobile = Some(input.value());
            app.set(updated);
        })
    };

    let gender = {
        let app = applicant.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let mut updated = (*app).clone();
            updated.gender = Some(input.value());
            app.set(updated);
        })
    };

    let faculty = {
        let app = applicant.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let mut updated = (*app).clone();
            updated.faculty = Some(input.value());
            app.set(updated);
        })
    };

    let department = {
        let app = applicant.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let mut updated = (*app).clone();
            updated.department = Some(input.value());
            app.set(updated);
        })
    };

    let skill = {
        let app = applicant.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let mut updated = (*app).clone();
            updated.skill = Some(input.value());
            app.set(updated);
        })
    };

    let event_participation = {
        let app = applicant.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let mut updated = (*app).clone();
            updated.event_participation = Some(input.checked());
            app.set(updated);
        })
    };

    let why_apply = {
        let app = applicant.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            let mut updated = (*app).clone();
            updated.why_apply = Some(input.value());
            app.set(updated);
        })
    };

    let event_experience = {
        let app = applicant.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            let mut updated = (*app).clone();
            updated.event_experience = Some(input.value());
            app.set(updated);
        })
    };

    let on_submit = {
        let applicant = applicant.clone();
        let error = error.clone();
        let success = success.clone();
        let is_loading = is_loading.clone();
        let token = props.auth.token();
        let on_update = props.on_update.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let applicant = (*applicant).clone();
            let error = error.clone();
            let success = success.clone();
            let is_loading = is_loading.clone();
            let token = token.clone();
            let on_update = on_update.clone();

            // Validation
            if let Some(mobile) = &applicant.mobile {
                if mobile.len() != 10 || !mobile.chars().all(|c| c.is_numeric()) {
                    error.set(Some("Mobile must be 10 digits".to_string()));
                    return;
                }
            }

            if let Some(why_apply) = &applicant.why_apply {
                let word_count = why_apply.split_whitespace().count();
                if word_count > 150 {
                    error.set(Some(format!(
                        "Why apply must be 150 words or less (current: {})",
                        word_count
                    )));
                    return;
                }
            }

            if let Some(event_experience) = &applicant.event_experience {
                let word_count = event_experience.split_whitespace().count();
                if word_count > 50 {
                    error.set(Some(format!(
                        "Event experience must be 50 words or less (current: {})",
                        word_count
                    )));
                    return;
                }
            }

            is_loading.set(true);
            error.set(None);
            success.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match api::update_applicant(&applicant, token).await {
                    Ok(updated) => {
                        success.set(Some("Profile updated successfully!".to_string()));
                        is_loading.set(false);
                        on_update.emit(updated);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        is_loading.set(false);
                    }
                }
            });
        })
    };

    let get_department_options = |faculty: &str| -> Vec<&'static str> {
        match faculty {
            "arts" => vec!["History", "Economics", "Political Science"],
            "science" => vec!["Physics", "Chemistry", "Biology"],
            "engineering" => vec!["CSE", "ECE", "Mechanical"],
            "ISLM" => vec!["International Relations", "Development Studies"],
            _ => vec![],
        }
    };

    let departments = applicant
        .faculty
        .as_deref()
        .map(get_department_options)
        .unwrap_or_default();

    html! {
        <div class="bg-white shadow sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
                <h3 class="text-lg font-medium leading-6 text-gray-900">
                    { "Application Details" }
                </h3>
                <p class="mt-1 text-sm text-gray-500">
                    { "Fill in your information to complete your application." }
                </p>
            </div>

            <form onsubmit={on_submit} class="space-y-6 px-4 py-5 sm:px-6">
                {
                    if let Some(err) = &*error {
                        html! {
                            <div class="rounded-md bg-red-50 p-4">
                                <div class="text-red-700">{ err }</div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if let Some(msg) = &*success {
                        html! {
                            <div class="rounded-md bg-green-50 p-4">
                                <div class="text-green-700">{ msg }</div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
                    <div>
                        <label for="grad_year" class="block text-sm font-medium text-gray-700">
                            { "Graduation Year" }
                        </label>
                        <select
                            id="grad_year"
                            required=true
                            value={applicant.grad_year.clone().unwrap_or_default()}
                            onchange={grad_year}
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        >
                            <option value="">{ "Select graduation year" }</option>
                            <option value="2028">{ "2028" }</option>
                            <option value="2029">{ "2029" }</option>
                        </select>
                    </div>

                    <div>
                        <label for="mobile" class="block text-sm font-medium text-gray-700">
                            { "Mobile (10-digit)" }
                        </label>
                        <input
                            id="mobile"
                            type="tel"
                            required=true
                            maxlength="10"
                            value={applicant.mobile.clone().unwrap_or_default()}
                            oninput={mobile}
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                            placeholder="1234567890"
                        />
                    </div>

                    <div>
                        <label for="gender" class="block text-sm font-medium text-gray-700">
                            { "Gender" }
                        </label>
                        <select
                            id="gender"
                            required=true
                            value={applicant.gender.clone().unwrap_or_default()}
                            onchange={gender}
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        >
                            <option value="">{ "Select gender" }</option>
                            <option value="Male">{ "Male" }</option>
                            <option value="Female">{ "Female" }</option>
                            <option value="Other">{ "Other" }</option>
                        </select>
                    </div>

                    <div>
                        <label for="faculty" class="block text-sm font-medium text-gray-700">
                            { "Faculty" }
                        </label>
                        <select
                            id="faculty"
                            required=true
                            value={applicant.faculty.clone().unwrap_or_default()}
                            onchange={faculty}
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        >
                            <option value="">{ "Select faculty" }</option>
                            <option value="arts">{ "Arts" }</option>
                            <option value="science">{ "Science" }</option>
                            <option value="engineering">{ "Engineering" }</option>
                            <option value="ISLM">{ "ISLM" }</option>
                        </select>
                    </div>

                    <div>
                        <label for="department" class="block text-sm font-medium text-gray-700">
                            { "Department" }
                        </label>
                        <select
                            id="department"
                            required=true
                            value={applicant.department.clone().unwrap_or_default()}
                            onchange={department}
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        >
                            <option value="">{ "Select department" }</option>
                            {
                                departments.iter().map(|dept| {
                                    html! {
                                        <option value={dept.to_string()}>{ dept }</option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                    </div>

                    <div>
                        <label for="skill" class="block text-sm font-medium text-gray-700">
                            { "Skill" }
                        </label>
                        <select
                            id="skill"
                            required=false
                            value={applicant.skill.clone().unwrap_or_default()}
                            onchange={skill}
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        >
                            <option value="">{ "Select skill" }</option>
                            <option value="design">{ "Design" }</option>
                            <option value="tech">{ "Tech" }</option>
                        </select>
                    </div>
                </div>

                <div>
                    <label class="flex items-center">
                        <input
                            type="checkbox"
                            checked={applicant.event_participation.unwrap_or(false)}
                            onchange={event_participation}
                            class="rounded border-gray-300 text-blue-600 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                        />
                        <span class="ml-2 text-sm text-gray-700">
                            { "Have you participated in any events?" }
                        </span>
                    </label>
                </div>

                <div>
                    <label for="why_apply" class="block text-sm font-medium text-gray-700">
                        { "Why do you want to apply? (max 150 words)" }
                    </label>
                    <textarea
                        id="why_apply"
                        required=true
                        maxlength="2000"
                        rows="4"
                        value={applicant.why_apply.clone().unwrap_or_default()}
                        oninput={why_apply}
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        placeholder="Tell us why you want to join E-Cell..."
                    />
                    <p class="mt-1 text-sm text-gray-500">
                        {
                            {
                            let count = applicant
                                .why_apply
                                .as_deref()
                                .unwrap_or("")
                                .split_whitespace()
                                .count();
                            format!("{} / 150 words", count)
                            }
                        }
                    </p>
                </div>

                <div>
                    <label for="event_experience" class="block text-sm font-medium text-gray-700">
                        { "Event Experience Details (max 50 words, if yes above)" }
                    </label>
                    <textarea
                        id="event_experience"
                        maxlength="500"
                        rows="3"
                        value={applicant.event_experience.clone().unwrap_or_default()}
                        oninput={event_experience}
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm border p-2"
                        placeholder="Tell us about your event experience..."
                    />
                    <p class="mt-1 text-sm text-gray-500">
                        {
                            {
                            let count = applicant
                                .event_experience
                                .as_deref()
                                .unwrap_or("")
                                .split_whitespace()
                                .count();
                            format!("{} / 50 words", count)
                            }
                        }
                    </p>
                </div>

                <div>
                    <button
                        type="submit"
                        disabled={*is_loading}
                        class="w-full inline-flex justify-center rounded-md border border-transparent bg-blue-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
                    >
                        {
                            if *is_loading {
                                html! { "Saving..." }
                            } else {
                                html! { "Save Profile" }
                            }
                        }
                    </button>
                </div>
            </form>
        </div>
    }
}
