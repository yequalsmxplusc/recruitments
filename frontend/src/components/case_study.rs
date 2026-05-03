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

    let is_skill = props.skill.as_deref().map_or(false, |s| {
        let s = s.to_lowercase();
        s == "tech" || s == "design"
    });
    let max_submissions = if is_skill { 2 } else { 1 };
    let countdown = use_state(|| None::<u32>);

    // Assignment Resolver Logic
    let get_case_study_info = |round: u32, skill: Option<&str>| {
        if round == 1 {
            match skill {
                Some("tech") => (
                    "Case Study 1 (Tech)",
                    "https://drive.google.com/uc?export=download&id=ROUND1_TECH_ID",
                    "case_study_1.md",
                ),
                Some("design") => (
                    "Case Study 1 (Design)",
                    "https://drive.google.com/uc?export=download&id=ROUND1_DESIGN_ID",
                    "case_study_1.pdf",
                ),
                _ => (
                    "Case Study 1",
                    "https://drive.google.com/uc?export=download&id=ROUND1_GENERIC_ID",
                    "case_study_1.pdf",
                ),
            }
        } else {
            match skill {
                Some("tech") => (
                    "Case Study 2 (Tech)",
                    "https://drive.google.com/uc?export=download&id=ROUND2_TECH_ID",
                    "case_study_2.md",
                ),
                Some("design") => (
                    "Case Study 2 (Design)",
                    "https://drive.google.com/uc?export=download&id=ROUND2_DESIGN_ID",
                    "case_study_2.pdf",
                ),
                _ => ("Case Study 2", "", ""),
            }
        }
    };

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

            let Some(input) = file_input.cast::<web_sys::HtmlInputElement>() else {
                return;
            };
            let Some(files) = input.files() else {
                return;
            };
            let Some(file) = files.get(0) else {
                return;
            };

            let filename = file.name();

            // Validate file type (PDF or MD for tech)
            let is_valid = if filename.ends_with(".pdf") {
                true
            } else if filename.ends_with(".md") {
                // MD is allowed for tech track
                true
            } else {
                false
            };

            if !is_valid {
                error.set(Some(
                    "Only PDF files (and .md for Tech) are allowed".to_string(),
                ));
                return;
            }

            is_loading.set(true);
            error.set(None);
            success.set(None);

            // Clone handles for the async block to ensure compilation
            let error = error.clone();
            let success = success.clone();
            let is_loading = is_loading.clone();
            let countdown = countdown.clone();
            let token = token.clone();
            let file = file.clone();
            let case_study = *active_case_study;

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
            move || {
                drop(timeout);
            }
        });
    }

    html! {
        <div class="glass-card rounded-2xl mt-6">
            <div class="px-4 py-5 sm:px-6 border-b">
                <h3 class="text-lg font-medium leading-6 theme-text-primary">
                    { "Case Study Submissions" }
                </h3>
                <p class="mt-1 text-sm opacity-75">
                    {
                        if is_skill {
                            "Submit your solutions for both case studies"
                        } else {
                            "Submit your solution for the case study"
                        }
                    }
                </p>
            </div>

            <div class="px-4 py-5 sm:px-6">
                {
                    if let Some(err) = &*error {
                        html! {
                            <div class="rounded-lg bg-red-500/20 border border-red-500 p-4 mb-4">
                                <div class="text-red-600 dark:text-red-400">{ err }</div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if let Some(msg) = &*success {
                        html! {
                            <div class="rounded-lg bg-green-500/20 border border-green-500 p-4 mb-4">
                                <div class="text-green-600 dark:text-green-400">
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
                                        "px-4", "py-2", "rounded-lg", "font-medium", "transition-colors",
                                        if *active_case_study == 1 {
                                            "bg-blue-600 text-white"
                                        } else {
                                            "bg-gray-300 dark:bg-gray-700 theme-text-primary hover:opacity-80"
                                        }
                                    )}
                                >
                                    { "Case Study 1" }
                                </button>
                                <button
                                    onclick={switch_case_study(2)}
                                    class={classes!(
                                        "px-4", "py-2", "rounded-lg", "font-medium", "transition-colors",
                                        if *active_case_study == 2 {
                                            "bg-blue-600 text-white"
                                        } else {
                                            "bg-gray-300 dark:bg-gray-700 theme-text-primary hover:opacity-80"
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

                            let (title, download_url, download_filename) = get_case_study_info(i, props.skill.as_deref());

                            html! {
                                <div class="border-l-4 border-blue-500 bg-blue-500/10 p-4 rounded-r-lg">
                                    <div class="flex">
                                        <div class="flex-1">
                                            <h4 class="text-sm font-medium theme-text-primary">
                                                { title }
                                            </h4>

                                            {
                                                if !download_url.is_empty() {
                                                    html! {
                                                        <div class="mt-3 mb-4">
                                                            <a
                                                                href={download_url.to_string()}
                                                                download={download_filename.to_string()}
                                                                class="inline-flex items-center px-4 py-2 bg-blue-600 text-white text-xs font-medium rounded-lg hover:bg-blue-700 transition-colors"
                                                            >
                                                                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a2 2 0 002 2h12a2 2 0 002-2v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>
                                                                </svg>
                                                                { "Download Case Study" }
                                                            </a>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }

                                            {
                                                if let Some(url) = submission_url.filter(|s| !s.is_empty()) {
                                                    html! {
                                                        <div class="mt-2 pt-2 border-t border-blue-500/20">
                                                            <p class="text-sm text-green-600 dark:text-green-400 font-medium">
                                                                { "✓ Uploaded successfully" }
                                                            </p>
                                                            <a href={url} target="_blank" class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 underline">
                                                                { "View your submission" }
                                                            </a>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {
                                                        <div class="mt-4">
                                                            <div class="flex items-center justify-center w-full">
                                                                <label class="flex flex-col items-center justify-center w-full h-32 border-2 border-blue-400 border-dashed rounded-lg cursor-pointer bg-blue-500/10 hover:bg-blue-500/20 transition-colors">
                                                                    <div class="flex flex-col items-center justify-center pt-5 pb-6">
                                                                        <svg class="w-8 h-8 text-blue-600 dark:text-blue-400 mb-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                                                                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 5.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 19V6m0 0L8 8m2-2l2 2"/>
                                                                        </svg>
                                                                        <p class="text-sm theme-text-primary opacity-75">
                                                                            { if props.skill.as_deref() == Some("tech") { "Click to upload PDF or MD" } else { "Click to upload PDF" } }
                                                                        </p>
                                                                    </div>
                                                                    <input
                                                                        ref={file_input.clone()}
                                                                        type="file"
                                                                        accept={if props.skill.as_deref() == Some("tech") { ".pdf,.md" } else { ".pdf" }}
                                                                        onchange={on_file_select.clone()}
                                                                        class="hidden"
                                                                    />
                                                                </label>
                                                            </div>
                                                            <p class="text-xs opacity-60 mt-2">
                                                                { if props.skill.as_deref() == Some("tech") { "PDF or MD files, max 10MB" } else { "PDF files only, max 10MB" } }
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
                                <span class="ml-3 theme-text-primary">{ "Uploading..." }</span>
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
