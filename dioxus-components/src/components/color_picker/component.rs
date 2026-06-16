use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::color_picker::{self, Color, ColorAreaProps, ColorPickerContext};
use dioxus_primitives::slider::*;
use palette::{encoding, FromColor, Hsv, RgbHue, Srgb};

#[component_styles("./style.css")]
struct Styles;

fn format_color_hex(color: Color) -> String {
    format!("#{color:X}")
}

fn color_hex_from_hsv(color: Hsv<encoding::Srgb, f64>) -> String {
    let rgb: Color = Srgb::<f64>::from_color(color).into_format();
    format_color_hex(rgb)
}

fn is_zero_hue_alias(value: f64) -> bool {
    value.rem_euclid(360.0) == 0.0
}

fn preserves_hue_alias(current: f64, next: f64) -> bool {
    (current == 360.0 && is_zero_hue_alias(next)) || (next == 360.0 && is_zero_hue_alias(current))
}

/// The props for the [`ColorPickerRoot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerRootProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Hsv<encoding::Srgb, f64>>,

    /// Whether the color picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker element
    pub children: Element,
}

/// Context-only root for [`ColorPicker`]. Provides [`ColorPickerContext`] to descendants
/// without rendering a [`ColorPickerSurface`] — use this when the surface is placed
/// elsewhere (e.g. inside a popover).
#[component]
pub fn ColorPickerRoot(props: ColorPickerRootProps) -> Element {
    rsx! {
        color_picker::ColorPicker {
            class: Styles::dx_color_picker,
            color: props.color,
            on_color_change: props.on_color_change,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorPicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Hsv<encoding::Srgb, f64>>,

    /// Whether the color picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Additional content to append to the default color picker surface.
    pub children: Element,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    rsx! {
        color_picker::ColorPicker {
            class: Styles::dx_color_picker,
            color: props.color,
            on_color_change: props.on_color_change,
            disabled: props.disabled,
            attributes: props.attributes,
            ColorPickerSurface { {props.children} }
        }
    }
}

/// The props for the [`ColorPickerSurface`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerSurfaceProps {
    /// Additional attributes to extend the picker surface.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker surface.
    pub children: Element,
}

#[component]
pub fn ColorPickerSurface(props: ColorPickerSurfaceProps) -> Element {
    rsx! {
        div {
            class: Styles::dx_color_picker_surface,
            ..props.attributes,
            ColorPickerSelect {}
            {props.children}
        }
    }
}

/// The props for the [`ColorSwatch`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorSwatchProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Additional attributes to extend the color swatch element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color swatch element
    pub children: Element,
}

/// # ColorSwatch
///
/// The [`ColorSwatch`] displays a preview of a selected color.
#[component]
pub fn ColorSwatch(props: ColorSwatchProps) -> Element {
    let hex_color = use_memo(move || {
        let rgb: Color = Srgb::<f64>::from_color((props.color)()).into_format();
        format_color_hex(rgb)
    });

    rsx! {
        div {
            role: "img",
            aria_label: format!("Selected color {hex_color}"),
            class: Styles::dx_color_swatch.to_string(),
            style: "--swatch-color: {hex_color}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorSlider`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorSliderProps {
    pub title: ReadSignal<String>,

    /// Additional attributes to extend the color slider element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color slider element
    pub children: Element,
}

/// # ColorSlider
///
/// The [`ColorSlider`] allows users to adjust the hue of the color held by
/// the surrounding [`ColorPickerContext`].
#[component]
fn ColorSlider(props: ColorSliderProps) -> Element {
    let ctx = use_context::<ColorPickerContext>();
    let mut current_hue = use_signal(|| ctx.color().hue.into_positive_degrees());
    let mut pending_color_hex = use_signal(|| None::<String>);

    let thumb_color = use_memo(move || {
        Srgb::<f64>::from_color(Hsv::<encoding::Srgb, f64>::new(
            RgbHue::new(current_hue()),
            1.0,
            1.0,
        ))
        .into_format()
    });

    use_effect(move || {
        let color = ctx.color();
        let parent_hex = color_hex_from_hsv(color);

        if pending_color_hex.peek().as_deref() == Some(parent_hex.as_str()) {
            pending_color_hex.set(None);
            return;
        }

        let value = color.hue.into_positive_degrees();
        let current = *current_hue.peek();

        // Update the signal only if this is an actual new position,
        // and not a "flip" of the circle by the palette library.
        if !preserves_hue_alias(current, value) && value != current {
            current_hue.set(value);
        }
    });

    let display_value = {
        let value = current_hue();
        format!("{value:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
            + "°"
    };

    rsx! {

        div {
            class: Styles::dx_color_slider_container.to_string(),
            ..props.attributes,
            label { class: Styles::dx_color_slider_title.to_string(), {props.title} }
            output { class: Styles::dx_color_slider_output.to_string(), "{display_value}" }
            Slider {
                class: Styles::dx_color_slider.to_string(),
                label: "Color Slider",
                horizontal: true,
                max: 360.0,
                value: Some(current_hue()),
                on_value_change: move |h: f64| {
                    // Allow the value to be exactly 360.0
                    // The palette will understand that 360.0 == 0.0, but the signal will remain 360.0 for the UI.
                    let current = ctx.color();
                    let next = Hsv::<encoding::Srgb, f64>::new(
                        RgbHue::new(h),
                        current.saturation,
                        current.value,
                    );
                    pending_color_hex.set(Some(color_hex_from_hsv(next)));
                    current_hue.set(h);
                    ctx.set_hue(h);
                },
                SliderTrack { class: Styles::dx_color_slider_track.to_string(),
                    SliderThumb {
                        class: Styles::dx_color_slider_thumb.to_string(),
                        aria_label: "Hue",
                        aria_valuetext: format!("{:.0}°", current_hue()),
                        background_color: format_color_hex(thumb_color()),
                    }
                }
            }
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{color_hex_from_hsv, preserves_hue_alias};
    use palette::{encoding, Hsv, RgbHue};

    #[test]
    fn hsv_hex_round_trip_matches_normalized_parent_color() {
        let requested = Hsv::<encoding::Srgb, f64>::new(RgbHue::new(360.0), 1.0, 1.0);
        let normalized = Hsv::<encoding::Srgb, f64>::new(RgbHue::new(0.0), 1.0, 1.0);

        assert_eq!(
            color_hex_from_hsv(requested),
            color_hex_from_hsv(normalized)
        );
    }

    #[test]
    fn alias_detection_only_preserves_0_and_360_equivalence() {
        assert!(preserves_hue_alias(360.0, 0.0));
        assert!(preserves_hue_alias(0.0, 360.0));
        assert!(!preserves_hue_alias(4.0, 355.0));
        assert!(!preserves_hue_alias(120.0, 121.0));
    }
}

#[component]
fn ColorArea(props: ColorAreaProps) -> Element {
    rsx! {
        color_picker::ColorArea {
            class: Styles::dx_color_area_container.to_string(),
            step: props.step,
            attributes: props.attributes,
            color_picker::AreaTrack { class: Styles::dx_color_area_track.to_string(),
                color_picker::AreaThumb { class: Styles::dx_color_area_thumb.to_string(),
                    color_picker::AreaThumbSaturationInput { class: Styles::dx_color_area_input.to_string() }
                    color_picker::AreaThumbValueInput { class: Styles::dx_color_area_input.to_string() }
                }
            }
            {props.children}
        }
    }
}

/// The props for the [`ColorPickerSelect`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerSelectProps {
    /// Additional attributes to extend the color picker select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker select element
    pub children: Element,
}

#[component]
pub fn ColorPickerSelect(props: ColorPickerSelectProps) -> Element {
    rsx! {
        div {
            class: Styles::dx_color_picker_dialog.to_string(),
            ..props.attributes,
            ColorArea {}
            ColorSlider { title: "Hue" }
        }
    }
}
