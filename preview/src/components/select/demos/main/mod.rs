use dioxus::prelude::*;
use dioxus_components::select::*;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::Display)]
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
    let fruits = Fruit::iter().enumerate().map(|(index, fruit)| {
        rsx! {
            SelectOption::<Fruit> { index, value: fruit, text_value: "{fruit}",
                "{fruit.emoji()} {fruit}"
            }
        }
    });

    rsx! {
        Select::<Fruit> {
            label: "Select a fruit",
            value: Some(value.into()),
            on_value_change: move |next| value.set(next),
            width: "12rem",
            SelectGroup {
                SelectGroupLabel { "Fruits" }
                {fruits}
            }
        }
    }
}
