use dioxus::prelude::*;
use dioxus_components::select::*;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Orangeade,
    Strawberry,
    Watermelon,
    Other,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Orangeade => "🧃",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
            Fruit::Other => "✨",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| Some(Fruit::Apple));
    let mut callback_count = use_signal(|| 0usize);
    let selected_text =
        use_memo(move || value().map_or_else(|| "None".to_string(), |fruit| fruit.to_string()));
    let fruits = Fruit::iter()
        .filter(|fruit| *fruit != Fruit::Other)
        .enumerate()
        .map(|(i, f)| {
            rsx! {
                SelectOption::<Fruit> { index: i, value: f, text_value: "{f}",
                    disabled: matches!(f, Fruit::Orange),
                    "{f.emoji()} {f}"
                }
            }
        });

    rsx! {
        div {
            Select::<Fruit> {
                id: "fruit-select",
                "data-testid": "fruit-select-root",
                "data-audit": "forwarded",
                label: "Fruit selection",
                value: Some(value.into()),
                on_value_change: move |next| {
                    callback_count += 1;
                    value.set(next);
                },
                width: "12rem",
                SelectGroup {
                    SelectGroupLabel { "Fruits" }
                    {fruits}
                }
                SelectGroup {
                    SelectGroupLabel { "Other" }
                    SelectOption::<Fruit> {
                        index: Fruit::COUNT - 1,
                        value: Fruit::Other,
                        text_value: "Other",
                        "Other"
                    }
                }
            }
            output {
                "data-testid": "fruit-select-value",
                aria_live: "polite",
                "Selected: {selected_text}"
            }
            output {
                "data-testid": "fruit-select-callback-count",
                aria_live: "polite",
                "Callbacks: {callback_count}"
            }
            button {
                "data-testid": "select-outside",
                type: "button",
                "Outside button"
            }
        }
        Select::<Fruit> {
            id: "disabled-group-select",
            "data-testid": "disabled-group-select-root",
                label: "Disabled group selection",
                width: "12rem",
                SelectGroup { disabled: true,
                    SelectGroupLabel { "Unavailable fruits" }
                    SelectOption::<Fruit> {
                        index: 0,
                        value: Fruit::Apple,
                        text_value: "Apple",
                        "🍎 Apple"
                    }
                    SelectOption::<Fruit> {
                        index: 1,
                        value: Fruit::Banana,
                        text_value: "Banana",
                        "🍌 Banana"
                    }
                }
        }
        Select::<Fruit> {
            id: "disabled-root-select",
            "data-testid": "disabled-root-select-root",
                label: "Disabled root selection",
                disabled: true,
                width: "12rem",
                SelectGroup {
                    SelectGroupLabel { "Disabled root fruits" }
                    SelectOption::<Fruit> {
                        index: 0,
                        value: Fruit::Apple,
                        text_value: "Apple",
                        "🍎 Apple"
                    }
                }
            }
    }
}
