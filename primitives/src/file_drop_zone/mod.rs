//! Defines the [`FileDropZone`] primitive: an unstyled, accessible file
//! selection and drag-and-drop behavior component.
//!
//! `FileDropZone` owns the hidden `<input type="file">`, drag state, native
//! file picker activation, validation, and structured rejection reporting. It
//! is not an uploader: consumers receive accepted and rejected files and decide
//! what to do next.
//!
//! # Browser caveats
//!
//! - The browser `accept` attribute is advisory, not a security boundary. Files
//!   are always revalidated after selection/drop.
//! - During drag, browsers typically expose only the MIME type (not the file
//!   name), so drag accept/reject is provisional. The post-drop result is
//!   authoritative.
//! - File dialog cancel detection is best-effort and must not drive critical
//!   state.
//! - Browser file APIs do not expose reliable local filesystem paths.

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Accept rules
// ---------------------------------------------------------------------------

/// A single accepted file type, expressed as an optional MIME type and/or a set
/// of file extensions.
///
/// A file matches this type if its MIME type matches [`AcceptedFileType::mime`]
/// (supporting `type/*` wildcards) **or** its extension is contained in
/// [`AcceptedFileType::extensions`]. Either match is sufficient, which avoids
/// rejecting valid browser files when one metadata field is missing or
/// inconsistent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AcceptedFileType {
    /// The MIME type to match, e.g. `"image/png"` or the wildcard `"image/*"`.
    /// Comparison is ASCII case-insensitive. `None` means no MIME constraint.
    pub mime: Option<String>,
    /// File extensions to match, normalized to lowercase without a leading dot
    /// (e.g. `["png", "jpg"]`). Matching is ASCII case-insensitive.
    pub extensions: Vec<String>,
}

impl AcceptedFileType {
    /// Create an [`AcceptedFileType`] from a MIME type only.
    pub fn mime(mime: impl Into<String>) -> Self {
        Self {
            mime: Some(mime.into()),
            extensions: Vec::new(),
        }
    }

    /// Create an [`AcceptedFileType`] from a list of extensions. Each extension
    /// is normalized (leading dot stripped, lowercased).
    pub fn extensions<I, S>(extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            mime: None,
            extensions: extensions
                .into_iter()
                .map(|e| normalize_extension(e.as_ref()))
                .collect(),
        }
    }

    /// Returns whether the given MIME type matches this type's MIME rule.
    fn mime_matches(&self, mime_type: &str) -> bool {
        match &self.mime {
            Some(pattern) => mime_pattern_matches(pattern, mime_type),
            None => false,
        }
    }

    /// Returns whether the given file extension matches this type's extension
    /// list. `ext` is expected to already be normalized (lowercase, no dot).
    fn extension_matches(&self, ext: &str) -> bool {
        self.extensions
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(ext))
    }
}

/// The accept rules for a [`FileDropZone`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FileDropZoneAccept {
    /// Accept any file.
    #[default]
    Any,
    /// Accept only files matching one of the provided types.
    Types(Vec<AcceptedFileType>),
}

impl FileDropZoneAccept {
    /// Build the HTML `accept` attribute string: a comma-separated list of MIME
    /// types and dot-prefixed extensions. Returns an empty string for
    /// [`FileDropZoneAccept::Any`].
    pub fn to_accept_string(&self) -> String {
        match self {
            FileDropZoneAccept::Any => String::new(),
            FileDropZoneAccept::Types(types) => {
                let mut parts: Vec<String> = Vec::new();
                for ty in types {
                    if let Some(mime) = &ty.mime {
                        let mime = mime.trim();
                        if !mime.is_empty() {
                            parts.push(mime.to_string());
                        }
                    }
                    for ext in &ty.extensions {
                        let ext = normalize_extension(ext);
                        if !ext.is_empty() {
                            parts.push(format!(".{ext}"));
                        }
                    }
                }
                parts.join(",")
            }
        }
    }

    /// Test whether a file (by name and MIME type) passes the accept rules.
    ///
    /// A file is accepted if [`FileDropZoneAccept::Any`], or if any configured
    /// [`AcceptedFileType`] matches the file's MIME type or its extension. The
    /// extension is derived from `file_name`. An empty `file_name` (as during a
    /// drag, when the browser only exposes the MIME type) simply contributes no
    /// extension to match against.
    pub fn accepts(&self, file_name: &str, mime_type: &str) -> bool {
        match self {
            FileDropZoneAccept::Any => true,
            FileDropZoneAccept::Types(types) => {
                let ext = file_extension(file_name);
                types.iter().any(|ty| {
                    ty.mime_matches(mime_type)
                        || ext
                            .as_deref()
                            .map(|e| ty.extension_matches(e))
                            .unwrap_or(false)
                })
            }
        }
    }
}

/// Normalize a file extension: strip a single leading dot and lowercase it.
fn normalize_extension(ext: &str) -> String {
    ext.trim().trim_start_matches('.').to_ascii_lowercase()
}

/// Extract the lowercase extension (without dot) from a file name, if any.
fn file_extension(file_name: &str) -> Option<String> {
    let name = file_name.trim();
    let dot = name.rfind('.')?;
    // A leading dot (dotfile with no extension) or trailing dot yields nothing.
    if dot == 0 || dot + 1 >= name.len() {
        return None;
    }
    Some(name[dot + 1..].to_ascii_lowercase())
}

/// Test whether a MIME `pattern` (possibly a `type/*` wildcard) matches a
/// concrete `mime_type`. Comparison is ASCII case-insensitive.
fn mime_pattern_matches(pattern: &str, mime_type: &str) -> bool {
    let pattern = pattern.trim();
    let mime_type = mime_type.trim();
    if pattern.is_empty() {
        return false;
    }
    // Full wildcard.
    if pattern == "*" || pattern == "*/*" {
        return !mime_type.is_empty();
    }
    if let Some(prefix) = pattern.strip_suffix("/*") {
        // e.g. "image/*" matches "image/anything".
        return mime_type
            .split_once('/')
            .map(|(ty, _)| ty.eq_ignore_ascii_case(prefix))
            .unwrap_or(false);
    }
    pattern.eq_ignore_ascii_case(mime_type)
}

// ---------------------------------------------------------------------------
// File data model
// ---------------------------------------------------------------------------

/// An accepted file together with its browser metadata.
///
/// `last_modified` is milliseconds since the Unix epoch, or `0.0` when the
/// browser does not expose it. There is intentionally no local filesystem path:
/// browser file APIs hide it for privacy.
#[derive(Clone)]
pub struct AcceptedFile {
    /// The underlying `web-sys` file handle.
    ///
    /// This field exists on all targets so consumer code (such as `validator`
    /// and `on_accepted`) compiles identically for native/server and wasm/client
    /// builds under Dioxus fullstack. It is only ever populated on the
    /// web/wasm target; on other targets no [`AcceptedFile`] is constructed.
    pub file: web_sys::File,
    /// The file name.
    pub name: String,
    /// The file size in bytes.
    pub size: u64,
    /// The MIME type as reported by the browser, or an empty string.
    pub mime_type: String,
    /// Last modified time in milliseconds since the Unix epoch, `0.0` if
    /// unavailable.
    pub last_modified: f64,
}

impl PartialEq for AcceptedFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.size == other.size
            && self.mime_type == other.mime_type
            && self.last_modified == other.last_modified
    }
}

impl std::fmt::Debug for AcceptedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AcceptedFile")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("mime_type", &self.mime_type)
            .field("last_modified", &self.last_modified)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Rejection model
// ---------------------------------------------------------------------------

/// A stable, machine-readable rejection code.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectionCode {
    /// The file's type did not match the accept rules (`file-invalid-type`).
    FileInvalidType,
    /// The file is larger than `max_size` (`file-too-large`).
    FileTooLarge,
    /// The file is smaller than `min_size` (`file-too-small`).
    FileTooSmall,
    /// Accepting the file would exceed `max_files` (`too-many-files`).
    TooManyFiles,
    /// A consumer-supplied validator rejected the file
    /// (`custom-validation-failed`).
    CustomValidationFailed,
}

impl RejectionCode {
    /// The stable kebab-case string code, matching react-dropzone conventions.
    pub fn as_str(&self) -> &'static str {
        match self {
            RejectionCode::FileInvalidType => "file-invalid-type",
            RejectionCode::FileTooLarge => "file-too-large",
            RejectionCode::FileTooSmall => "file-too-small",
            RejectionCode::TooManyFiles => "too-many-files",
            RejectionCode::CustomValidationFailed => "custom-validation-failed",
        }
    }
}

/// A single rejection reason: a stable code plus a human-readable message.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectionError {
    /// The stable, machine-readable code.
    pub code: RejectionCode,
    /// A human-readable description.
    pub message: String,
}

impl RejectionError {
    /// Construct a [`RejectionError`].
    pub fn new(code: RejectionCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// A rejected file: its metadata plus every validation failure, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRejection {
    /// The file name.
    pub name: String,
    /// The file size in bytes.
    pub size: u64,
    /// The MIME type as reported by the browser, or an empty string.
    pub mime_type: String,
    /// All validation failures for this file, in deterministic order.
    pub errors: Vec<RejectionError>,
}

// ---------------------------------------------------------------------------
// Pure validation core
//
// These functions operate only on plain metadata so they are fully testable in
// a native (non-wasm) environment without constructing a `web_sys::File`.
// ---------------------------------------------------------------------------

/// The metadata required to run the built-in validation checks on one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMeta {
    /// The file name.
    pub name: String,
    /// The file size in bytes.
    pub size: u64,
    /// The MIME type, or an empty string when unavailable.
    pub mime_type: String,
}

/// The configured, non-custom validation rules.
#[derive(Debug, Clone, Default)]
struct ValidationConfig {
    accept: FileDropZoneAccept,
    min_size: Option<u64>,
    max_size: Option<u64>,
    /// The maximum number of files that may be accepted, if limited.
    max_files: Option<usize>,
}

/// Run the per-file built-in checks (type, then min size, then max size) and
/// collect every failure in deterministic order.
///
/// Note: the max-file-count check is *not* applied here, because it depends on
/// how many files have already been accepted; see [`validate_files`].
fn validate_file_builtin(meta: &FileMeta, config: &ValidationConfig) -> Vec<RejectionError> {
    let mut errors = Vec::new();

    // 1. File type.
    if !config.accept.accepts(&meta.name, &meta.mime_type) {
        errors.push(RejectionError::new(
            RejectionCode::FileInvalidType,
            format!(
                "File type \"{}\" is not accepted.",
                if meta.mime_type.is_empty() {
                    "unknown"
                } else {
                    meta.mime_type.as_str()
                }
            ),
        ));
    }

    // 2. Minimum size.
    if let Some(min) = config.min_size {
        if meta.size < min {
            errors.push(RejectionError::new(
                RejectionCode::FileTooSmall,
                format!("File is smaller than the minimum size of {min} bytes."),
            ));
        }
    }

    // 3. Maximum size.
    if let Some(max) = config.max_size {
        if meta.size > max {
            errors.push(RejectionError::new(
                RejectionCode::FileTooLarge,
                format!("File is larger than the maximum size of {max} bytes."),
            ));
        }
    }

    errors
}

/// The outcome of validating a batch of files.
struct ValidationOutcome<F> {
    accepted: Vec<F>,
    rejected: Vec<FileRejection>,
}

/// Validate a batch of files against `config` and an optional custom validator.
///
/// `items` pairs each file payload `F` with its [`FileMeta`]. The payload is
/// generic so the pure logic can be exercised in tests (with `()` or
/// `FileMeta`) and reused by the wasm layer (with [`AcceptedFile`]).
///
/// Validation order per file: file type, minimum size, maximum size, then max
/// file count (applied to any file that would exceed the limit), then the
/// custom validator runs on files that passed the built-in checks. All failures
/// on a file are collected in order.
fn validate_files<F>(
    items: Vec<(F, FileMeta)>,
    config: &ValidationConfig,
    mut custom: impl FnMut(&F) -> Vec<RejectionError>,
) -> ValidationOutcome<F> {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    let mut accepted_count = 0usize;

    for (payload, meta) in items {
        let mut errors = validate_file_builtin(&meta, config);

        // 4. Max file count: a file that passes per-file checks but would push
        // the accepted count past the limit is rejected as too-many-files.
        let exceeds_count = config
            .max_files
            .map(|max| errors.is_empty() && accepted_count >= max)
            .unwrap_or(false);
        if exceeds_count {
            errors.push(RejectionError::new(
                RejectionCode::TooManyFiles,
                format!(
                    "Too many files. A maximum of {} file(s) may be selected.",
                    config.max_files.unwrap_or(0)
                ),
            ));
        }

        // 5. Custom validator runs on files that passed the built-in checks.
        if errors.is_empty() {
            let custom_errors = custom(&payload);
            errors.extend(custom_errors);
        }

        if errors.is_empty() {
            accepted_count += 1;
            accepted.push(payload);
        } else {
            rejected.push(FileRejection {
                name: meta.name,
                size: meta.size,
                mime_type: meta.mime_type,
                errors,
            });
        }
    }

    ValidationOutcome { accepted, rejected }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// The props for the [`FileDropZone`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FileDropZoneProps {
    /// The allowed file types. Defaults to [`FileDropZoneAccept::Any`].
    #[props(default)]
    pub accept: FileDropZoneAccept,

    /// Whether more than one file may be selected. When `false`, the effective
    /// maximum file count is 1. Defaults to `false`.
    #[props(default)]
    pub multiple: bool,

    /// Minimum allowed file size in bytes.
    #[props(default)]
    pub min_size: Option<u64>,

    /// Maximum allowed file size in bytes.
    #[props(default)]
    pub max_size: Option<u64>,

    /// Maximum number of accepted files per drop/selection. When `multiple` is
    /// `false`, an effective limit of 1 is used regardless of this value.
    #[props(default)]
    pub max_files: Option<usize>,

    /// Optional consumer-supplied validator. It runs after the built-in checks
    /// pass and may return additional [`RejectionError`]s.
    #[props(default)]
    pub validator: Option<Callback<AcceptedFile, Vec<RejectionError>>>,

    /// Whether interaction is disabled. Prevents drag, drop, click, keyboard,
    /// and imperative picker activation. Defaults to `false`.
    #[props(default)]
    pub disabled: bool,

    /// Whether the zone is in a pending/loading state. Behaves like `disabled`
    /// for interaction. Defaults to `false`.
    #[props(default)]
    pub loading: bool,

    /// Whether clicking the root opens the file picker. Defaults to `true`.
    #[props(default = true)]
    pub activate_on_click: bool,

    /// Whether Enter/Space on the root opens the file picker. Defaults to
    /// `true`.
    #[props(default = true)]
    pub activate_on_keyboard: bool,

    /// Whether drag/drop events are handled. Defaults to `true`.
    #[props(default = true)]
    pub enable_drag: bool,

    /// Whether to attach document-level `dragover`/`drop` handlers that prevent
    /// the browser from navigating when files are dropped outside the zone.
    /// Defaults to `false`.
    #[props(default)]
    pub prevent_drop_on_document: bool,

    /// Whether to stop propagation of drag events handled by this zone, to
    /// avoid notifying parent drop zones. Defaults to `false`.
    #[props(default)]
    pub stop_drag_propagation: bool,

    /// Opt-in mode that preserves pointer events on children. The primitive
    /// only exposes this as a `data-interactive-children` attribute; the styled
    /// layer decides what to do with it. Defaults to `false`.
    #[props(default)]
    pub interactive_children: bool,

    /// A monotonic open-request token. When this changes to `Some(n)` with `n`
    /// greater than the previously observed value, the file picker is opened.
    /// No-op when disabled or loading.
    #[props(default)]
    pub open_request: ReadSignal<Option<u64>>,

    /// Fired for every completed selection/drop with both accepted files and
    /// rejections.
    #[props(default)]
    pub on_drop: Option<Callback<(Vec<AcceptedFile>, Vec<FileRejection>)>>,

    /// Fired with only the accepted files for a completed selection/drop.
    #[props(default)]
    pub on_accepted: Option<Callback<Vec<AcceptedFile>>>,

    /// Fired with only the rejected files for a completed selection/drop.
    #[props(default)]
    pub on_rejected: Option<Callback<Vec<FileRejection>>>,

    /// Fired when the component requests the native file picker.
    #[props(default)]
    pub on_file_dialog_open: Option<Callback<()>>,

    /// Best-effort callback when the native picker appears to close without
    /// selecting files. Cancel detection is unreliable across browsers.
    #[props(default)]
    pub on_file_dialog_cancel: Option<Callback<()>>,

    /// Reports unexpected browser or extraction errors.
    #[props(default)]
    pub on_error: Option<Callback<String>>,

    /// Additional attributes applied to the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the drop zone.
    pub children: Element,
}

/// # FileDropZone
///
/// An unstyled, accessible file selection and drag-and-drop primitive. It owns
/// a hidden `<input type="file">`, tracks drag state, runs validation, and
/// dispatches structured accepted/rejected callbacks. It does not perform
/// uploads.
///
/// The root element has `role="button"` and is keyboard-activatable (Enter /
/// Space) by default. State is exposed through data attributes for the styled
/// layer.
///
/// ## Styling
///
/// The root element carries these data attributes:
/// - `data-idle`: present when not dragging and the file dialog is not active.
/// - `data-drag-active`: present while one or more items are dragged over the
///   zone.
/// - `data-accept`: present when dragged items appear acceptable (provisional).
/// - `data-reject`: present when dragged items appear unacceptable
///   (provisional).
/// - `data-file-dialog-active`: present while the native picker is open.
/// - `data-disabled`: present when `disabled` or `loading`.
/// - `data-loading`: present when `loading`.
/// - `data-interactive-children`: present when `interactive_children` is set.
#[component]
pub fn FileDropZone(props: FileDropZoneProps) -> Element {
    let mut drag_active = use_signal(|| false);
    let mut drag_accept = use_signal(|| false);
    let mut drag_reject = use_signal(|| false);
    let mut file_dialog_active = use_signal(|| false);

    // Ref to the hidden file input so we can imperatively open the picker.
    let mut input_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let disabled = props.disabled;
    let loading = props.loading;
    let interaction_blocked = disabled || loading;

    let accept = props.accept.clone();
    let accept_string = accept.to_accept_string();

    // When not in multiple mode, the effective max file count is 1.
    let effective_max_files = if props.multiple {
        props.max_files
    } else {
        Some(1)
    };

    let config = ValidationConfig {
        accept: accept.clone(),
        min_size: props.min_size,
        max_size: props.max_size,
        max_files: effective_max_files,
    };

    // Shared validation + dispatch used by both drop and input change.
    let validator = props.validator;
    let on_drop_cb = props.on_drop;
    let on_accepted_cb = props.on_accepted;
    let on_rejected_cb = props.on_rejected;
    let run_validation = use_callback(move |files: Vec<AcceptedFile>| {
        let items: Vec<(AcceptedFile, FileMeta)> = files
            .into_iter()
            .map(|f| {
                let meta = FileMeta {
                    name: f.name.clone(),
                    size: f.size,
                    mime_type: f.mime_type.clone(),
                };
                (f, meta)
            })
            .collect();

        let outcome = validate_files(items, &config, |file| match &validator {
            Some(cb) => cb.call(file.clone()),
            None => Vec::new(),
        });

        let accepted = outcome.accepted;
        let rejected = outcome.rejected;

        if let Some(cb) = &on_accepted_cb {
            cb.call(accepted.clone());
        }
        if let Some(cb) = &on_rejected_cb {
            cb.call(rejected.clone());
        }
        if let Some(cb) = &on_drop_cb {
            cb.call((accepted, rejected));
        }
    });

    // Open the native picker through the hidden input.
    let on_file_dialog_open = props.on_file_dialog_open;
    let on_file_dialog_cancel = props.on_file_dialog_cancel;
    let open_picker = use_callback(move |_: ()| {
        if interaction_blocked {
            return;
        }
        if let Some(node) = input_ref() {
            file_dialog_active.set(true);
            if let Some(cb) = &on_file_dialog_open {
                cb.call(());
            }
            #[cfg(all(feature = "web", target_arch = "wasm32"))]
            {
                use dioxus::web::WebEventExt;
                use wasm_bindgen::JsCast;
                if let Some(input) = node
                    .try_as_web_event()
                    .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
            }
            // Best-effort cancel detection: when focus returns to the window
            // after the picker closes without a selection, clear the dialog
            // state and fire on_file_dialog_cancel.
            #[cfg(all(feature = "web", target_arch = "wasm32"))]
            {
                use wasm_bindgen::closure::Closure;
                use wasm_bindgen::JsCast;
                let mut dialog_active = file_dialog_active;
                let on_cancel = on_file_dialog_cancel.clone();
                let closure: Closure<dyn FnMut()> = Closure::once(move || {
                    if *dialog_active.peek() {
                        dialog_active.set(false);
                        if let Some(cb) = on_cancel {
                            cb.call(());
                        }
                    }
                });
                if let Some(win) = web_sys::window() {
                    let _ = win.add_event_listener_with_callback_and_bool(
                        "focus",
                        closure.as_ref().unchecked_ref(),
                        true, // capture phase
                    );
                }
                // One-shot listener; intentional leak.
                closure.forget();
            }
            #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
            {
                let _ = &node;
            }
        }
    });

    // Imperative open: watch the monotonic token and open on a strictly
    // increasing value.
    let open_request = props.open_request;
    let mut last_seen: Signal<Option<u64>> = use_signal(|| None::<u64>);
    use_effect(move || {
        if let Some(token) = open_request() {
            if last_seen.peek().map_or(true, |prev| token > prev) {
                last_seen.set(Some(token));
                if !interaction_blocked {
                    open_picker.call(());
                }
            }
        }
    });

    // Document-level drop prevention. Called unconditionally to respect the
    // rules of hooks; the effect guards on the flag internally.
    prevent_document_drop(props.prevent_drop_on_document);

    let on_error = props.on_error;
    let enable_drag = props.enable_drag;
    let stop_drag_propagation = props.stop_drag_propagation;
    let accept_for_drag = accept.clone();

    let tabindex = if interaction_blocked { "-1" } else { "0" };

    rsx! {
        div {
            role: "button",
            tabindex,
            aria_disabled: if interaction_blocked { "true" },

            "data-idle": (!drag_active() && !file_dialog_active()).then_some(true),
            "data-drag-active": drag_active().then_some(true),
            "data-accept": drag_accept().then_some(true),
            "data-reject": drag_reject().then_some(true),
            "data-file-dialog-active": file_dialog_active().then_some(true),
            "data-disabled": interaction_blocked.then_some(true),
            "data-loading": loading.then_some(true),
            "data-interactive-children": props.interactive_children.then_some(true),

            ondragenter: {
                let accept = accept_for_drag.clone();
                move |event: Event<DragData>| {
                    if !enable_drag || interaction_blocked {
                        return;
                    }
                    if stop_drag_propagation {
                        event.stop_propagation();
                    }
                    event.prevent_default();
                    drag_active.set(true);
                    update_drag_state(&event, &accept, &mut drag_accept, &mut drag_reject);
                }
            },
            ondragover: {
                let accept = accept_for_drag.clone();
                move |event: Event<DragData>| {
                    if !enable_drag || interaction_blocked {
                        return;
                    }
                    if stop_drag_propagation {
                        event.stop_propagation();
                    }
                    // prevent_default on dragover is required to allow a drop.
                    event.prevent_default();
                    drag_active.set(true);
                    update_drag_state(&event, &accept, &mut drag_accept, &mut drag_reject);
                }
            },
            ondragleave: move |event: Event<DragData>| {
                if !enable_drag || interaction_blocked {
                    return;
                }
                if stop_drag_propagation {
                    event.stop_propagation();
                }
                drag_active.set(false);
                drag_accept.set(false);
                drag_reject.set(false);
            },
            ondrop: move |event: Event<DragData>| {
                event.prevent_default();
                if !enable_drag || interaction_blocked {
                    return;
                }
                if stop_drag_propagation {
                    event.stop_propagation();
                }
                drag_active.set(false);
                drag_accept.set(false);
                drag_reject.set(false);

                match extract_dropped_files(&event) {
                    Ok(files) => run_validation.call(files),
                    Err(err) => {
                        if let Some(cb) = &on_error {
                            cb.call(err);
                        }
                    }
                }
            },
            onclick: move |_| {
                if !props.activate_on_click || interaction_blocked {
                    return;
                }
                open_picker.call(());
            },
            onkeydown: move |event: Event<KeyboardData>| {
                if !props.activate_on_keyboard || interaction_blocked {
                    return;
                }
                let is_activation = matches!(event.key(), Key::Enter)
                    || matches!(&event.key(), Key::Character(c) if c == " ");
                if is_activation {
                    event.prevent_default();
                    open_picker.call(());
                }
            },

            ..props.attributes,

            // Hidden file input. Kept in the DOM so browser file selection
            // stays accessible and reliable.
            input {
                r#type: "file",
                accept: accept_string,
                multiple: props.multiple,
                disabled: interaction_blocked,
                tabindex: "-1",
                aria_hidden: "true",
                style: "position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border:0;",
                onmounted: move |evt| input_ref.set(Some(evt.data())),
                onchange: move |event: Event<FormData>| {
                    file_dialog_active.set(false);
                    match extract_input_files(&event) {
                        Ok(files) => run_validation.call(files),
                        Err(err) => {
                            if let Some(cb) = &on_error {
                                cb.call(err);
                            }
                        }
                    }
                    // Reset the input value so that selecting the same file again
                    // produces a fresh change event.
                    #[cfg(all(feature = "web", target_arch = "wasm32"))]
                    {
                        use dioxus::web::WebEventExt;
                        use wasm_bindgen::JsCast;
                        if let Some(input) = event
                            .try_as_web_event()
                            .and_then(|e| e.target())
                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                        {
                            let _ = input.set_value("");
                        }
                    }
                },
            }

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// Drag-state evaluation (provisional)
// ---------------------------------------------------------------------------

/// Inspect a drag event's items and update the provisional accept/reject
/// signals. During drag the browser usually exposes only the MIME type, so this
/// is advisory; the post-drop result is authoritative. We never hard-reject on
/// missing metadata.
#[allow(unused_variables)]
fn update_drag_state(
    event: &Event<DragData>,
    accept: &FileDropZoneAccept,
    drag_accept: &mut Signal<bool>,
    drag_reject: &mut Signal<bool>,
) {
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    {
        use dioxus::web::WebEventExt;
        let Some(drag_event) = event.try_as_web_event() else {
            return;
        };
        let Some(data_transfer) = drag_event.data_transfer() else {
            // No metadata available; stay neutral.
            return;
        };
        let items = data_transfer.items();

        let mut saw_non_file = false;
        let mut any_clear_mismatch = false;
        let mut any_match = false;
        let mut saw_typed_file = false;

        for i in 0..items.length() {
            let Some(item) = items.get(i) else { continue };
            if item.kind() != "file" {
                saw_non_file = true;
                continue;
            }
            let mime = item.type_();
            if mime.is_empty() {
                // Browser hid the type; cannot judge yet.
                continue;
            }
            saw_typed_file = true;
            // Only MIME is available during drag (no file name).
            if accept.accepts("", &mime) {
                any_match = true;
            } else {
                any_clear_mismatch = true;
            }
        }

        if saw_non_file {
            drag_reject.set(true);
            drag_accept.set(false);
            return;
        }

        match accept {
            FileDropZoneAccept::Any => {
                drag_accept.set(true);
                drag_reject.set(false);
            }
            FileDropZoneAccept::Types(_) => {
                if any_match {
                    drag_accept.set(true);
                    drag_reject.set(false);
                } else if saw_typed_file && any_clear_mismatch {
                    drag_accept.set(false);
                    drag_reject.set(true);
                } else {
                    // Metadata not exposed; stay neutral pending drop.
                    drag_accept.set(false);
                    drag_reject.set(false);
                }
            }
        }
    }

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    {
        // On non-web targets, treat `Any` as accept and otherwise stay neutral.
        match accept {
            FileDropZoneAccept::Any => {
                drag_accept.set(true);
                drag_reject.set(false);
            }
            FileDropZoneAccept::Types(_) => {
                drag_accept.set(false);
                drag_reject.set(false);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// File extraction (web-only; non-web returns empty/no-op)
// ---------------------------------------------------------------------------

/// Extract dropped files from a drag event's `DataTransfer`.
#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn extract_dropped_files(event: &Event<DragData>) -> Result<Vec<AcceptedFile>, String> {
    use dioxus::web::WebEventExt;
    let drag_event = event
        .try_as_web_event()
        .ok_or_else(|| "Failed to access the native drag event.".to_string())?;
    let data_transfer = drag_event
        .data_transfer()
        .ok_or_else(|| "Drop event did not carry a data transfer.".to_string())?;
    match data_transfer.files() {
        Some(list) => Ok(accepted_files_from_list(&list)),
        None => Ok(Vec::new()),
    }
}

/// Non-web stub: there are no real files to extract.
#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
fn extract_dropped_files(_event: &Event<DragData>) -> Result<Vec<AcceptedFile>, String> {
    Ok(Vec::new())
}

/// Extract selected files from the hidden input's change event.
#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn extract_input_files(event: &Event<FormData>) -> Result<Vec<AcceptedFile>, String> {
    use dioxus::web::WebEventExt;
    use wasm_bindgen::JsCast;
    let web_event = event
        .try_as_web_event()
        .ok_or_else(|| "Failed to access the native change event.".to_string())?;
    let input = web_event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .ok_or_else(|| "Change event target was not a file input.".to_string())?;
    match input.files() {
        Some(list) => Ok(accepted_files_from_list(&list)),
        None => Ok(Vec::new()),
    }
}

/// Non-web stub: there are no real files to extract.
#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
fn extract_input_files(_event: &Event<FormData>) -> Result<Vec<AcceptedFile>, String> {
    Ok(Vec::new())
}

/// Build [`AcceptedFile`]s from a `web_sys::FileList`.
#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn accepted_files_from_list(list: &web_sys::FileList) -> Vec<AcceptedFile> {
    let mut files = Vec::with_capacity(list.length() as usize);
    for i in 0..list.length() {
        let Some(file) = list.get(i) else { continue };
        let name = file.name();
        let size = file.size() as u64;
        let mime_type = file.type_();
        let last_modified = file.last_modified();
        files.push(AcceptedFile {
            file,
            name,
            size,
            mime_type,
            last_modified,
        });
    }
    files
}

// ---------------------------------------------------------------------------
// Document-level drop prevention
// ---------------------------------------------------------------------------

/// Attach `dragover`/`drop` listeners on `window` that call `preventDefault`,
/// stopping the browser from navigating when files are dropped outside the
/// zone. The listeners are removed on cleanup.
///
/// This must be called unconditionally (rules of hooks): pass `enabled = false`
/// to make it a no-op rather than guarding the call site. Because `enabled` is
/// captured by value, the listeners are attached once based on its initial
/// value and are not re-evaluated if the prop later changes.
#[cfg_attr(
    not(all(feature = "web", target_arch = "wasm32")),
    allow(unused_variables)
)]
fn prevent_document_drop(enabled: bool) {
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        crate::use_effect_with_cleanup(move || {
            // Always register the effect so the hook count is stable, but only
            // attach listeners when enabled.
            let window = enabled.then(web_sys::window).flatten();
            let prevent = Closure::<dyn FnMut(web_sys::Event)>::new(|e: web_sys::Event| {
                e.prevent_default();
            });

            if let Some(window) = &window {
                let target: &web_sys::EventTarget = window.as_ref();
                let _ = target
                    .add_event_listener_with_callback("dragover", prevent.as_ref().unchecked_ref());
                let _ = target
                    .add_event_listener_with_callback("drop", prevent.as_ref().unchecked_ref());
            }

            move || {
                if let Some(window) = &window {
                    let target: &web_sys::EventTarget = window.as_ref();
                    let _ = target.remove_event_listener_with_callback(
                        "dragover",
                        prevent.as_ref().unchecked_ref(),
                    );
                    let _ = target.remove_event_listener_with_callback(
                        "drop",
                        prevent.as_ref().unchecked_ref(),
                    );
                }
                // Keep the closure alive until cleanup runs, then drop it.
                drop(prevent);
            }
        });
    }
}

// ---------------------------------------------------------------------------
// Tests (pure validation logic only; no DOM / no web-sys construction)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn ty(mime: Option<&str>, exts: &[&str]) -> AcceptedFileType {
        AcceptedFileType {
            mime: mime.map(|m| m.to_string()),
            extensions: exts.iter().map(|e| normalize_extension(e)).collect(),
        }
    }

    fn meta(name: &str, size: u64, mime: &str) -> FileMeta {
        FileMeta {
            name: name.to_string(),
            size,
            mime_type: mime.to_string(),
        }
    }

    // --- accept rules ------------------------------------------------------

    #[test]
    fn any_accepts_all_files() {
        let accept = FileDropZoneAccept::Any;
        assert!(accept.accepts("foo.png", "image/png"));
        assert!(accept.accepts("foo.exe", "application/octet-stream"));
        assert!(accept.accepts("", ""));
    }

    #[test]
    fn mime_exact_match_accepted() {
        let accept = FileDropZoneAccept::Types(vec![ty(Some("image/png"), &[])]);
        assert!(accept.accepts("a.png", "image/png"));
        assert!(accept.accepts("a.png", "IMAGE/PNG")); // case-insensitive
        assert!(!accept.accepts("a.gif", "image/gif"));
    }

    #[test]
    fn mime_wildcard_match() {
        let accept = FileDropZoneAccept::Types(vec![ty(Some("image/*"), &[])]);
        assert!(accept.accepts("a.png", "image/png"));
        assert!(accept.accepts("a.jpg", "image/jpeg"));
        assert!(!accept.accepts("a.txt", "text/plain"));
    }

    #[test]
    fn extension_match_case_insensitive() {
        let accept = FileDropZoneAccept::Types(vec![ty(None, &["png", "jpg"])]);
        assert!(accept.accepts("photo.PNG", ""));
        assert!(accept.accepts("photo.jpg", ""));
        assert!(!accept.accepts("photo.gif", ""));
    }

    #[test]
    fn extension_normalized_with_leading_dot() {
        let accept = FileDropZoneAccept::Types(vec![ty(None, &[".PNG"])]);
        assert!(accept.accepts("a.png", ""));
    }

    #[test]
    fn mime_or_extension_either_matches() {
        // One AcceptedFileType with both a MIME and an extension: either match
        // is sufficient.
        let accept = FileDropZoneAccept::Types(vec![ty(Some("image/png"), &["jpg"])]);
        // Matches on MIME, extension does not match.
        assert!(accept.accepts("a.weird", "image/png"));
        // Matches on extension, MIME does not match.
        assert!(accept.accepts("a.jpg", "application/octet-stream"));
        // Neither matches.
        assert!(!accept.accepts("a.gif", "image/gif"));
    }

    #[test]
    fn reject_wrong_mime_and_wrong_extension() {
        let accept = FileDropZoneAccept::Types(vec![ty(Some("image/png"), &["png"])]);
        assert!(!accept.accepts("doc.pdf", "application/pdf"));
    }

    // --- to_accept_string --------------------------------------------------

    #[test]
    fn to_accept_string_any_is_empty() {
        assert_eq!(FileDropZoneAccept::Any.to_accept_string(), "");
    }

    #[test]
    fn to_accept_string_generates_mime_and_dotted_extensions() {
        let accept = FileDropZoneAccept::Types(vec![
            ty(Some("image/png"), &["png"]),
            ty(Some("image/*"), &[]),
            ty(None, &["JPG", ".jpeg"]),
        ]);
        assert_eq!(
            accept.to_accept_string(),
            "image/png,.png,image/*,.jpg,.jpeg"
        );
    }

    // --- size validation ---------------------------------------------------

    #[test]
    fn file_too_small() {
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Any,
            min_size: Some(100),
            max_size: None,
            max_files: None,
        };
        let errors = validate_file_builtin(&meta("a.bin", 50, ""), &config);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, RejectionCode::FileTooSmall);
    }

    #[test]
    fn file_too_large() {
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Any,
            min_size: None,
            max_size: Some(100),
            max_files: None,
        };
        let errors = validate_file_builtin(&meta("a.bin", 500, ""), &config);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, RejectionCode::FileTooLarge);
    }

    #[test]
    fn size_within_bounds_ok() {
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Any,
            min_size: Some(100),
            max_size: Some(1000),
            max_files: None,
        };
        let errors = validate_file_builtin(&meta("a.bin", 500, ""), &config);
        assert!(errors.is_empty());
    }

    // --- multiple errors collected in order --------------------------------

    #[test]
    fn multiple_errors_collected_in_order() {
        // Wrong type AND too small: both errors, type first.
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Types(vec![ty(Some("image/png"), &[])]),
            min_size: Some(1000),
            max_size: None,
            max_files: None,
        };
        let errors = validate_file_builtin(&meta("a.txt", 10, "text/plain"), &config);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].code, RejectionCode::FileInvalidType);
        assert_eq!(errors[1].code, RejectionCode::FileTooSmall);
    }

    #[test]
    fn type_then_min_then_max_order() {
        // Construct a file that is simultaneously too small and (impossibly)
        // too large would conflict, so check type + max ordering instead.
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Types(vec![ty(Some("image/png"), &[])]),
            min_size: None,
            max_size: Some(5),
            max_files: None,
        };
        let errors = validate_file_builtin(&meta("a.txt", 100, "text/plain"), &config);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].code, RejectionCode::FileInvalidType);
        assert_eq!(errors[1].code, RejectionCode::FileTooLarge);
    }

    // --- batch / count / custom validation ---------------------------------

    #[test]
    fn validate_files_splits_accepted_and_rejected() {
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Types(vec![ty(Some("image/*"), &[])]),
            min_size: None,
            max_size: None,
            max_files: None,
        };
        let items = vec![
            ((), meta("a.png", 10, "image/png")),
            ((), meta("b.txt", 10, "text/plain")),
        ];
        let outcome = validate_files(items, &config, |_| Vec::new());
        assert_eq!(outcome.accepted.len(), 1);
        assert_eq!(outcome.rejected.len(), 1);
        assert_eq!(outcome.rejected[0].name, "b.txt");
        assert_eq!(
            outcome.rejected[0].errors[0].code,
            RejectionCode::FileInvalidType
        );
    }

    #[test]
    fn too_many_files_rejects_overflow() {
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Any,
            min_size: None,
            max_size: None,
            max_files: Some(2),
        };
        let items = vec![
            ((), meta("a", 1, "")),
            ((), meta("b", 1, "")),
            ((), meta("c", 1, "")),
        ];
        let outcome = validate_files(items, &config, |_| Vec::new());
        assert_eq!(outcome.accepted.len(), 2);
        assert_eq!(outcome.rejected.len(), 1);
        assert_eq!(
            outcome.rejected[0].errors[0].code,
            RejectionCode::TooManyFiles
        );
    }

    #[test]
    fn custom_validator_rejects() {
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Any,
            min_size: None,
            max_size: None,
            max_files: None,
        };
        let items = vec![("reject-me", meta("a", 1, "")), ("ok", meta("b", 1, ""))];
        let outcome = validate_files(items, &config, |payload| {
            if *payload == "reject-me" {
                vec![RejectionError::new(
                    RejectionCode::CustomValidationFailed,
                    "nope",
                )]
            } else {
                Vec::new()
            }
        });
        assert_eq!(outcome.accepted.len(), 1);
        assert_eq!(outcome.rejected.len(), 1);
        assert_eq!(
            outcome.rejected[0].errors[0].code,
            RejectionCode::CustomValidationFailed
        );
    }

    #[test]
    fn custom_validator_skipped_when_builtin_fails() {
        // The custom validator only runs on files that passed built-in checks.
        let config = ValidationConfig {
            accept: FileDropZoneAccept::Types(vec![ty(Some("image/png"), &[])]),
            min_size: None,
            max_size: None,
            max_files: None,
        };
        let mut custom_ran = false;
        let items = vec![((), meta("a.txt", 1, "text/plain"))];
        let outcome = validate_files(items, &config, |_| {
            custom_ran = true;
            Vec::new()
        });
        assert!(!custom_ran);
        assert_eq!(outcome.rejected.len(), 1);
        assert_eq!(outcome.rejected[0].errors.len(), 1);
    }

    #[test]
    fn rejection_code_strings_are_stable() {
        assert_eq!(RejectionCode::FileInvalidType.as_str(), "file-invalid-type");
        assert_eq!(RejectionCode::FileTooLarge.as_str(), "file-too-large");
        assert_eq!(RejectionCode::FileTooSmall.as_str(), "file-too-small");
        assert_eq!(RejectionCode::TooManyFiles.as_str(), "too-many-files");
        assert_eq!(
            RejectionCode::CustomValidationFailed.as_str(),
            "custom-validation-failed"
        );
    }
}
