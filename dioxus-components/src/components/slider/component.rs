use std::ops::Range;

use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::slider::{self};

use crate::components::input::{InputContent, InputLabel, InputWrapper};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Slider(
    #[props(default)] value: ReadSignal<Option<f64>>,
    #[props(default = 0.0)] default_value: f64,
    #[props(default = 0.0)] min: ReadSignal<f64>,
    #[props(default = 100.0)] max: ReadSignal<f64>,
    #[props(default = 1.0)] step: ReadSignal<f64>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default = true)] horizontal: bool,
    #[props(default)] inverted: bool,
    #[props(default)] on_value_change: Callback<f64>,
    /// Thumb tooltip label callback.
    #[props(default)] label: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    /// Field label rendered above the slider.
    #[props(default, into)] field_label: InputLabel,
    /// Description rendered below the field label.
    #[props(default, into)] description: InputContent,
    /// Error rendered below the slider.
    #[props(default, into)] error: InputContent,
    /// Marks the field as required.
    #[props(default = false)] required: bool,
    /// Shows the required asterisk without native validation.
    #[props(default = false)] with_asterisk: bool,
) -> Element {
    let is_disabled = (disabled)();
    rsx! {
        InputWrapper {
            label: field_label,
            description,
            error,
            required,
            with_asterisk,
            disabled: is_disabled,
            slider::Slider {
                class: Styles::dx_slider.to_string(),
                value,
                default_value,
                min,
                max,
                step,
                disabled,
                horizontal,
                inverted,
                on_value_change,
                label,
                attributes,
                slider::SliderTrack { class: Styles::dx_slider_track.to_string(),
                    slider::SliderRange { class: Styles::dx_slider_range.to_string() }
                    slider::SliderThumb { class: Styles::dx_slider_thumb.to_string() }
                }
            }
        }
    }
}

#[component]
pub fn RangeSlider(
    #[props(default)] value: ReadSignal<Option<Range<f64>>>,
    #[props(default = (0.0f64..100.0f64))] default_value: Range<f64>,
    #[props(default = 0.0)] min: ReadSignal<f64>,
    #[props(default = 100.0)] max: ReadSignal<f64>,
    #[props(default = 1.0)] step: ReadSignal<f64>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default = true)] horizontal: bool,
    #[props(default)] inverted: bool,
    #[props(default)] on_value_change: Callback<Range<f64>>,
    /// Thumb tooltip label callback.
    #[props(default)] label: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    /// Field label rendered above the slider.
    #[props(default, into)] field_label: InputLabel,
    /// Description rendered below the field label.
    #[props(default, into)] description: InputContent,
    /// Error rendered below the slider.
    #[props(default, into)] error: InputContent,
    /// Marks the field as required.
    #[props(default = false)] required: bool,
    /// Shows the required asterisk without native validation.
    #[props(default = false)] with_asterisk: bool,
) -> Element {
    let is_disabled = (disabled)();
    rsx! {
        InputWrapper {
            label: field_label,
            description,
            error,
            required,
            with_asterisk,
            disabled: is_disabled,
            slider::RangeSlider {
                class: Styles::dx_slider.to_string(),
                value,
                default_value,
                min,
                max,
                step,
                disabled,
                horizontal,
                inverted,
                on_value_change,
                label,
                attributes,
                slider::SliderTrack { class: Styles::dx_slider_track.to_string(),
                    slider::SliderRange { class: Styles::dx_slider_range.to_string() }
                    slider::SliderThumb { class: Styles::dx_slider_thumb.to_string(), index: 0usize }
                    slider::SliderThumb { class: Styles::dx_slider_thumb.to_string(), index: 1usize }
                }
            }
        }
    }
}
