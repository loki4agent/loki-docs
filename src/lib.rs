pub mod components;
pub mod types;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::params::Params;
use leptos_router::path;

use crate::components::doc_engine::DocEngine;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct DocEngineParams {
    pub lang: String,
    pub doc_path: Option<String>,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! {
                <div class="flex flex-col items-center justify-center min-h-screen p-8 text-center bg-slate-50 dark:bg-slate-950">
                    <h1 class="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-brand-primary to-brand-secondary mb-4">"404 - Not Found"</h1>
                    <p class="text-slate-600 dark:text-slate-400 mb-6">"The requested document or page does not exist."</p>
                    <a href="/en/index" class="px-5 py-2.5 rounded-lg bg-indigo-600 text-white font-medium shadow-md hover:bg-indigo-700 transition duration-200">"Go to English Docs"</a>
                </div>
            }>
                <Route path=path!("/:lang/*doc_path") view=DocEngine/>
                <Route path=path!("") view=|| view! { <Redirect path="/en/index"/> }/>
            </Routes>
        </Router>
    }
}
