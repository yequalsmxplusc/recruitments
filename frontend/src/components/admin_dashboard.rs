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

fn get_stages(applicant: &Applicant) -> Vec<&'static str> {
    let is_skill = applicant
        .skill
        .as_deref()
        .map_or(false, |s| !s.is_empty());
    if is_skill {
        vec!["Round 1", "Case Study 1", "Case Study 2", "Interview"]
    } else {
        vec!["Round 1", "Case Study 1", "Interview"]
    }
}

fn advance_round(applicant: &Applicant) -> Option<String> {
    let stages = get_stages(applicant);
    let current = applicant.round.as_deref().unwrap_or("Round 1");
    let pos = stages.iter().position(|&s| s == current).unwrap_or(0);
    if pos + 1 < stages.len() {
        Some(stages[pos + 1].to_string())
    } else {
        None
    }
}

fn demote_round(applicant: &Applicant) -> Option<String> {
    let stages = get_stages(applicant);
    let current = applicant.round.as_deref().unwrap_or("Round 1");
    let pos = stages.iter().position(|&s| s == current).unwrap_or(0);
    if pos > 0 {
        Some(stages[pos - 1].to_string())
    } else {
        None
    }
}

#[function_component]
pub fn AdminDashboard(props: &Props) -> Html {
    let applicants = use_state(Vec::<Applicant>::new);
    let error = use_state(|| None::<String>);
    // Set of selected applicant IDs
    let selected: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let loading_ids: UseStateHandle<HashSet<String>> = use_state(HashSet::new);

    // Initial fetch
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

    // ── helpers ──────────────────────────────────────────────────────────────

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

    // ── select-all toggle ────────────────────────────────────────────────────
    let on_select_all = {
        let applicants = applicants.clone();
        let selected = selected.clone();
        Callback::from(move |_: Event| {
            let all_ids: HashSet<String> = (*applicants).iter().map(|a| a.id.clone()).collect();
            if (*selected).len() == all_ids.len() {
                selected.set(HashSet::new());
            } else {
                selected.set(all_ids);
            }
        })
    };

    // ── per-row checkbox toggle ──────────────────────────────────────────────
    let on_toggle_select = {
        let selected = selected.clone();
        Callback::from(move |id: String| {
            let mut s = (*selected).clone();
            if s.contains(&id) {
                s.remove(&id);
            } else {
                s.insert(id);
            }
            selected.set(s);
        })
    };

    // ── bulk advance ─────────────────────────────────────────────────────────
    let on_bulk_advance = {
        let applicants = applicants.clone();
        let selected = selected.clone();
        let patch_applicant = patch_applicant.clone();
        Callback::from(move |_: MouseEvent| {
            for applicant in (*applicants).iter() {
                if (*selected).contains(&applicant.id) {
                    if let Some(next) = advance_round(applicant) {
                        let mut updated = applicant.clone();
                        updated.round = Some(next);
                        patch_applicant.emit(updated);
                    }
                }
            }
        })
    };

    // ── bulk demote ──────────────────────────────────────────────────────────
    let on_bulk_demote = {
        let applicants = applicants.clone();
        let selected = selected.clone();
        let patch_applicant = patch_applicant.clone();
        Callback::from(move |_: MouseEvent| {
            for applicant in (*applicants).iter() {
                if (*selected).contains(&applicant.id) {
                    if let Some(prev) = demote_round(applicant) {
                        let mut updated = applicant.clone();
                        updated.round = Some(prev);
                        patch_applicant.emit(updated);
                    }
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
                        { "Round Management" }
                        <span class="ml-3 text-base font-normal opacity-60">
                            { format!("{} applicants", all_count) }
                        </span>
                    </h1>

                    // ── bulk action bar ────────────────────────────────────
                    if sel_count > 0 {
                        <div class="flex items-center gap-3 flex-wrap">
                            <span class="text-sm theme-text-primary opacity-75">
                                { format!("{} selected", sel_count) }
                            </span>
                            <button
                                onclick={on_bulk_advance.clone()}
                                class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 text-sm rounded-lg transition-colors duration-200 font-medium">
                                { "▶ Advance Selected" }
                            </button>
                            <button
                                onclick={on_bulk_demote.clone()}
                                class="bg-orange-500 hover:bg-orange-600 text-white px-4 py-2 text-sm rounded-lg transition-colors duration-200 font-medium">
                                { "◀ Demote Selected" }
                            </button>
                        </div>
                    }
                </div>

                if let Some(err) = &*error {
                    <div class="text-red-500 mb-4 p-4 rounded-lg border border-red-300">{ err }</div>
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
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Email" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Current Round" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Progress" }</th>
                                    <th class="py-3 px-4 text-left text-sm font-semibold theme-text-primary uppercase tracking-wide">{ "Actions" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for (*applicants).iter().map(|applicant| {
                                    let applicant = applicant.clone();
                                    let id = applicant.id.clone();
                                    let is_selected = (*selected).contains(&id);
                                    let is_loading = (*loading_ids).contains(&id);
                                    let stages = get_stages(&applicant);
                                    let current_round = applicant.round.as_deref().unwrap_or("Round 1");
                                    let current_pos = stages.iter().position(|&s| s == current_round).unwrap_or(0);
                                    let can_advance = current_pos + 1 < stages.len();
                                    let can_demote = current_pos > 0;

                                    let on_toggle = {
                                        let on_toggle_select = on_toggle_select.clone();
                                        let id = id.clone();
                                        Callback::from(move |_: Event| on_toggle_select.emit(id.clone()))
                                    };

                                    let on_advance = {
                                        let applicant = applicant.clone();
                                        let patch = patch_applicant.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            if let Some(next) = advance_round(&applicant) {
                                                let mut updated = applicant.clone();
                                                updated.round = Some(next);
                                                patch.emit(updated);
                                            }
                                        })
                                    };

                                    let on_demote = {
                                        let applicant = applicant.clone();
                                        let patch = patch_applicant.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            if let Some(prev) = demote_round(&applicant) {
                                                let mut updated = applicant.clone();
                                                updated.round = Some(prev);
                                                patch.emit(updated);
                                            }
                                        })
                                    };

                                    let row_bg = if is_selected {
                                        "bg-blue-500/10"
                                    } else {
                                        "hover:bg-black/5 dark:hover:bg-white/5"
                                    };

                                    html! {
                                        <tr key={id.clone()} class={classes!("border-b", "transition-colors", row_bg)}>
                                            <td class="py-3 px-4">
                                                <input
                                                    type="checkbox"
                                                    checked={is_selected}
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
                                            <td class="py-3 px-4 theme-text-primary text-sm opacity-75">{ &applicant.email }</td>
                                            <td class="py-3 px-4">
                                                <span class={classes!(
                                                    "text-sm", "font-semibold", "px-2.5", "py-1", "rounded-full",
                                                    if current_round == "Interview" { "bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-400" }
                                                    else if current_round.starts_with("Case") { "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/40 dark:text-yellow-400" }
                                                    else { "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300" }
                                                )}>
                                                    { current_round }
                                                </span>
                                            </td>
                                            <td class="py-3 px-4">
                                                <div class="flex items-center gap-1.5">
                                                    { for stages.iter().enumerate().map(|(i, &stage)| {
                                                        let color = if i < current_pos { "bg-green-500" }
                                                            else if i == current_pos { "bg-blue-500" }
                                                            else { "bg-gray-300 dark:bg-gray-600" };
                                                        html! {
                                                            <div
                                                                title={stage}
                                                                class={classes!("h-2", "w-8", "rounded-full", color, "transition-colors")}
                                                            />
                                                        }
                                                    })}
                                                    <span class="text-xs ml-1 opacity-50 theme-text-primary">
                                                        { format!("{}/{}", current_pos + 1, stages.len()) }
                                                    </span>
                                                </div>
                                            </td>
                                            <td class="py-3 px-4">
                                                <div class="flex items-center gap-2">
                                                    <button
                                                        onclick={on_demote}
                                                        disabled={!can_demote || is_loading}
                                                        class={classes!(
                                                            "px-3", "py-1.5", "text-sm", "rounded-lg",
                                                            "transition-colors", "duration-200", "font-medium",
                                                            if can_demote && !is_loading {
                                                                "bg-orange-100 hover:bg-orange-200 text-orange-700 dark:bg-orange-900/40 dark:hover:bg-orange-900/60 dark:text-orange-400"
                                                            } else {
                                                                "bg-gray-100 text-gray-400 cursor-not-allowed dark:bg-gray-800 dark:text-gray-600"
                                                            }
                                                        )}>
                                                        { "◀ Back" }
                                                    </button>
                                                    <button
                                                        onclick={on_advance}
                                                        disabled={!can_advance || is_loading}
                                                        class={classes!(
                                                            "px-3", "py-1.5", "text-sm", "rounded-lg",
                                                            "transition-colors", "duration-200", "font-medium",
                                                            if can_advance && !is_loading {
                                                                "bg-blue-600 hover:bg-blue-700 text-white"
                                                            } else {
                                                                "bg-gray-100 text-gray-400 cursor-not-allowed dark:bg-gray-800 dark:text-gray-600"
                                                            }
                                                        )}>
                                                        if is_loading {
                                                            { "..." }
                                                        } else {
                                                            { "Advance ▶" }
                                                        }
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