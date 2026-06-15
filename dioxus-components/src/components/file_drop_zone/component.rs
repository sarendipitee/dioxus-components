//! Styled wrapper around the [`dioxus_primitives::file_drop_zone`] primitive.
//!
//! [`FileDropZone`] applies the reusable drop-zone styling and forwards every
//! prop (including drag, validation, and file-input behavior) to the primitive.
//! The state-aware display helpers ([`FileDropZoneIdle`],
//! [`FileDropZoneAcceptDisplay`], [`FileDropZoneRejectDisplay`]) are rendered as
//! descendants of the zone and shown/hidden purely via CSS using the
//! `data-accept` / `data-reject` attributes the primitive sets on its root.

use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::file_drop_zone::{self, FileDropZoneProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

// Re-export the primitive's public data model so consumers can build props
// without depending on `dioxus-primitives` directly.
pub use dioxus_primitives::file_drop_zone::{
    AcceptedFile, AcceptedFileType, FileDropZoneAccept, FileMeta, FileRejection, RejectionCode,
    RejectionError,
};

#[component_styles("./style.css")]
struct Styles;

/// The canonical styled file drop zone.
///
/// This wraps the [`FileDropZone`](dioxus_primitives::file_drop_zone::FileDropZone)
/// primitive without changing any of its behavior: it merges the reusable
/// `dx_file_drop_zone` class into the forwarded attributes and passes every
/// prop straight through. All drag, keyboard, click, and validation logic lives
/// in the primitive.
///
/// State is exposed by the primitive as `data-*` attributes on the root
/// element; the styling reacts to `data-drag-active`, `data-accept`,
/// `data-reject`, `data-disabled`, and `data-loading`.
#[component]
pub fn FileDropZone(props: FileDropZoneProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_file_drop_zone,
    });

    let mut props = props;
    props.attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        file_drop_zone::FileDropZone { ..props }
    }
}

/// Renders its children while the zone is idle (not accepting or rejecting a
/// drag). Visibility is controlled entirely by CSS via the parent zone's
/// `data-accept` / `data-reject` attributes; the children stay mounted.
///
/// Place inside a [`FileDropZone`].
#[component]
pub fn FileDropZoneIdle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_file_drop_zone_idle,
        "data-slot": "file-drop-zone-idle",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}

/// Renders its children only while the nearest ancestor [`FileDropZone`] has its
/// `data-accept` state active (a drag that appears acceptable). Visibility is
/// CSS-only.
#[component]
pub fn FileDropZoneAcceptDisplay(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_file_drop_zone_accept_display,
        "data-slot": "file-drop-zone-accept",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}

/// Renders its children only while the nearest ancestor [`FileDropZone`] has its
/// `data-reject` state active (a drag that appears unacceptable). Visibility is
/// CSS-only.
#[component]
pub fn FileDropZoneRejectDisplay(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_file_drop_zone_reject_display,
        "data-slot": "file-drop-zone-reject",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}
