use dioxus_components::virtual_list::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "dx-virtual-list-demo",
            p { class: "dx-virtual-list-subtitle", "Scroll this page to verify virtualized rendering with dynamic row heights." }
            style { r#".dx_virtual_list_container {{
  position: relative;
  max-height: 36rem;
  contain: layout paint;
  overflow-y: auto;
}}

.dx-virtual-list-demo {{
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  gap: 0.75rem;
}}

.dx-virtual-list-demo .dx-virtual-list-subtitle {{
  margin: 0;
  margin-bottom: 0.75rem;
  color: var(--fg-muted);
  font-size: var(--text-sm);
}}

.dx-virtual-list-card {{
  padding: 0.75rem 0.9rem;
  border: 1px solid var(--surface-border);
  border-radius: 0.625rem;
  background: var(--surface-hover);
}}

.dx-virtual-list-card h3 {{
  margin: 0 0 0.3rem;
  color: var(--fg);
  font-size: var(--text-md);
}}

.dx-virtual-list-card p {{
  margin: 0;
  color: var(--fg);
  font-size: var(--text-sm);
  line-height: 1.4;
}}"# }
            VirtualList {
                count: 2000usize,
                buffer: 12usize,
                // Estimate height based on content pattern (idx % 6 repeats)
                // Measured: min=68.4px (0 repeats), max=127.2px (5 repeats)
                estimate_size: |idx| {
                    let base_height = 68;
                    let repeats: usize = idx % 6;
                    let lines = repeats.div_ceil(2); // 2 repeats per line
                    let per_line = 20;
                    base_height + lines as u32 * per_line
                },
                render_item: move |idx: usize| {
                    let extra_text = "Extra content to vary row height. ".repeat(idx % 6);
                    rsx! {
                        article { class: "dx-virtual-list-card",
                            h3 { "#{idx + 1} - Item {idx + 1}" }
                            p { "Virtualized row preview. Index = {idx}" }
                            p { "{extra_text}" }
                        }
                    }
                },
            }
        }
    }
}
