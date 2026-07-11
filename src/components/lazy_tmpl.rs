use leptos::prelude::*;
use minijinja::{value::Value, Environment};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct LazyTemplateManager {
    pub cache: Arc<Mutex<HashMap<String, String>>>,
    pub pending_tasks: Arc<Mutex<HashSet<String>>>,
}

impl LazyTemplateManager {
    pub fn register_in_env(&self, env: &mut Environment, set_render_version: WriteSignal<u32>) {
        let cache = self.cache.clone();
        let pending = self.pending_tasks.clone();

        env.add_function("has_template", move |name: String| -> bool {
            let has = cache.lock().unwrap().contains_key(&name);
            if !has {
                let mut pending_ref = pending.lock().unwrap();
                if !pending_ref.contains(&name) {
                    pending_ref.insert(name.clone());

                    let cache_clone = cache.clone();
                    let pending_clone = pending.clone();
                    let name_clone = name.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let origin = web_sys::window()
                            .and_then(|w| w.location().origin().ok())
                            .unwrap_or_default();
                        let url = format!("{}/templates/{}", origin, name_clone);
                        let client = reqwest::Client::new();
                        if let Ok(resp) = client.get(&url).send().await {
                            if let Ok(source) = resp.text().await {
                                cache_clone
                                    .lock()
                                    .unwrap()
                                    .insert(name_clone.clone(), source);
                                set_render_version.update(|v| *v += 1);
                            }
                        }
                        pending_clone.lock().unwrap().remove(&name_clone);
                    });
                }
            }
            has
        });

        env.add_function("placeholder", move |name: String| -> Value {
            let placeholder_id = format!("lazy-tpl-{}", name.replace('.', "-"));
            let html_skeleton = format!(
                "<div id=\"{}\" class=\"animate-pulse bg-slate-200 dark:bg-slate-800/80 h-16 rounded-md flex items-center justify-center text-sm text-slate-500\">Loading {}...</div>",
                placeholder_id, name
            );
            Value::from_safe_string(html_skeleton)
        });
    }
}
