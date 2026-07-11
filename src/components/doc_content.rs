use crate::components::toc::Toc;
use crate::types::{DocMeta, TocItem};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism, js_name = highlightAll)]
    fn prism_highlight_all();
}

fn parse_markdown_with_toc(md: &str) -> (String, Vec<TocItem>) {
    use pulldown_cmark::{Event, Parser, Tag, TagEnd};

    let parser = Parser::new(md);
    let mut events = Vec::new();
    let mut toc = Vec::new();

    let mut in_heading = None;
    let mut heading_text = String::new();

    for ev in parser {
        match ev {
            Event::Start(Tag::Heading { level, .. }) => {
                let lvl = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                in_heading = Some(lvl);
                heading_text.clear();
            }
            Event::Text(ref text) if in_heading.is_some() => {
                heading_text.push_str(text);
            }
            Event::Code(ref text) if in_heading.is_some() => {
                heading_text.push_str(text);
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(l) = in_heading {
                    let id = heading_text
                        .to_lowercase()
                        .chars()
                        .filter_map(|c| {
                            if c.is_alphanumeric() {
                                Some(c)
                            } else if c.is_whitespace() || c == '-' || c == '_' {
                                Some('-')
                            } else {
                                None
                            }
                        })
                        .collect::<String>()
                        .replace("--", "-");

                    let id = if id.is_empty() {
                        format!("heading-{}", toc.len())
                    } else {
                        id
                    };

                    toc.push(TocItem {
                        level: l,
                        id: id.clone(),
                        text: heading_text.clone(),
                    });

                    let heading_html_start = format!(
                        "<h{} id=\"{}\" class=\"group flex items-center gap-2 scroll-mt-20\">",
                        l, id
                    );
                    let heading_html_end = format!("<a href=\"#{}\" class=\"opacity-0 group-hover:opacity-100 text-indigo-500 select-none transition-opacity duration-150\">\"#\"</a></h{}>", id, l);

                    events.push(Event::Html(heading_html_start.into()));
                    events.push(Event::Text(heading_text.clone().into()));
                    events.push(Event::Html(heading_html_end.into()));
                }
                in_heading = None;
            }
            _ => {
                if in_heading.is_none() {
                    events.push(ev);
                }
            }
        }
    }

    let mut html_content = String::new();
    pulldown_cmark::html::push_html(&mut html_content, events.into_iter());
    (html_content, toc)
}

use crate::DocEngineParams;
use leptos_router::hooks::use_params;

#[component]
pub fn DocContent() -> impl IntoView {
    let params = use_params::<DocEngineParams>();

    let (is_loading, set_is_loading) = signal(false);

    // Bind LocalResource directly to the params signal
    let doc_res = LocalResource::new(move || {
        let p_res = params.get();
        set_is_loading.set(true);
        async move {
            let res = async {
                let p = p_res.ok()?;
                let l = p.lang.clone();
                let doc_path = p.doc_path.clone().unwrap_or_else(|| "index".to_string());

                let origin = web_sys::window()
                    .and_then(|w| w.location().origin().ok())
                    .unwrap_or_default();
                let md_url = format!("{}/content/{}/{}.md", origin, l, doc_path);
                let client = reqwest::Client::new();
                let text = client.get(&md_url).send().await.ok()?.text().await.ok()?;
                Some((l, text))
            }
            .await;
            set_is_loading.set(false);
            res
        }
    });

    // Retrieve the shared manifest resource from the parent DocEngine context
    let manifest_res = use_context::<LocalResource<Option<Vec<DocMeta>>>>()
        .expect("manifest_res context not provided in parent DocEngine");

    // Highlight code blocks on content mount or update
    Effect::new(move |_| {
        if let Some(Some(_)) = doc_res.get() {
            // Synchronize with the browser's next animation repaint frame (<16ms)
            request_animation_frame(move || {
                prism_highlight_all();
            });
        }
    });

    let show_overlay = move || doc_res.get().is_some() && is_loading.get();

    view! {
        <Show when=show_overlay>
            <div class="fixed md:left-56 top-16 bottom-0 right-0 left-0 bg-slate-50/30 dark:bg-slate-950/30 backdrop-blur-[1px] flex items-center justify-center z-50 animate-in fade-in duration-200">
                <div class="flex flex-col items-center gap-3 p-5 rounded-2xl bg-white/80 dark:bg-slate-900/80 shadow-lg border border-slate-200/50 dark:border-slate-800/50 backdrop-blur-md animate-in zoom-in-95 duration-200">
                    <div class="relative w-12 h-12 flex items-center justify-center">
                        <div class="absolute inset-0 rounded-full border-4 border-slate-200/60 dark:border-slate-800/60"></div>
                        <div class="absolute inset-0 rounded-full border-4 border-t-indigo-600 dark:border-t-indigo-400 border-r-transparent border-b-transparent border-l-transparent animate-spin"></div>
                    </div>
                    <span class="text-xs font-semibold tracking-wide text-slate-600 dark:text-slate-400 uppercase select-none animate-pulse-soft">"Loading..."</span>
                </div>
            </div>
        </Show>

        <Transition fallback=|| view! {
            <div class="animate-pulse space-y-4">
                <div class="h-8 bg-slate-200 dark:bg-slate-800 rounded w-1/3"/>
                <div class="h-4 bg-slate-200 dark:bg-slate-800 rounded w-1/4"/>
                <div class="space-y-2 mt-8">
                    <div class="h-4 bg-slate-200 dark:bg-slate-800 rounded"/>
                    <div class="h-4 bg-slate-200 dark:bg-slate-800 rounded w-5/6"/>
                </div>
            </div>
        }>
            {move || {
                match doc_res.get() {
                    None => Some(view! { <div/> }.into_any()),
                    Some(None) => Some(view! {
                        <div class="max-w-xl mx-auto py-16 px-6 text-center">
                            <div class="p-4 bg-rose-50 border border-rose-100 rounded-xl dark:bg-rose-950/20 dark:border-rose-900/30">
                                <h2 class="text-rose-600 dark:text-rose-400 font-bold mb-2">"Document Not Found"</h2>
                                <p class="text-sm text-rose-500/90">"The requested markdown resource could not be retrieved from the server."</p>
                            </div>
                        </div>
                    }.into_any()),
                    Some(Some((current_lang, raw_md))) => {
                        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
                        let parsed_matter = matter.parse(&raw_md);
                        let doc_meta: DocMeta = parsed_matter
                            .data
                            .and_then(|d| d.deserialize().ok())
                            .unwrap_or_default();

                        let (html_content, toc) = parse_markdown_with_toc(&parsed_matter.content);

                        // Parse Category Breadcrumbs Hrefs
                        let parts: Vec<&str> = doc_meta
                            .category
                            .split('/')
                            .filter(|s| !s.is_empty())
                            .collect();

                        let mut breadcrumbs = Vec::new();
                        if let Some(Some(manifest)) = manifest_res.get() {
                            let mut path_accum = String::new();
                            for part in parts {
                                if !path_accum.is_empty() {
                                    path_accum.push('/');
                                }
                                path_accum.push_str(part);

                                // Find the first document belonging to this category subset
                                let matched_doc = manifest.iter().find(|doc| {
                                    doc.lang == current_lang && (
                                        doc.category == path_accum ||
                                        doc.category.starts_with(&format!("{}/", path_accum))
                                    )
                                });

                                let href = if let Some(d) = matched_doc {
                                    format!("/{}/{}", current_lang, d.slug)
                                } else {
                                    format!("/{}/index", current_lang)
                                };

                                breadcrumbs.push((part.to_string(), href));
                            }
                        }

                        Some(view! {
                            <div class="flex flex-col lg:flex-row gap-8 w-full pl-6 pr-4 md:pr-8">
                                <div class="flex-1 min-w-0 max-w-4xl">
                                    <article class="line-numbers prose prose-sm prose-slate dark:prose-invert max-w-none">
                                        // Clickable Breadcrumbs Navigation
                                        <nav class="flex items-center flex-wrap gap-1.5 text-xs text-slate-500 mb-6 font-sans">
                                            <a href=format!("/{}/index", current_lang) class="flex items-center text-slate-600 dark:text-slate-400 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                                                <svg class="w-4 h-4 shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                                    <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"/>
                                                </svg>
                                            </a>

                                            {breadcrumbs.into_iter().map(|(name, href)| {
                                                view! {
                                                    <svg class="w-3.5 h-3.5 text-slate-400 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                                                    </svg>
                                                    <a href=href class="text-slate-600 dark:text-slate-400 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors font-medium">
                                                        {name}
                                                    </a>
                                                }
                                            }).collect::<Vec<_>>()}

                                            <svg class="w-3.5 h-3.5 text-slate-400 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                                            </svg>
                                            <span class="bg-slate-100 dark:bg-slate-800 text-teal-600 dark:text-teal-400 px-2.5 py-0.5 rounded-full font-medium border border-slate-200/50 dark:border-slate-800/40 shrink-0">
                                                {doc_meta.title.clone()}
                                            </span>
                                        </nav>
                                        <h1 class="text-2xl font-bold tracking-tight mb-1.5 text-slate-900 dark:text-white">
                                            {doc_meta.title.clone()}
                                        </h1>
                                        <div class="flex flex-wrap items-center gap-3 text-xs text-slate-500 mb-3 border-b border-slate-200/60 dark:border-slate-800 pb-4">
                                            <span>"Updated: " {doc_meta.date.clone()}</span>
                                            <span>"•"</span>
                                            <span>"Language: " <span class="uppercase font-semibold text-cyan-600 dark:text-cyan-400">{current_lang.clone()}</span></span>
                                        </div>
                                        <div class="markdown-body leading-7 text-slate-700 dark:text-slate-300" inner_html=html_content />
                                    </article>

                                    <footer class="mt-12 pt-6 border-t border-slate-200/50 dark:border-slate-800/60 text-xs text-slate-400 dark:text-slate-500">
                                        <span>
                                            "Copyright © 2026 "
                                            <a href="https://loki4agent.com" target="_blank" rel="noopener noreferrer" class="hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors font-medium">
                                                "loki4agent.com"
                                            </a>
                                            ". All rights reserved."
                                        </span>
                                    </footer>
                                </div>

                                <Toc toc=toc lang=current_lang.clone()/>
                            </div>
                        }.into_any())
                    }
                }
            }}
        </Transition>
    }
}
