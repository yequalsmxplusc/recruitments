use yew::prelude::*;

#[function_component(ProcessOverview)]
pub fn process_overview() -> Html {
    html! {
        <div class="w-full max-w-4xl lg:max-w-6xl xl:max-w-7xl glass-card rounded-2xl px-6 py-8">
            <h3 class="text-2xl font-semibold mb-4 theme-text-primary">
                { "Recruitments Rounds" }
            </h3>
            <div class="space-y-3 text-base lg:text-lg theme-text-primary opacity-90">
                <p>
                    { "• Round 1: Fill out the application form with your basic details." }
                </p>
                <p>
                    { "• Case Study 1: Case study / scenario based cases." }
                </p>
                <p>
                    { "• Case Study 2: Specialised Challenges for candidates interested in design or tech" }
                </p>
                <p>
                    { "• Interview: Selected candidates confirm an interview slot." }
                </p>
                <p>
                    { "• Final Selection: Results will be communicated after the interview process." }
                </p>
            </div>
        </div>
    }
}