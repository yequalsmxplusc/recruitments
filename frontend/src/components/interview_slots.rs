use yew::prelude::*;
use crate::models::applicant::{Applicant, InterviewSlot};
use crate::services::api;
use crate::auth::context::AuthContextHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub applicant: Applicant,
    pub auth: AuthContextHandle,
    pub on_slot_booked: Callback<String>,
}

#[function_component]
pub fn InterviewSlotBooking(props: &Props) -> Html {
    let slots = use_state(|| Vec::<InterviewSlot>::new());
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let is_loading = use_state(|| false);
    let token = props.auth.token();

    // Load available slots on mount
    {
        let slots = slots.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let token = token.clone();

        use_effect_with((), move |_| {
            is_loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match api::get_available_slots(token).await {
                    Ok(available_slots) => {
                        slots.set(available_slots);
                        is_loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        is_loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    let on_slot_select = {
        let applicant = props.applicant.clone();
        let error = error.clone();
        let success = success.clone();
        let is_loading = is_loading.clone();
        let token = props.auth.token();
        let on_slot_booked = props.on_slot_booked.clone();

        move |slot_datetime: String| {
            let mut updated_applicant = applicant.clone();
            updated_applicant.interview_slot = Some(slot_datetime.clone());

            let error = error.clone();
            let success = success.clone();
            let is_loading = is_loading.clone();
            let token = token.clone();
            let on_slot_booked = on_slot_booked.clone();

            is_loading.set(true);
            error.set(None);
            success.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match api::update_applicant(&updated_applicant, token).await {
                    Ok(_) => {
                        success.set(Some("Interview slot booked successfully!".to_string()));
                        is_loading.set(false);
                        on_slot_booked.emit(slot_datetime);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        is_loading.set(false);
                    }
                }
            });
        }
    };

    let format_datetime = |datetime_str: &str| -> String {
        // Parse ISO format and display in readable format
        if let Some(date_part) = datetime_str.split('T').next() {
            if let Some(time_part) = datetime_str.split('T').nth(1) {
                let time_only = time_part.split(':').take(2).collect::<Vec<_>>().join(":");
                return format!("{} at {}", date_part, time_only);
            }
        }
        datetime_str.to_string()
    };

    html! {
        <div class="glass-card rounded-2xl mt-6">
            <div class="px-4 py-5 sm:px-6 border-b">
                <h3 class="text-lg font-medium leading-6 theme-text-primary">
                    { "Interview Slot Booking" }
                </h3>
                <p class="mt-1 text-sm opacity-75">
                    { "Select an available time slot for your interview" }
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
                                <div class="text-green-600 dark:text-green-400">{ msg }</div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if let Some(selected_slot) = props.applicant.interview_slot.as_deref().filter(|s| !s.is_empty()) {
                        html! {
                            <div class="rounded-lg bg-blue-500/20 border border-blue-500 p-4 mb-6">
                                <div class="flex">
                                    <div class="flex-1">
                                        <h4 class="text-sm font-medium theme-text-primary">
                                            { "Current Slot" }
                                        </h4>
                                        <p class="mt-1 text-sm text-blue-600 dark:text-blue-400">
                                            { format_datetime(selected_slot) }
                                        </p>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if *is_loading {
                        html! {
                            <div class="flex items-center justify-center">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                                <span class="ml-3 theme-text-primary">{ "Loading slots..." }</span>
                            </div>
                        }
                    } else if slots.is_empty() {
                        html! {
                            <div class="text-center py-6">
                                <p class="theme-text-primary opacity-75">{ "No slots available at the moment. Please check back later." }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                                {
                                    slots.iter().map(|slot| {
                                        let slot_datetime = slot.date_time.clone();
                                        let slot_datetime_clone = slot_datetime.clone();
                                        let is_selected = props
                                            .applicant
                                            .interview_slot
                                            .as_deref()
                                            .map(|s| s == &slot_datetime)
                                            .unwrap_or(false);

                                        let on_click = {
                                            let on_slot_select = on_slot_select.clone();
                                            let slot_datetime = slot_datetime.clone();
                                            Callback::from(move |_| {
                                                on_slot_select(slot_datetime.clone());
                                            })
                                        };

                                        html! {
                                            <button
                                                onclick={on_click}
                                                disabled={*is_loading}
                                                class={classes!(
                                                    "p-4", "rounded-lg", "border-2", "text-left", "font-medium", "transition", "cursor-pointer",
                                                    if is_selected {
                                                        "border-green-500 bg-green-500/10 text-green-600 dark:text-green-400"
                                                    } else if slot.remaining.unwrap_or(0) > 0 {
                                                        "glass-card border-blue-400/50 hover:border-blue-500 hover:bg-blue-500/5"
                                                    } else {
                                                        "border-red-400/50 bg-red-500/10 text-red-600 dark:text-red-400 cursor-not-allowed opacity-75"
                                                    }
                                                )}
                                            >
                                                <div class="font-semibold theme-text-primary">
                                                    { format_datetime(&slot_datetime_clone) }
                                                </div>
                                                <div class="text-sm mt-1 opacity-75">
                                                    {
                                                        if is_selected {
                                                            html! { "✓ Selected" }
                                                        } else if slot.remaining.unwrap_or(0) > 0 {
                                                            html! { 
                                                                format!("{} slots available", slot.remaining.unwrap_or(0))
                                                            }
                                                        } else {
                                                            html! { "Slot Full" }
                                                        }
                                                    }
                                                </div>
                                            </button>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }
                }
            </div>
        </div>
    }
}
