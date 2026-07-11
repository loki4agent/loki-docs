use crate::types::DocMeta;
use crate::DocEngineParams;
use leptos::prelude::*;
use leptos_router::hooks::use_params;

#[component]
pub fn Header() -> impl IntoView {
    let params = use_params::<DocEngineParams>();
    let current_lang = move || {
        params.with(|p| {
            p.as_ref()
                .ok()
                .map(|p| p.lang.clone())
                .unwrap_or_else(|| "en".to_string())
        })
    };

    // Shared manifest resource
    let manifest_res = use_context::<LocalResource<Option<Vec<DocMeta>>>>()
        .expect("manifest_res context not provided in parent DocEngine");

    // Search state
    let (query, set_query) = signal(String::new());
    let (show_results, set_show_results) = signal(false);

    // Filter results
    let search_results = Memo::new(move |_| {
        let q = query.get();
        if q.trim().is_empty() {
            return vec![];
        }
        let lang = current_lang();
        let q_lower = q.to_lowercase();

        if let Some(Some(ref manifest)) = manifest_res.get() {
            manifest
                .iter()
                .filter(|doc| doc.lang == lang)
                .filter(|doc| doc.title.to_lowercase().contains(&q_lower))
                .cloned()
                .take(8)
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    let on_blur = move |_| {
        leptos::prelude::set_timeout(
            move || {
                set_show_results.set(false);
            },
            std::time::Duration::from_millis(180),
        );
    };

    let placeholder_text = move || {
        if current_lang() == "zh" {
            "搜索文档标题..."
        } else {
            "Search titles..."
        }
    };

    view! {
        <header class="w-full glass border-b border-slate-200/50 dark:border-slate-800/50 shadow-sm transition-colors duration-300">
            <div class="max-w-7xl mx-auto px-4 md:px-8 h-16 flex items-center justify-between gap-4">
                <a href=move || format!("/{}/index", current_lang()) class="flex items-center gap-2 flex-shrink-0">
                    <span class="text-xl font-bold bg-gradient-to-r from-indigo-500 via-purple-500 to-cyan-500 bg-clip-text text-transparent">"Loki Auto"</span>
                    <span class="px-2 py-0.5 text-[10px] font-bold rounded-md bg-indigo-500/10 text-indigo-500 dark:bg-indigo-500/20 dark:text-indigo-400">"Docs"</span>
                </a>

                <div class="flex-1 max-w-md mx-auto relative">
                    <div class="relative">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <svg class="h-4 w-4 text-slate-400 dark:text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                            </svg>
                        </div>
                        <input
                            type="text"
                            placeholder=placeholder_text
                            prop:value=query
                            on:input=move |ev| {
                                set_query.set(event_target_value(&ev));
                                set_show_results.set(true);
                            }
                            on:focus=move |_| set_show_results.set(true)
                            on:blur=on_blur
                            class="w-full pl-9 pr-8 h-9 text-sm bg-slate-100/80 hover:bg-slate-200/60 focus:bg-white dark:bg-slate-800/60 dark:hover:bg-slate-800/80 dark:focus:bg-slate-900 border border-slate-200 hover:border-slate-300 focus:border-indigo-500/60 dark:border-slate-800/80 dark:hover:border-slate-700 dark:focus:border-indigo-500/50 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500/20 transition-all duration-200 text-slate-900 dark:text-white placeholder:text-slate-400 dark:placeholder:text-slate-500"
                        />
                        <Show when=move || !query.get().is_empty()>
                            <button
                                on:click=move |_| set_query.set(String::new())
                                class="absolute inset-y-0 right-0 pr-2.5 flex items-center text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
                            >
                                <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </Show>
                    </div>

                    <Show when=move || show_results.get() && !query.get().trim().is_empty()>
                        <div class="absolute left-0 right-0 mt-1.5 max-h-60 overflow-y-auto z-50 glass border border-slate-200/80 dark:border-slate-800/80 rounded-lg shadow-xl py-1 text-sm">
                            <Show
                                when=move || !search_results.get().is_empty()
                                fallback=move || view! {
                                    <div class="px-4 py-3 text-xs text-slate-500 text-center">
                                        {move || if current_lang() == "zh" { "未找到匹配的标题" } else { "No matching titles found" }}
                                    </div>
                                }
                            >
                                <For
                                    each=move || search_results.get()
                                    key=|doc| format!("{}-{}", doc.lang, doc.slug)
                                    children=move |doc| {
                                        let lang = doc.lang.clone();
                                        let slug = doc.slug.clone();
                                        let href = format!("/{}/{}", lang, slug);
                                        view! {
                                            <a
                                                href=href
                                                on:click=move |_| {
                                                    set_query.set(String::new());
                                                    set_show_results.set(false);
                                                }
                                                class="block px-4 py-2 hover:bg-indigo-50/50 dark:hover:bg-indigo-950/30 text-slate-700 dark:text-slate-300 transition-colors duration-150 cursor-pointer"
                                            >
                                                <div class="flex items-center justify-between gap-3">
                                                    <span class="font-medium text-slate-800 dark:text-slate-200 truncate">{doc.title.clone()}</span>
                                                    <span class="text-[10px] px-1.5 py-0.5 rounded bg-slate-100/60 dark:bg-slate-800/80 text-slate-500 dark:text-slate-400 max-w-[150px] truncate">{doc.category.clone()}</span>
                                                </div>
                                            </a>
                                        }
                                    }
                                />
                            </Show>
                        </div>
                    </Show>
                </div>

                <div class="flex items-center gap-4 flex-shrink-0">
                    <div class="flex items-center gap-1 bg-slate-200/50 dark:bg-slate-800/80 p-0.5 rounded-lg text-xs font-medium">
                        <a href="/en/index" class=move || {
                            let l = current_lang();
                            format!("px-2.5 py-1 rounded-md {}", if l == "en" { "bg-white dark:bg-slate-700 shadow-sm text-indigo-600 dark:text-white" } else { "text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-white" })
                        }>"EN"</a>
                        <a href="/zh/index" class=move || {
                            let l = current_lang();
                            format!("px-2.5 py-1 rounded-md {}", if l == "zh" { "bg-white dark:bg-slate-700 shadow-sm text-indigo-600 dark:text-white" } else { "text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-white" })
                        }>"中文"</a>
                    </div>
                </div>
            </div>
        </header>
    }
}
