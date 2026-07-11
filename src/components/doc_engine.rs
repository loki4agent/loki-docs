use crate::components::doc_content::DocContent;
use crate::components::header::Header;
use crate::components::sidebar::SidebarNode;
use crate::types::{DocMeta, MenuNode};
use crate::DocEngineParams;
use leptos::prelude::*;
use leptos_router::hooks::*;
use std::collections::HashSet;

fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let mut a_chars = a.chars().peekable();
    let mut b_chars = b.chars().peekable();

    loop {
        match (a_chars.peek(), b_chars.peek()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(&ac), Some(&bc)) => {
                if ac.is_ascii_digit() && bc.is_ascii_digit() {
                    let mut a_num: u64 = 0;
                    while let Some(&c) = a_chars.peek() {
                        if c.is_ascii_digit() {
                            a_num = a_num * 10 + c.to_digit(10).unwrap() as u64;
                            a_chars.next();
                        } else {
                            break;
                        }
                    }

                    let mut b_num: u64 = 0;
                    while let Some(&c) = b_chars.peek() {
                        if c.is_ascii_digit() {
                            b_num = b_num * 10 + c.to_digit(10).unwrap() as u64;
                            b_chars.next();
                        } else {
                            break;
                        }
                    }

                    if a_num != b_num {
                        return a_num.cmp(&b_num);
                    }
                } else {
                    if ac != bc {
                        return ac.cmp(&bc);
                    }
                    a_chars.next();
                    b_chars.next();
                }
            }
        }
    }
}

#[component]
pub fn DocEngine() -> impl IntoView {
    let params = use_params::<DocEngineParams>();
    let (expanded_cats, set_expanded_cats) = signal::<HashSet<String>>(HashSet::new());

    let current_lang = move || {
        params.with(|p| {
            p.as_ref()
                .ok()
                .map(|p| p.lang.clone())
                .unwrap_or_else(|| "en".to_string())
        })
    };

    let active_slug = move || {
        params.with(|p| {
            p.as_ref()
                .ok()
                .and_then(|p| p.doc_path.clone())
                .unwrap_or_else(|| "index".to_string())
        })
    };

    let manifest_res = LocalResource::new(move || async move {
        let origin = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_default();
        let meta_url = format!("{}/meta.json", origin);
        let client = reqwest::Client::new();
        let text = client.get(&meta_url).send().await.ok()?.text().await.ok()?;
        let manifest: Vec<DocMeta> = serde_json::from_str(&text).unwrap_or_default();
        Some(manifest)
    });

    provide_context(manifest_res);

    Effect::new(move |_| {
        let slug = active_slug();
        let lang = current_lang();
        if let Some(Some(manifest)) = manifest_res.get() {
            if let Some(doc) = manifest.iter().find(|d| d.lang == lang && d.slug == slug) {
                set_expanded_cats.update(|set| {
                    let parts: Vec<&str> =
                        doc.category.split('/').filter(|s| !s.is_empty()).collect();
                    let mut current_path = String::new();
                    for part in parts {
                        if !current_path.is_empty() {
                            current_path.push('/');
                        }
                        current_path.push_str(part);
                        set.insert(current_path.clone());
                    }
                });
            }
        }
    });

    let build_tree = move |manifest: &[DocMeta], lang: &str, active_slug: &str| -> Vec<MenuNode> {
        let mut root = MenuNode::default();
        for doc in manifest.iter() {
            if doc.lang == lang {
                let parts: Vec<&str> = doc.category.split('/').filter(|s| !s.is_empty()).collect();
                let mut curr_node = &mut root;
                let mut path_accumulator = String::new();
                for part in parts {
                    if !path_accumulator.is_empty() {
                        path_accumulator.push('/');
                    }
                    path_accumulator.push_str(part);

                    let idx =
                        if let Some(i) = curr_node.children.iter().position(|c| c.name == part) {
                            i
                        } else {
                            curr_node.children.push(MenuNode {
                                name: part.to_string(),
                                full_path: path_accumulator.clone(),
                                is_open: false,
                                is_active: false,
                                slug: None,
                                children: Vec::new(),
                                order: 1000,
                            });
                            curr_node.children.len() - 1
                        };
                    curr_node = &mut curr_node.children[idx];
                }
                curr_node.children.push(MenuNode {
                    name: doc.title.clone(),
                    full_path: path_accumulator.clone(),
                    is_open: false,
                    is_active: doc.slug == active_slug,
                    slug: Some(doc.slug.clone()),
                    children: Vec::new(),
                    order: doc.order,
                });
            }
        }

        fn post_process_tree(node: &mut MenuNode, active_slug: &str) -> (bool, bool) {
            if node.slug.as_deref() == Some(active_slug) {
                node.is_active = true;
                return (true, true);
            }
            let mut any_active = false;
            let mut any_open = false;
            for child in node.children.iter_mut() {
                let (c_active, c_open) = post_process_tree(child, active_slug);
                if c_active {
                    any_active = true;
                }
                if c_open {
                    any_open = true;
                }
            }
            if !node.children.is_empty() {
                node.is_open = any_open || any_active;

                // Propagate minimum order of children to parent category node
                if let Some(min_order) = node.children.iter().map(|c| c.order).min() {
                    node.order = min_order;
                }

                node.children.sort_by(|a, b| {
                    let a_is_cat = !a.children.is_empty();
                    let b_is_cat = !b.children.is_empty();
                    if a_is_cat != b_is_cat {
                        b_is_cat.cmp(&a_is_cat)
                    } else if a.order != b.order {
                        a.order.cmp(&b.order)
                    } else {
                        natural_cmp(&a.name, &b.name)
                    }
                });
            }
            (any_active || node.is_active, any_open || node.is_open)
        }

        post_process_tree(&mut root, active_slug);
        root.children
    };

    let navigate = use_navigate();
    let on_container_click = move |ev: web_sys::MouseEvent| {
        let navigate = navigate.clone();
        if let Some(target) = ev.target() {
            use leptos::wasm_bindgen::JsCast;
            let mut curr_node = Some(target.unchecked_into::<web_sys::Node>());
            while let Some(node) = curr_node {
                if node.node_type() == web_sys::Node::ELEMENT_NODE {
                    let el = node.unchecked_into::<web_sys::Element>();
                    if el.tag_name().to_lowercase() == "a" {
                        if let Some(href) = el.get_attribute("href") {
                            if href.starts_with('#') {
                                break;
                            }
                            if href.starts_with('/') && !href.starts_with("//") {
                                ev.prevent_default();
                                navigate(&href, Default::default());
                            }
                        }
                        break;
                    }
                    curr_node = el.parent_node();
                } else {
                    curr_node = node.parent_node();
                }
            }
        }
    };

    view! {
        <div class="h-screen flex flex-col font-sans bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 overflow-hidden" on:click=on_container_click>
            <Header />

            <div class="flex-1 w-full flex pl-4 md:pl-2 pr-0 overflow-hidden">
                <aside class="hidden md:block w-56 shrink-0 h-full overflow-y-auto border-r border-slate-200 dark:border-slate-800/50 pr-1 pt-3 pb-8">
                    <Suspense fallback=|| view! {
                        <div class="animate-pulse bg-slate-200/50 dark:bg-slate-800/40 h-48 rounded-lg"/>
                    }>
                        {move || {
                            match manifest_res.get() {
                                None => Some(view! { <div class="animate-pulse bg-slate-200/50 dark:bg-slate-800/40 h-48 rounded-lg"/> }.into_any()),
                                Some(None) => Some(view! { <div class="text-xs text-rose-500">"Failed loading manifest"</div> }.into_any()),
                                Some(Some(manifest)) => {
                                    let tree = build_tree(&manifest, &current_lang(), &active_slug());
                                    Some(view! {
                                        <nav class="space-y-2 pr-1">
                                            {tree.into_iter().map(|node| {
                                                view! {
                                                    <SidebarNode node=node lang=current_lang() expanded_cats=expanded_cats set_expanded_cats=set_expanded_cats/>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </nav>
                                    }.into_any())
                                }
                            }
                        }}
                    </Suspense>
                </aside>

                <main class="flex-1 min-w-0 h-full overflow-y-auto pt-3 pb-8">
                    <DocContent />
                </main>
            </div>
        </div>
    }
}
