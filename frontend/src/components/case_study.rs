use crate::auth::context::AuthContextHandle;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
    pub skill: Option<String>,
    pub submission1_url: Option<String>,
    pub submission2_url: Option<String>,
}

#[function_component]
pub fn CaseStudySubmission(props: &Props) -> Html {
    let file_input = use_node_ref();
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let is_loading = use_state(|| false);
    let active_case_study = use_state(|| 1u32);

    let is_skill = props
        .skill
        .as_deref()
        .map_or(false, |s| {
            let s = s.to_lowercase();
            s == "tech" || s == "design"
        });
    let max_submissions = if is_skill { 2 } else { 1 };
    let countdown = use_state(|| None::<u32>);

    let on_file_select = {
        let file_input = file_input.clone();
        let error = error.clone();
        let success = success.clone();
        let is_loading = is_loading.clone();
        let token = props.auth.token();
        let active_case_study = active_case_study.clone();
        let countdown = countdown.clone();

        Callback::from(move |_: Event| {
            if *is_loading {
                return;
            }

            let file_input = file_input.clone();
            let error = error.clone();
            let success = success.clone();
            let is_loading = is_loading.clone();
            let token = token.clone();
            let countdown = countdown.clone();
            let case_study = *active_case_study;

            let Some(input) = file_input.cast::<web_sys::HtmlInputElement>() else { return; };
            let Some(files) = input.files() else { return; };
            let Some(file) = files.get(0) else { return; };

            let filename = file.name();

            // Validate file type (PDF only)
            if !filename.ends_with(".pdf") {
                error.set(Some("Only PDF files are allowed".to_string()));
                return;
            }

            is_loading.set(true);
            error.set(None);
            success.set(None);

            let file = file.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let gloo_file: gloo_file::File = file.into();
                let bytes = match gloo_file::futures::read_as_bytes(&gloo_file).await {
                    Ok(b) => b,
                    Err(e) => {
                        error.set(Some(format!("Failed to read file: {:?}", e)));
                        is_loading.set(false);
                        return;
                    }
                };

                match crate::services::api::submit_case_study(
                    bytes.to_vec(),
                    filename,
                    case_study,
                    token,
                )
                .await
                {
                    Ok(_) => {
                        success.set(Some(format!(
                            "Case study {} uploaded successfully!",
                            case_study
                        )));
                        is_loading.set(false);
                        countdown.set(Some(5));
                    }
                    Err(e) => {
                        error.set(Some(e));
                        is_loading.set(false);
                    }
                }
            });
        })
    };

    let switch_case_study = |case: u32| {
        let active = active_case_study.clone();
        Callback::from(move |_| {
            active.set(case);
        })
    };

    {
        let countdown = countdown.clone();
        use_effect_with(countdown.clone(), move |count| {
            let mut timeout = None;
            if let Some(c) = **count {
                if c > 0 {
                    let countdown = countdown.clone();
                    timeout = Some(gloo_timers::callback::Timeout::new(1000, move || {
                        countdown.set(Some(c - 1));
                    }));
                } else {
                    web_sys::window().unwrap().location().reload().unwrap();
                }
            }
            move || { drop(timeout); }
        });
    }

    html! {
        <div class="bg-white shadow sm:rounded-lg mt-6">
            <div class="px-4 py-5 sm:px-6">
                <h3 class="text-lg font-medium leading-6 text-gray-900">
                    { "Case Study Submissions" }
                </h3>
                <p class="mt-1 text-sm text-gray-500">
                    {
                        if is_skill {
                            "Submit your solutions for both case studies"
                        } else {
                            "Submit your solution for the case study"
                        }
                    }
                </p>
            </div>

            <div class="border-t border-gray-200 px-4 py-5 sm:px-6">
                {
                    if let Some(err) = &*error {
                        html! {
                            <div class="rounded-md bg-red-50 p-4 mb-4">
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
                            <div class="rounded-md bg-green-50 p-4 mb-4">
                                <div class="text-green-700">
                                    { msg }
                                    {
                                        if let Some(c) = *countdown {
                                            format!(" Refreshing in {} seconds...", c)
                                        } else {
                                            "".to_string()
                                        }
                                    }
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if is_skill {
                        html! {
                            <div class="flex space-x-2 mb-6">
                                <button
                                    onclick={switch_case_study(1)}
                                    class={classes!(
                                        "px-4", "py-2", "rounded-md", "font-medium",
                                        if *active_case_study == 1 {
                                            "bg-blue-600 text-white"
                                        } else {
                                            "bg-gray-100 text-gray-700 hover:bg-gray-200"
                                        }
                                    )}
                                >
                                    { "Case Study 1" }
                                </button>
                                <button
                                    onclick={switch_case_study(2)}
                                    class={classes!(
                                        "px-4", "py-2", "rounded-md", "font-medium",
                                        if *active_case_study == 2 {
                                            "bg-blue-600 text-white"
                                        } else {
                                            "bg-gray-100 text-gray-700 hover:bg-gray-200"
                                        }
                                    )}
                                >
                                    { "Case Study 2" }
                                </button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="space-y-4">
                    {
                        (1..=max_submissions).map(|i| {
                            let is_visible = if is_skill {
                                *active_case_study == i
                            } else {
                                true
                            };

                            if !is_visible {
                                return html! {};
                            }

                            let submission_url = if i == 1 {
                                props.submission1_url.clone()
                            } else {
                                props.submission2_url.clone()
                            };

                            html! {
                                <div class="border-l-4 border-blue-500 bg-blue-50 p-4">
                                    <div class="flex">
                                        <div class="flex-1">
                                            <h4 class="text-sm font-medium text-gray-900">
                                                { format!("Case Study {}", i) }
                                            </h4>
                                            {
                                                if let Some(url) = submission_url.filter(|s| !s.is_empty()) {
                                                    html! {
                                                        <div class="mt-2">
                                                            <p class="text-sm text-gray-600">
                                                                { "✓ Uploaded successfully" }
                                                            </p>
                                                            <a href={url} target="_blank" class="text-sm text-blue-600 hover:text-blue-800">
                                                                { "View submission" }
                                                            </a>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {
                                                        <div class="mt-4">
                                                            <div class="flex items-center justify-center w-full">
                                                                <label class="flex flex-col items-center justify-center w-full h-32 border-2 border-blue-300 border-dashed rounded-lg cursor-pointer bg-blue-50 hover:bg-blue-100">
                                                                    <div class="flex flex-col items-center justify-center pt-5 pb-6">
                                                                        <svg class="w-8 h-8 text-gray-500 mb-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                                                                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 5.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 19V6m0 0L8 8m2-2l2 2"/>
                                                                        </svg>
                                                                        <p class="text-sm text-gray-500">
                                                                            { "Click to upload PDF" }
                                                                        </p>
                                                                    </div>
                                                                    <input
                                                                        ref={file_input.clone()}
                                                                        type="file"
                                                                        accept=".pdf"
                                                                        onchange={on_file_select.clone()}
                                                                        class="hidden"
                                                                    />
                                                                </label>
                                                            </div>
                                                            <p class="text-xs text-gray-500 mt-2">
                                                                { "PDF files only, max 10MB" }
                                                            </p>
                                                        </div>
                                                    }
                                                }
                                            }
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                {
                    if *is_loading {
                        html! {
                            <div class="mt-4 flex items-center justify-center">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                                <span class="ml-3 text-gray-600">{ "Uploading..." }</span>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
