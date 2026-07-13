use std::time::Duration;

use dioxus::prelude::*;
use dioxus_components::combobox::{Autocomplete, ComboboxEmpty, ComboboxOption};

const PEOPLE: &[SearchResult] = &[
    SearchResult::new("ada-lovelace", "Ada Lovelace", "Mathematician"),
    SearchResult::new("alan-turing", "Alan Turing", "Computer scientist"),
    SearchResult::new("grace-hopper", "Grace Hopper", "Computer scientist"),
    SearchResult::new("margaret-hamilton", "Margaret Hamilton", "Software engineer"),
    SearchResult::new("radia-perlman", "Radia Perlman", "Network engineer"),
    SearchResult::new("barbara-liskov", "Barbara Liskov", "Computer scientist"),
];

#[derive(Clone, Copy)]
struct SearchResult {
    value: &'static str,
    label: &'static str,
    description: &'static str,
}

impl SearchResult {
    const fn new(value: &'static str, label: &'static str, description: &'static str) -> Self {
        Self {
            value,
            label,
            description,
        }
    }
}

fn search_people(query: &str) -> Vec<SearchResult> {
    let query = query.trim().to_lowercase();
    PEOPLE
        .iter()
        .copied()
        .filter(|person| {
            person.label.to_lowercase().contains(&query)
                || person.description.to_lowercase().contains(&query)
        })
        .collect()
}

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| None::<String>);
    let mut query = use_signal(String::new);
    let mut results = use_signal(Vec::<SearchResult>::new);
    let mut loading = use_signal(|| false);
    let mut request_id = use_signal(|| 0u64);

    rsx! {
        div { style: "width: 24rem;",
            p {
                if loading() {
                    "Simulating a 350 ms server response..."
                } else if query().trim().is_empty() {
                    "Search is idle."
                } else {
                    "Server returned {results().len()} result(s)."
                }
            }
            Autocomplete {
                value: Some(value.into()),
                on_value_change: move |next| value.set(next),
                query: Some(query()),
                on_query_change: move |next: String| {
                    query.set(next.clone());
                    let next_request_id = request_id() + 1;
                    request_id.set(next_request_id);
                    results.set(Vec::new());

                    if next.trim().is_empty() {
                        loading.set(false);
                        return;
                    }

                    loading.set(true);
                    spawn(async move {
                        gloo_timers::future::sleep(Duration::from_millis(350)).await;
                        if request_id() == next_request_id {
                            results.set(search_people(&next));
                            loading.set(false);
                        }
                    });
                },
                placeholder: "Search people...",
                aria_label: "Server-backed people search",
                list_aria_label: "People search results",
                loading: loading(),
                if query().trim().is_empty() {
                    ComboboxEmpty { "Type a name or role to search." }
                } else if loading() {
                    div { role: "status", "Searching server..." }
                } else {
                    ComboboxEmpty { "Server returned no matches." }
                }
                for (index, person) in results().iter().enumerate() {
                    ComboboxOption::<String> {
                        index,
                        value: person.value.to_string(),
                        text_value: person.label.to_string(),
                        div { style: "display: grid; gap: 0.125rem;",
                            span { "{person.label}" }
                            span { style: "color: var(--muted-foreground); font-size: var(--text-sm);", "{person.description}" }
                        }
                    }
                }
            }
            p {
                "Selected: {value().unwrap_or_else(|| \"none\".to_string())}"
            }
        }
    }
}
