The Combobox component family powers editable inputs that reveal a list of options while users type.
In the demos on this page, use `Combobox` when you need full control of query/value flow, `Autocomplete` for simple string-driven lookup behavior, and `MultiSelect`/`TagsInput` when selections should stay visible after choice.

`Combobox` itself is the stateful core: it owns open/close, active highlight movement, option registration, and submit semantics for a popover listbox.
Higher-level wrappers still keep their own query and selection model, which is why demos below vary by API shape instead of reusing one universal pattern.

In this page’s demos, filtering is intentionally left to your data pipeline. The component only renders options in the order you pass to `ComboboxOption`; if you need rank-by-query behavior, sort your source data before rendering and pass matching `index` values with that order.

## Demos

- `Combobox` demonstrates the primitive surface for custom selection behavior.
- `Autocomplete` demonstrates controlled text search input with submitted option values mapped back to displayed labels.
- `MultiSelect` demonstrates bounded multi-selection with query filtering and rendered chips.
- `TagsInput` demonstrates parsing and removing tokenized entries in a pill-based input.
- `VirtualizedComboboxOptions` demonstrates rendering thousands of options efficiently while keeping stable `ComboboxOption` indexes/ids.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);
let mut query = use_signal(String::new);

Combobox::<String> {
    value: Some(value.into()),
    on_value_change: move |next: Option<String>| {
        value.set(next);
    },
    query: Some(query()),
    on_query_change: move |next| query.set(next),
    placeholder: "Select framework...",
    aria_label: "Select framework",
    list_aria_label: "Frameworks",
    ComboboxEmpty { "No framework found." }
    ComboboxOption::<String> {
        index: 0usize,
        value: "next".to_string(),
        text_value: "Next.js",
        "Next.js"
    }
}
```

## MultiSelect

```rust
MultiSelect::<String> {
    default_values: vec!["mushroom".to_string()],
    max_values: 3usize,
    render_value: |value: String| rsx! { "{value}" },
    placeholder: "Pick toppings...",
    ComboboxOption::<String> {
        index: 0usize,
        value: "mushroom".to_string(),
        text_value: "Mushroom",
        "Mushroom"
    }
}
```

## Virtualized Options

```rust
use dioxus_primitives::combobox::VirtualizedComboboxOptions;

Combobox::<String> {
    VirtualizedComboboxOptions {
        count: 1000usize,
        estimate_size: |_: usize| 36,
        render_option: |index: usize| rsx! {
            ComboboxOption::<String> {
                index,
                value: format!("option-{index}"),
                text_value: format!("Option {index}"),
                "Option {index}"
            }
        }
    }
}
```
