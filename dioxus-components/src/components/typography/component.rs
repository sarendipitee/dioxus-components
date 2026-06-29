use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
struct Styles;

/// Shared typography size scale for text and headings.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TypographySize {
    /// Extra-small typography.
    Xs,
    /// Small typography.
    Sm,
    /// Default body typography.
    #[default]
    Md,
    /// Large typography.
    Lg,
    /// Extra-large typography.
    Xl,
}

impl TypographySize {
    /// Returns the data attribute value for this typography size.
    pub fn as_str(&self) -> &'static str {
        match self {
            TypographySize::Xs => "xs",
            TypographySize::Sm => "sm",
            TypographySize::Md => "md",
            TypographySize::Lg => "lg",
            TypographySize::Xl => "xl",
        }
    }
}

/// Foreground tone for reusable typography.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TypographyTone {
    /// Inherit the current foreground color.
    #[default]
    Default,
    /// Use muted foreground color.
    Muted,
    /// Use faint foreground color.
    Faint,
    /// Use muted surface foreground color.
    SurfaceMuted,
    /// Use accent foreground color.
    Accent,
    /// Use destructive foreground color.
    Destructive,
}

impl TypographyTone {
    /// Returns the data attribute value for this typography tone.
    pub fn as_str(&self) -> &'static str {
        match self {
            TypographyTone::Default => "default",
            TypographyTone::Muted => "muted",
            TypographyTone::Faint => "faint",
            TypographyTone::SurfaceMuted => "surface-muted",
            TypographyTone::Accent => "accent",
            TypographyTone::Destructive => "destructive",
        }
    }
}

/// Font weight for reusable typography.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TypographyWeight {
    /// Inherit the surrounding font weight.
    #[default]
    Inherit,
    /// Use a lighter font weight.
    Lighter,
    /// Use normal font weight.
    Normal,
    /// Use medium font weight.
    Medium,
    /// Use semibold font weight.
    Semibold,
    /// Use bold font weight.
    Bold,
}

impl TypographyWeight {
    /// Returns the data attribute value for this typography weight.
    pub fn as_str(&self) -> &'static str {
        match self {
            TypographyWeight::Inherit => "inherit",
            TypographyWeight::Lighter => "lighter",
            TypographyWeight::Normal => "normal",
            TypographyWeight::Medium => "medium",
            TypographyWeight::Semibold => "semibold",
            TypographyWeight::Bold => "bold",
        }
    }
}

/// Text alignment for reusable typography.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TextAlign {
    /// Inherit the surrounding text alignment.
    #[default]
    Inherit,
    /// Align text to the inline start edge.
    Start,
    /// Center text.
    Center,
    /// Align text to the inline end edge.
    End,
}

impl TextAlign {
    /// Returns the data attribute value for this text alignment.
    pub fn as_str(&self) -> &'static str {
        match self {
            TextAlign::Inherit => "inherit",
            TextAlign::Start => "start",
            TextAlign::Center => "center",
            TextAlign::End => "end",
        }
    }
}

/// Text wrapping behavior for reusable typography.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TextWrap {
    /// Use normal wrapping.
    #[default]
    Wrap,
    /// Keep text on one line.
    NoWrap,
    /// Balance line breaks when supported.
    Balance,
    /// Prefer visually clean line breaks when supported.
    Pretty,
}

impl TextWrap {
    /// Returns the data attribute value for this text wrapping mode.
    pub fn as_str(&self) -> &'static str {
        match self {
            TextWrap::Wrap => "wrap",
            TextWrap::NoWrap => "nowrap",
            TextWrap::Balance => "balance",
            TextWrap::Pretty => "pretty",
        }
    }
}

/// Semantic element rendered by the `Text` component.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TextElement {
    /// Render a paragraph.
    #[default]
    P,
    /// Render an inline span.
    Span,
    /// Render a generic block.
    Div,
    /// Render a label.
    Label,
}

impl TextElement {
    /// Returns the data attribute value for this text element.
    pub fn as_str(&self) -> &'static str {
        match self {
            TextElement::P => "p",
            TextElement::Span => "span",
            TextElement::Div => "div",
            TextElement::Label => "label",
        }
    }
}

/// Semantic heading level rendered by the `Heading` component.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum HeadingLevel {
    /// Render an `h1`.
    H1,
    /// Render an `h2`.
    #[default]
    H2,
    /// Render an `h3`.
    H3,
    /// Render an `h4`.
    H4,
    /// Render an `h5`.
    H5,
    /// Render an `h6`.
    H6,
}

impl HeadingLevel {
    /// Returns the data attribute value for this heading level.
    pub fn as_str(&self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
        }
    }
}

/// Props for `Text`.
#[derive(Props, Clone, PartialEq)]
pub struct TextProps {
    /// Visual size of the text.
    #[props(default)]
    pub size: TypographySize,
    /// Foreground tone of the text.
    #[props(default)]
    pub tone: TypographyTone,
    /// Font weight of the text.
    #[props(default)]
    pub weight: TypographyWeight,
    /// Text alignment.
    #[props(default)]
    pub align: TextAlign,
    /// Text wrapping mode.
    #[props(default)]
    pub wrap: TextWrap,
    /// Whether to truncate text to one line.
    #[props(default)]
    pub truncate: bool,
    /// Optional number of lines to clamp text to.
    #[props(default)]
    pub line_clamp: Option<u8>,
    /// Semantic element to render.
    #[props(default)]
    pub element: TextElement,
    /// Global DOM attributes applied to the rendered element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Text content.
    pub children: Element,
}

/// Renders themed text with reusable typography styling.
#[component]
pub fn Text(props: TextProps) -> Element {
    let attributes = typography_attributes(
        Styles::dx_text.to_string(),
        props.size,
        props.tone,
        props.weight,
        props.align,
        props.wrap,
        props.truncate,
        props.line_clamp,
        props.attributes,
    );
    let children = props.children;

    match props.element {
        TextElement::P => rsx! { p { ..attributes, {children} } },
        TextElement::Span => rsx! { span { ..attributes, {children} } },
        TextElement::Div => rsx! { div { ..attributes, {children} } },
        TextElement::Label => rsx! { label { ..attributes, {children} } },
    }
}

/// Props for `Heading`.
#[derive(Props, Clone, PartialEq)]
pub struct HeadingProps {
    /// Visual size of the heading.
    #[props(default)]
    pub size: TypographySize,
    /// Foreground tone of the heading.
    #[props(default)]
    pub tone: TypographyTone,
    /// Font weight of the heading.
    #[props(default)]
    pub weight: TypographyWeight,
    /// Text alignment.
    #[props(default)]
    pub align: TextAlign,
    /// Text wrapping mode.
    #[props(default)]
    pub wrap: TextWrap,
    /// Whether to truncate text to one line.
    #[props(default)]
    pub truncate: bool,
    /// Optional number of lines to clamp text to.
    #[props(default)]
    pub line_clamp: Option<u8>,
    /// Semantic heading level to render.
    #[props(default)]
    pub level: HeadingLevel,
    /// Global DOM attributes applied to the rendered heading element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Heading content.
    pub children: Element,
}

/// Renders a themed semantic heading with reusable typography styling.
#[component]
pub fn Heading(props: HeadingProps) -> Element {
    let attributes = typography_attributes(
        Styles::dx_heading.to_string(),
        props.size,
        props.tone,
        props.weight,
        props.align,
        props.wrap,
        props.truncate,
        props.line_clamp,
        props.attributes,
    );
    let children = props.children;

    match props.level {
        HeadingLevel::H1 => rsx! { h1 { ..attributes, {children} } },
        HeadingLevel::H2 => rsx! { h2 { ..attributes, {children} } },
        HeadingLevel::H3 => rsx! { h3 { ..attributes, {children} } },
        HeadingLevel::H4 => rsx! { h4 { ..attributes, {children} } },
        HeadingLevel::H5 => rsx! { h5 { ..attributes, {children} } },
        HeadingLevel::H6 => rsx! { h6 { ..attributes, {children} } },
    }
}

fn typography_attributes(
    class: String,
    size: TypographySize,
    tone: TypographyTone,
    weight: TypographyWeight,
    align: TextAlign,
    wrap: TextWrap,
    truncate: bool,
    line_clamp: Option<u8>,
    attributes: Vec<Attribute>,
) -> Vec<Attribute> {
    let base = attributes!(div {
        class,
        "data-size": size.as_str(),
        "data-tone": tone.as_str(),
        "data-weight": weight.as_str(),
        "data-align": align.as_str(),
        "data-wrap": wrap.as_str(),
        "data-truncate": truncate.to_string(),
        "data-line-clamp": line_clamp.map(|value| value.to_string()),
        style: line_clamp.map(|value| format!("--line-clamp: {value};")),
    });

    merge_attributes(vec![base, attributes])
}
