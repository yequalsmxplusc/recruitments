use crate::auth::context::AuthContextHandle;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::models::applicant::Applicant;
use crate::routers::Route;
use crate::services::api;
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &Props) -> Html {
    // Use Option<Applicant> because initially no data loaded
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

    html! {
        <div class="min-h-screen flex flex-col theme-wrapper overflow-x-hidden">
            <Header auth={props.auth.clone()} />
            <main class="flex-grow flex flex-col items-center p-4 lg:p-8 space-y-6">
                <div class="w-full max-w-4xl lg:max-w-6xl xl:max-w-7xl glass-card rounded-3xl theme-card border px-6 py-8 shadow-xl">
        {
        if let Some(err) = &*error {
            html! { <div class="text-red-500 text-lg lg:text-xl">{ err }</div> }
        } else if let Some(applicant) = &*applicant {
             let status_text = match applicant.status.as_deref() {
                Some("Rejected") => "No longer in Consideration",
                Some(s) => s,
                None => "Pending",
                };
            let status_class = match status_text {
                "Selected" => "text-green-600",
                "In Consideration" => "text-yellow-600",
                "No longer in Consideration" => "text-red-600",
                _ => "theme-text-primary",
            };
            let has_booked = applicant.interview_slot.as_deref().map_or(false, |s| !s.is_empty());

            let format_datetime = |datetime_str: &str| -> String {
                if let Some(date_part) = datetime_str.split('T').next() {
                    if let Some(time_part) = datetime_str.split('T').nth(1) {
                        let time_only = time_part.split(':').take(2).collect::<Vec<_>>().join(":");
                        return format!("{} at {}", date_part, time_only);
                    }
                }
                datetime_str.to_string()
            };

            html! {
                <>
                    <div class="glass-card rounded-2xl w-full">
                        <div class="px-4 py-5 sm:px-6 border-b">
                            <h3 class="text-lg sm:text-xl lg:text-2xl leading-6 font-medium theme-text-primary">
                                { "E-Cell Recruitment 2026" }
                            </h3>
                            <p class="mt-1 text-sm sm:text-base opacity-75">
                                {
                                    if has_booked {
                                        "Application Complete! You have successfully booked your interview. We will get back to you soon."
                                    } else if applicant.round.as_deref() == Some("Interview") {
                                        "Congratulations! You have reached the Interview round. Please select your slot in the Rounds section."
                                    } else if applicant.round.as_deref() == Some("Case Study 1") || applicant.round.as_deref() == Some("Case Study 2") {
                                        "Present your submissions for the Case Study in the Rounds section."
                                    } else {
                                        "Welcome! Complete your application to proceed."
                                    }
                                }
                            </p>
                        </div>
                        <div class="px-4 py-5 sm:px-6">
                            <dl class="grid grid-cols-1 gap-x-4 gap-y-8 sm:grid-cols-2">
                                <div class="sm:col-span-1">
                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                        { "Full name" }
                                    </dt>
                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary font-semibold">
                                        { &applicant.name }
                                    </dd>
                                </div>
                                <div class="sm:col-span-1">
                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                        { "Email address" }
                                    </dt>
                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary font-semibold">
                                        { &applicant.email }
                                    </dd>
                                </div>

                                {
                                    if applicant.round.as_deref() == Some("Interview") {
                                        html! {
                                            <>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                                        { "Mobile Number" }
                                                    </dt>
                                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary">
                                                        { applicant.mobile.as_deref().unwrap_or("N/A") }
                                                    </dd>
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                                        { "Graduation Year" }
                                                    </dt>
                                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary">
                                                        { applicant.grad_year.as_deref().unwrap_or("N/A") }
                                                    </dd>
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                                        { "Gender" }
                                                    </dt>
                                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary">
                                                        { applicant.gender.as_deref().unwrap_or("N/A") }
                                                    </dd>
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                                        { "Faculty & Department" }
                                                    </dt>
                                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary">
                                                        { format!("{} - {}", applicant.faculty.as_deref().unwrap_or("N/A"), applicant.department.as_deref().unwrap_or("N/A")) }
                                                    </dd>
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                                        { "Skill" }
                                                    </dt>
                                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary">
                                                        { applicant.skill.as_deref().unwrap_or("N/A") }
                                                    </dd>
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                                        { "Consideration Round" }
                                                    </dt>
                                                    <dd class="mt-1 text-sm sm:text-base theme-text-primary">
                                                        { applicant.round.as_deref().unwrap_or("N/A") }
                                                    </dd>
                                                </div>
                                                {
                                                    if let Some(slot) = applicant.interview_slot.as_deref().filter(|s| !s.is_empty()) {
                                                        html! {
                                                            <div class="sm:col-span-2 bg-blue-500/20 border border-blue-500 p-4 rounded-lg">
                                                                <dt class="text-sm sm:text-base font-semibold text-blue-600 dark:text-blue-400">
                                                                    { "Selected Interview Slot" }
                                                                </dt>
                                                                <dd class="mt-1 text-lg font-bold text-blue-700 dark:text-blue-300">
                                                                    { format_datetime(slot) }
                                                                </dd>
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }

                                <div class="sm:col-span-1">
                                    <dt class="text-sm sm:text-base font-medium opacity-75">
                                        { "Application Status" }
                                    </dt>
                                    <dd class={classes!("mt-1", "text-sm", "sm:text-base", "font-bold", status_class)}>
                                        { status_text }
                                        // { applicant.status.as_deref().unwrap_or("Pending") }
                                    </dd>
                                </div>
                                {
                                    if applicant.status.as_deref() == Some("Selected") {
                                       html! {
                                           <div class="sm:col-span-1">
                                               <a href="https://chat.whatsapp.com/groupchat" target="_blank" class="bg-green-500 text-white py-2 px-4 rounded-lg shadow-lg hover:bg-green-600 inline-block transition-colors duration-200">
                                                   { "Join WhatsApp Group" }
                                               </a>
                                           </div>
                                       }
                                    } else {
                                        html! {}
                                    }
                                }

                                <div class="sm:col-span-2 text-center mt-6">
                                    {
                                        if has_booked {
                                            html! {
                                                <button disabled=true class="bg-gray-400 text-white py-3 px-6 rounded-lg cursor-not-allowed inline-block font-semibold opacity-60">
                                                    { "All Rounds Completed" }
                                                </button>
                                            }
                                        } else {
                                            html! {
                                                <Link<Route> to={Route::Rounds} classes="bg-blue-600 text-white py-3 px-6 rounded-lg shadow-lg hover:bg-blue-700 inline-block font-semibold transition-colors duration-200">
                                                    { "Go to My Rounds" }
                                                </Link<Route>>
                                            }
                                        }
                                    }
                                </div>
                            </dl>
                        </div>
                    </div>
                </>
            }
        } else {
            html! { <p class="theme-text-primary text-lg">{ "Loading applicant data..." }</p> }
        }}
                </div>
            </main>
            <Footer/>
        </div>
    }
}
