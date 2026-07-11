use crate::types::MenuNode;
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
fn MenuChevron(is_open: impl Fn() -> bool + Send + Sync + Clone + 'static) -> impl IntoView {
    view! {
        <svg class=move || format!("w-3.5 h-3.5 shrink-0 transform transition-transform duration-200 text-slate-400 {}", if is_open() { "rotate-90" } else { "" })
             width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M9 5l7 7-7 7" />
        </svg>
    }
}

#[component]
pub fn SidebarNode(
    node: MenuNode,
    lang: String,
    expanded_cats: ReadSignal<HashSet<String>>,
    set_expanded_cats: WriteSignal<HashSet<String>>,
    #[prop(default = 0)] depth: usize,
) -> impl IntoView {
    let style_str = format!(
        "padding-left: calc(var(--sidebar-indent-base) + {} * var(--sidebar-indent-step)); padding-right: var(--sidebar-padding-right);",
        depth
    );

    if !node.children.is_empty() {
        let name = node.name.clone();
        let full_path = node.full_path.clone();
        let full_path_clone = full_path.clone();
        let set_expanded = set_expanded_cats.clone();

        let on_summary_click = move |ev: web_sys::MouseEvent| {
            ev.prevent_default();
            let path = full_path.clone();
            set_expanded.update(|set| {
                if set.contains(&path) {
                    set.remove(&path);
                } else {
                    set.insert(path);
                }
            });
        };

        let full_path_clone2 = full_path_clone.clone();
        let is_expanded = move || expanded_cats.with(|set| set.contains(&full_path_clone));
        let is_expanded_chevron = move || expanded_cats.with(|set| set.contains(&full_path_clone2));

        let heading_class = if depth == 0 {
            "text-sm font-semibold text-slate-800 dark:text-slate-200 tracking-wide"
        } else {
            "text-[13px] font-medium text-slate-700 dark:text-slate-300"
        };

        view! {
            <details class="[&_summary::-webkit-details-marker]:hidden"
                     open=is_expanded>
                <summary class="flex items-center justify-between py-1 text-slate-800 dark:text-slate-200 cursor-pointer hover:bg-slate-200/30 dark:hover:bg-slate-800/40 rounded-lg transition-all duration-150 select-none focus:outline-none"
                         style=style_str
                         on:click=on_summary_click>
                    <span class=heading_class>
                        {name}
                    </span>
                    <MenuChevron is_open=is_expanded_chevron/>
                </summary>
                <div class="space-y-1 mt-1 mb-1.5">
                    {node.children.into_iter().map(|child| {
                        view! {
                            <SidebarNode node=child lang=lang.clone() expanded_cats=expanded_cats set_expanded_cats=set_expanded_cats depth=depth + 1/>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </details>
        }.into_any()
    } else {
        let name = node.name.clone();
        let dest_slug = node.slug.clone().unwrap_or_default();
        let is_active = node.is_active;

        let active_class = move || {
            if is_active {
                "bg-slate-200/50 text-indigo-600 dark:bg-slate-800/60 dark:text-indigo-400 font-semibold"
            } else {
                "text-slate-600 dark:text-slate-400 hover:bg-slate-200/30 dark:hover:bg-slate-800/40 hover:text-slate-800 dark:hover:text-slate-200 font-normal"
            }
        };

        view! {
            <a href=format!("/{}/{}", lang, dest_slug)
               class=move || format!("flex items-center py-1 text-[13px] rounded-lg transition-all duration-150 focus:outline-none {}", active_class())
               style=style_str>
                {name}
            </a>
        }.into_any()
    }
}
