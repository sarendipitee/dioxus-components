use dioxus::prelude::*;

use dioxus_primitives::{
    dioxus_attributes::attributes,
    merge_attributes,
    time_picker::{
        self, TimePickerHourSegmentProps, TimePickerInputProps, TimePickerMinuteSegmentProps,
        TimePickerSeparatorProps,
    },
};
use time::Time;

#[css_module("/src/components/time_picker/style.css")]
struct Styles;

/// The props for the styled [`TimePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<Time>>,

    /// The selected time.
    #[props(default)]
    pub selected_time: ReadSignal<Option<Time>>,

    /// Whether the time picker is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the time picker is read-only.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the selectable time range.
    #[props(default = time::macros::time!(00:00))]
    pub min_time: Time,

    /// Upper limit of the selectable time range.
    #[props(default = time::macros::time!(23:59))]
    pub max_time: Time,

    /// Callback when display hour placeholder.
    #[props(default = Callback::new(|_| "H".to_string()))]
    pub on_format_hour_placeholder: Callback<(), String>,

    /// Callback when display minute placeholder.
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_minute_placeholder: Callback<(), String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the time picker element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # TimePicker
///
/// A styled segmented time picker that renders hour and minute spinbutton segments.
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_time_picker.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {
            time_picker::TimePicker {
                on_value_change: props.on_value_change,
                selected_time: props.selected_time,
                disabled: props.disabled,
                read_only: props.read_only,
                min_time: props.min_time,
                max_time: props.max_time,
                roving_loop: props.roving_loop,
                attributes: merged,
                TimePickerInput {
                    on_format_hour_placeholder: props.on_format_hour_placeholder,
                    on_format_minute_placeholder: props.on_format_minute_placeholder,
                }
            }
        }
    }
}

#[component]
pub(crate) fn TimePickerInput(props: TimePickerInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_time_picker_group.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let extra_children = props.children;

    rsx! {
        time_picker::TimePickerInput {
            on_format_hour_placeholder: props.on_format_hour_placeholder,
            on_format_minute_placeholder: props.on_format_minute_placeholder,
            attributes: merged,
            time_picker::TimePickerInputValue {
                on_format_hour_placeholder: props.on_format_hour_placeholder,
                on_format_minute_placeholder: props.on_format_minute_placeholder,
                TimePickerHourSegment {}
                TimePickerSeparator {}
                TimePickerMinuteSegment {}
            }
            if let Some(extra_children) = extra_children {
                {extra_children}
            }
        }
    }
}

#[component]
pub(crate) fn TimePickerHourSegment(props: TimePickerHourSegmentProps) -> Element {
    rsx! {
        time_picker::TimePickerHourSegment {
            class: Styles::dx_time_segment.to_string(),
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn TimePickerMinuteSegment(props: TimePickerMinuteSegmentProps) -> Element {
    rsx! {
        time_picker::TimePickerMinuteSegment {
            class: Styles::dx_time_segment.to_string(),
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn TimePickerSeparator(props: TimePickerSeparatorProps) -> Element {
    rsx! {
        time_picker::TimePickerSeparator {
            class: Styles::dx_time_segment.to_string(),
            symbol: props.symbol,
            attributes: props.attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::time;

    #[component]
    fn TimePickerWithDefaultInput() -> Element {
        rsx! {
            TimePicker {
                selected_time: Some(time!(09:30)),
            }
        }
    }

    #[test]
    fn time_picker_renders_default_input_when_children_are_omitted() {
        let mut dom = VirtualDom::new(TimePickerWithDefaultInput);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("09"));
        assert!(html.contains("30"));
        assert!(html.contains("dx-time-picker"));
    }
}
