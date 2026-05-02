use yew::prelude::*;
use crate::models::applicant::Applicant;
use crate::services::api;
use crate::auth::context::AuthContextHandle;
use crate::components::header::Header;
use crate::components::footer::Footer;
use yew_router::components::Link;
use crate::routers::Route;

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
        <>
         <div class="backdrop-blur-sm bg-gray-900/30 min-h-screen z-10 flex flex-col items-center p-4 lg:p-12 space-y-6">
         <Header auth={props.auth.clone()} />
          <div class="w-full max-w-4xl lg:max-w-6xl xl:max-w-7xl bg-white shadow sm:rounded-lg overflow-y-auto max-h-[80vh] lg:max-h-full px-6 py-5">   
        {
        if let Some(err) = &*error {
            html! { <div class="text-red-500 text-lg lg:text-xl">{ err }</div> }
        } else if let Some(applicant) = &*applicant {
            let status_class = match applicant.status.as_deref() {
                Some("Selected") => "text-green-600",
                Some("In Consideration") => "text-yellow-600",
                Some("No longer in Consideration") => "text-red-600",
                _ => "text-gray-600",
            };
            html! {
                <>
                    <div class="bg-white shadow sm:rounded-lg w-full">
                        <div class="px-4 py-5 sm:px-6">
                            <h3 class="text-lg sm:text-xl lg:text-2xl leading-6 font-medium text-gray-900">
                                { "E-Cell Recruitment 2026" }
                            </h3>
                            <p class="mt-1 text-sm sm:text-base text-gray-500">
                                { "Welcome! Complete your application to proceed." }
                            </p>
                        </div>
                        <div class="border-t border-gray-200 px-4 py-5 sm:px-6">
                            <dl class="grid grid-cols-1 gap-x-4 gap-y-8 sm:grid-cols-2">
                                <div class="sm:col-span-1">
                                    <dt class="text-sm sm:text-base font-medium text-gray-500">
                                        { "Full name" }
                                    </dt>
                                    <dd class="mt-1 text-sm sm:text-base text-gray-900">
                                        { &applicant.name }
                                    </dd>
                                </div>
                                <div class="sm:col-span-1">
                                    <dt class="text-sm sm:text-base font-medium text-gray-500">
                                        { "Email address" }
                                    </dt>
                                    <dd class="mt-1 text-sm sm:text-base text-gray-900">
                                        { &applicant.email }
                                    </dd>
                                </div>
                                {
                                    if let Some(status) = &applicant.status {
                                        html! {
                                            <>
                                                <div class="sm:col-span-1">
                                                    <dt class="text-base sm:text-lg font-semibold text-gray-500">
                                                        { "Application Status" }
                                                    </dt>
                                                    <dd class={classes!("mt-1", "text-lg", "sm:text-xl", "font-bold", status_class)}>
                                                        { status }
                                                    </dd>
                                                </div>
                                                {
                                                    if status == "Selected" {
                                                       html! {
                                                           <div class="sm:col-span-1">
                                                               <a href="https://chat.whatsapp.com/HLQP0xicrTXJYIOVJGJTMR" target="_blank" class="bg-green-500 text-white py-2 px-4 rounded-full shadow-lg hover:bg-green-600 inline-block">
                                                                   { "Join WhatsApp Group" }
                                                               </a>
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
                                <div class="sm:col-span-2 text-center mt-6">
                                    <Link<Route> to={Route::Rounds} classes="bg-blue-600 text-white py-3 px-6 rounded-lg shadow-lg hover:bg-blue-700 inline-block font-semibold">
                                        { "Go to My Rounds" }
                                    </Link<Route>>
                                </div>
                            </dl>
                        </div>
                    </div>
                </>
            }
        } else {
            html! { <p class="text-white text-lg">{ "Loading applicant data..." }</p> }    
        }}
        </div>
        <Footer/>
        </div>
        </>
    }
}