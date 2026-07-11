use crate::types::TocItem;
use leptos::prelude::*;

#[component]
pub fn Toc(toc: Vec<TocItem>, lang: String) -> impl IntoView {
    if toc.is_empty() {
        return view! { <div class="hidden"/> }.into_any();
    }

    let is_zh = lang == "zh";
    view! {
        <aside class="hidden lg:block w-56 shrink-0 font-sans">
            <div class="sticky top-16 pl-4 border-l border-slate-200/80 dark:border-slate-800/80">
                <h4 class="text-[11.5px] font-bold text-slate-400 dark:text-slate-500 tracking-wider mb-3">
                    {if is_zh { "文档目录" } else { "On this page" }}
                </h4>
                <ul class="space-y-2.5 text-xs font-medium">
                    {toc.into_iter().map(|item| {
                        let indent_class = match item.level {
                            3 => "pl-3 text-slate-500 dark:text-slate-400",
                            4 => "pl-6 text-slate-400 dark:text-slate-500",
                            _ => "text-slate-700 dark:text-slate-300",
                        };
                        view! {
                            <li>
                                <a href=format!("#{}", item.id)
                                   class=format!("hover:text-indigo-500 dark:hover:text-indigo-400 transition-colors duration-150 block truncate {}", indent_class)>
                                    {item.text}
                                </a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        </aside>
    }.into_any()
}
