use crate::auth::context::AuthContextHandle;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::models::applicant::Applicant;
use crate::services::api;
use std::collections::HashSet;
use yew::functional::use_effect_with;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: AuthContextHandle,
}

const STATUSES: &[&str] = &["In Consideration", "Selected", "Rejected"];

fn next_status(current: &str) -> &'static str {
    match current {
        "In Consideration" => "Selected",
        "Selected" => "Rejected",
        "Rejected" => "In Consideration",
        _ => "In Consideration",
    }
}

fn status_badge_class(status: &str) -> &'static str {
    match status {
        "Selected" => "bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-400",
        "Rejected" => "bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-400",
        _ => "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/40 dark:text-yellow-400",
    }
}

#[function_component]
pub fn ApplicantList(props: &Props) -> Html {
    let applicants = use_state(Vec::<Applicant>::new);
    let error = use_state(|| None::<String>);
    let selected: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let loading_ids: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let bulk_status = use_state(|| "In Consideration".to_string());

    {
        let applicants = applicants.clone();
        let error = error.clone();
        let token = props.auth.token();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_all_applicants(token).await {
                    Ok(data) => applicants.set(data),
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

    let patch_applicant = {
        let applicants = applicants.clone();
        let error = error.clone();
        let loading_ids = loading_ids.clone();
        let token = props.auth.token();

        Callback::from(move |updated: Applicant| {
            let applicants = applicants.clone();
            let error = error.clone();
            let loading_ids = loading_ids.clone();
            let token = token.clone();
            let id = updated.id.clone();

            let mut lids = (*loading_ids).clone();
            lids.insert(id.clone());
            loading_ids.set(lids);

            wasm_bindgen_futures::spawn_local(async move {
                match api::update_applicant(&updated, token).await {
                    Ok(fresh) => {
                        let mut list = (*applicants).clone();
                        if let Some(idx) = list.iter().position(|a| a.id == fresh.id) {
                            list[idx] = fresh;
                        }
                        applicants.set(list);
                    }
                    Err(e) => error.set(Some(e)),
                }
                let mut lids = (*loading_ids).clone();
                lids.remove(&id);
                loading_ids.set(lids);
            });
        })
    };

    let on_select_all = {
        let applicants = applicants.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            let all: HashSet<String> = (*applicants).iter().map(|a| a.id.clone()).collect();
            if (*selected).len() == all.len() {
                selected.set(HashSet::new());
            } else {
                selected.set(all);
            }
        })
    };

    let on_toggle_select = {
        let selected = selected.clone();
        Callback::from(move |id: String| {
            let mut s = (*selected).clone();
            if s.contains(&id) { s.remove(&id); } else { s.insert(id); }
            selected.set(s);
        })
    };

    let on_bulk_status = {
        let applicants = applicants.clone();
        let selected = selected.clone();
        let bulk_status = bulk_status.clone();
        let patch = patch_applicant.clone();
        Callback::from(move |_: MouseEvent| {
            let target_status = (*bulk_status).clone();
            for applicant in (*applicants).iter() {
                if (*selected).contains(&applicant.id) {
                    let mut updated = applicant.clone();
                    updated.status = Some(target_status.clone());
                    patch.emit(updated);
                }
            }
        })
    };

    let on_reset_all = {
        let applicants = applicants.clone();
        let selected = selected.clone();
        let patch = patch_applicant.clone();
        Callback::from(move |_: MouseEvent| {
            for applicant in (*applicants).iter() {
                if (*selected).contains(&applicant.id) {
                    let mut updated = applicant.clone();
                    updated.status = Some("In Consideration".to_string());
                    updated.round = Some("Round 1".to_string());
                    patch.emit(updated);
                }
            }
        })
    };

    let all_count = applicants.len();
    let sel_count = selected.len();
    let all_selected = all_count > 0 && sel_count == all_count;

    html! {
        <div class="theme-wrapper min-h-screen flex flex-col">
            <Header auth={props.auth.clone()} />
            <div class="container mx-auto px-4 py-8 flex-grow">
                <div class="flex items-center justify-between mb-6 flex-wrap gap-3">
                    <h1 class="text-3xl font-bold theme-text-primary">
                        { "Applicants" }
                        <span class="ml-3 text-base font-normal opacity-60">
                            { format!("{} total", all_count) }
                        </span>
                    </h1>
                    if sel_count > 0 {
                        <div class="flex items-center gap-3 flex-wrap">
                            <span class="text-sm theme-text-primary opacity-75">
                                { format!("{} selected", sel_count) }
                            </span>
                            <div class="flex items-center gap-2">
                                <select
                                    class="theme-input border rounded-lg px-3 py-2 text-sm"
                                    onchange={{
                                        let bulk_status = bulk_status.clone();
                                        Callback::from(move |e: Event| {
                                            use web_sys::HtmlSelectElement;
                                            if let Some(el) = e.target_dyn_into::<HtmlSelectElement>() {
                                                bulk_status.set(el.value());
                                            }
                                        })
                                    }}>
                                    { for STATUSES.iter().map(|&s| html! {
                                        <option value={s} selected={*bulk_status == s}>{ s }</option>
                                    })}
                                </select>
                                <button
                                    onclick={on_bulk_status}
                                    class="bg-amber-500 hover:bg-amber-600 text-white px-4 py-2 text-sm rounded-lg transition-colors font-medium">
                                    { "Apply Status" }
                                </button>
                            </div>

                            <button
                                onclick={on_reset_all}
                                class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 text-sm rounded-lg transition-colors font-medium">
                                { "Reset Selected" }
                            </button>
                        </div>
                    }
                </div>

                if let Some(err) = &*error {
                    if err == "not_admin" {
                        <div class="text-center text-2xl text-red-600 font-semibold">{ "404: Page Not Found" }</div>
                    } else {
                        <div class="text-red-500 mb-4 p-4 rounded-lg border border-red-300">{ err }</div>
                    }
                } else {
                    <div class="overflow-x-auto rounded-xl border theme-card shadow-sm">
                        <table class="min-w-full">
                            <thead>
                                <tr class="border-b theme-card">
                                    <th class="py-3 px-4 text-left">
                                        <input
                                            type="checkbox"
                                            checked={all_selected}
                                            onchange={on_select_all}
                                            class="w-4 h-4 rounded accent-blue-600 cursor-pointer"
                                        />
                                    </th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Name" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Department" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Grad Year" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Gender" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Status" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Actions" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for (*applicants).iter().map(|applicant| {
                                    let applicant = applicant.clone();
                                    let id = applicant.id.clone();
                                    let is_selected_row = (*selected).contains(&id);
                                    let is_loading = (*loading_ids).contains(&id);
                                    let current_status = applicant.status.as_deref().unwrap_or("In Consideration");

                                    let on_toggle = {
                                        let on_toggle_select = on_toggle_select.clone();
                                        let id = id.clone();
                                        Callback::from(move |_: Event| on_toggle_select.emit(id.clone()))
                                    };

                                    // single-row status cycle
                                    let on_status_toggle = {
                                        let applicant = applicant.clone();
                                        let patch = patch_applicant.clone();
                                        let next = next_status(current_status).to_string();
                                        Callback::from(move |_: MouseEvent| {
                                            let mut updated = applicant.clone();
                                            updated.status = Some(next.clone());
                                            patch.emit(updated);
                                        })
                                    };
                                    let on_reset = {
                                        let applicant = applicant.clone();
                                        let patch = patch_applicant.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            let mut updated = applicant.clone();
                                            updated.status = Some("In Consideration".to_string());
                                            updated.round = Some("Round 1".to_string());
                                            patch.emit(updated);
                                        })
                                    };
                                    let row_bg = if is_selected_row {
                                        "bg-blue-500/10"
                                    } else {
                                        "hover:bg-black/5"
                                    };
                                    html! {
                                        <tr key={id.clone()} class={classes!("border-b", "transition-colors", row_bg)}>
                                            <td class="py-3 px-4">
                                                <input
                                                    type="checkbox"
                                                    checked={is_selected_row}
                                                    onchange={on_toggle}
                                                    class="w-4 h-4 rounded accent-blue-600 cursor-pointer"
                                                />
                                            </td>
                                            <td class="py-3 px-4 theme-text-primary font-medium">
                                                { &applicant.name }
                                                if applicant.is_admin {
                                                    <span class="ml-2 text-xs bg-purple-500 text-white px-1.5 py-0.5 rounded">{ "admin" }</span>
                                                }
                                            </td>
                                            <td class="py-3 px-4 theme-text-primary text-sm">
                                                { applicant.department.as_deref().unwrap_or("—") }
                                            </td>
                                            <td class="py-3 px-4 theme-text-primary text-sm">
                                                { applicant.grad_year.as_deref().unwrap_or("—") }
                                            </td>
                                            <td class="py-3 px-4 theme-text-primary text-sm">
                                                { applicant.gender.as_deref().unwrap_or("—") }
                                            </td>
                                            <td class="py-3 px-4">
                                                <span class={classes!(
                                                    "text-xs", "font-semibold", "px-2.5", "py-1", "rounded-full",
                                                    status_badge_class(current_status)
                                                )}>
                                                    { current_status }
                                                </span>
                                            </td>
                                            <td class="py-3 px-4">
                                                <div class="flex items-center gap-2">
                                                    <button
                                                        onclick={on_status_toggle}
                                                        disabled={is_loading}
                                                        class="bg-amber-500 hover:bg-amber-600 text-white px-3 py-1.5 text-xs rounded-lg transition-colors font-medium disabled:opacity-50 disabled:cursor-not-allowed">
                                                        if is_loading { { "..." } } else { { "Toggle" } }
                                                    </button>
                                                    <button
                                                        onclick={on_reset}
                                                        disabled={is_loading}
                                                        class="bg-gray-500 hover:bg-gray-600 text-white px-3 py-1.5 text-xs rounded-lg transition-colors font-medium disabled:opacity-50 disabled:cursor-not-allowed">
                                                        { "Reset" }
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                }
            </div>
            <Footer />
        </div>
    }
}