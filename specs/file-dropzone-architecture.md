# File DropZone Architecture

## Purpose

File DropZone provides reusable file selection and drag-and-drop behavior for Dioxus applications. It owns the browser file input, drag state, validation, rejection reporting, and accessibility contract for choosing local files.

File DropZone is not an uploader. It must not own network upload, server persistence, image processing, preview generation, or application-specific file storage. Consumers receive selected files and decide what to do next.

The design is informed by Mantine Dropzone and react-dropzone:

- Mantine Dropzone: https://mantine.dev/x/dropzone/
- react-dropzone docs: https://react-dropzone.js.org/
- react-dropzone README: https://github.com/react-dropzone/react-dropzone/blob/master/README.md
- react-dropzone types: https://github.com/react-dropzone/react-dropzone/blob/master/typings/react-dropzone.d.ts

## Acceptance Summary

- Supports file selection through both drag-and-drop and a native file picker.
- Keeps a real file input in the DOM so browser file selection remains accessible and reliable.
- Exposes accepted files and rejected files with structured rejection reasons.
- Supports MIME type and extension accept rules, minimum and maximum file size, maximum file count, and single-file or multi-file mode.
- Defines `accept` as a typed Rust structure that can also render the hidden input's `accept` string.
- Tracks and exposes idle, drag-active, drag-accept, drag-reject, disabled, loading, and file-dialog-active states.
- Allows consumers to disable click activation, keyboard activation, drag handling, and document-level drop prevention when needed.
- Supports a signal-driven imperative open action for external buttons or custom controls.
- Uses data attributes for styling state in the styled wrapper.
- Places base behavior in `primitives/`, canonical styled wrappers in `dioxus-components/`, and demos/docs in `preview/`.
- Includes primitive tests for validation and interaction state, styled crate checks, and preview/Playwright coverage for user workflows.

## Layering

### primitives/

`primitives/` owns the unstyled behavior and browser interaction model:

- hidden file input wiring
- drag enter, drag over, drag leave, and drop state
- file picker open state
- validation and rejection reason generation
- callback dispatch
- keyboard and pointer activation behavior
- accessibility attributes and focus behavior
- document-level drop prevention when enabled

The primitive must not depend on shadcn styling, preview demos, CSS modules, or `dioxus-components`.

### dioxus-components/

`dioxus-components/` owns the canonical styled reusable component:

- component CSS module classes
- visual variants for idle, accept, reject, disabled, and loading states
- default content slots or child composition helpers
- state data attributes for styling
- public styled wrapper API that forwards global attributes to the primitive

The styled wrapper should import the primitive rather than reimplementing drag, validation, or file input behavior.

### preview/

`preview/` owns demos, docs, metadata, and visual validation:

- common examples such as images-only, max file size, max file count, disabled/loading, custom open button, and rejected-file feedback
- preview routes and component metadata
- Playwright coverage for click, keyboard, drag/drop, and validation flows

Preview must not define canonical behavior or reusable library styles.

## Component Model

The component should be designed around a root drop zone element and a hidden input:

- Root element receives focus, drag events, click activation, keyboard activation, and state data attributes.
- Hidden input uses `type="file"` and mirrors accept, multiple, and disabled attributes.
- Consumers can provide children for normal content, or use state-aware child helpers in the styled layer.

Recommended component names:

- Primitive: `FileDropZone`
- Styled wrapper: `FileDropZone`
- Optional state-aware display helpers: `FileDropZoneIdle`, `FileDropZoneAccept`, `FileDropZoneReject`

The exact module naming should follow existing component naming conventions when implementation begins.

## Public API Requirements

### Selection And Validation Props

The API should support:

- `accept`: allowed MIME types and/or extensions.
- `multiple`: whether more than one file may be selected.
- `min_size`: minimum allowed file size in bytes.
- `max_size`: maximum allowed file size in bytes.
- `max_files`: maximum number of accepted files.
- `validator`: optional consumer-supplied validation hook for application-specific rejection reasons.

`accept` should be a typed Rust structure rather than only a raw string:

- `FileDropZoneAccept::Any`
- `FileDropZoneAccept::Types(Vec<AcceptedFileType>)`
- `AcceptedFileType { mime: Option<String>, extensions: Vec<String> }`

This model can represent react-dropzone's MIME-to-extension map and Mantine's MIME presets while staying Rust-friendly. A raw string convenience constructor may exist, but the canonical API should preserve structured MIME and extension data for validation.

Rules:

- MIME values are ASCII case-insensitive.
- MIME wildcards such as `image/*` are supported.
- Extensions may be supplied with or without a leading dot and are normalized to lowercase without the leading dot for matching.
- Extension matching is ASCII case-insensitive.
- A file is accepted if any configured accepted type matches its MIME type or extension.
- If both MIME and extension are present on one `AcceptedFileType`, either match is sufficient. This avoids rejecting valid browser files when one metadata field is missing.
- The hidden input `accept` attribute is generated from the same structure as a comma-separated list of MIME values and dot-prefixed extensions.

Accept rules should handle both MIME types and file extensions because browser file metadata can be incomplete or inconsistent. The implementation must document that browser accept filtering is advisory and must be revalidated after selection/drop.

### Interaction Props

The API should support:

- `disabled`: prevents drag, drop, click, keyboard, and imperative file picker activation.
- `loading`: visually indicates pending work and prevents new selection. Loading should behave like disabled for interaction.
- `activate_on_click`: controls whether clicking the root opens the file picker.
- `activate_on_keyboard`: controls whether Enter or Space on the root opens the file picker.
- `enable_drag`: controls whether drag/drop events are handled.
- `prevent_drop_on_document`: prevents accidental browser navigation when files are dropped outside the zone.
- `stop_drag_propagation`: prevents drag events from bubbling to parent drop zones when needed.
- `interactive_children`: opt-in mode for preserving pointer events on children inside the drop zone.

When an external button calls the imperative open action, docs should recommend disabling root click activation or stopping propagation to avoid double-opening the file picker. This mirrors react-dropzone's documented caveat for custom open buttons.

The styled component may disable pointer events on non-interactive children to keep root drag/click behavior predictable, following Mantine's documented pattern. If consumers place a button, link, menu, or other interactive control inside the drop zone, they must be able to opt that child back into pointer events through an explicit component prop, helper class, or documented CSS selector.

### Event Callbacks

The API should support:

- `on_drop`: receives both accepted files and file rejections for every completed selection/drop.
- `on_accepted`: receives only accepted files.
- `on_rejected`: receives only rejected files.
- `on_file_dialog_open`: fires when the component requests the native picker.
- `on_file_dialog_cancel`: best-effort callback when the native picker appears to close without files.
- `on_error`: reports unexpected browser or extraction errors.

`on_file_dialog_cancel` must be documented as best-effort. react-dropzone documents cancel detection as unstable in browsers, especially around large files and delayed focus events.

### Imperative Open

The component should expose a signal-driven open request for parent components to open the native file picker programmatically.

Requirements:

- the primitive accepts an optional `open_request` signal or comparable Dioxus trigger value
- changing or incrementing the trigger opens the hidden input's file picker
- no-op when disabled or loading
- uses the same hidden input as click/keyboard activation
- does not bypass validation
- can be used by external buttons in preview examples

Before implementation, the assignee must check existing project conventions for signal-driven imperative actions. If a convention exists, File DropZone should follow it. If no convention exists, use a monotonic request token, such as `Signal<u64>` or an equivalent typed wrapper, so repeated open requests are observable even when the value is otherwise stateless.

## File Data Model

Accepted file callbacks should expose enough browser file metadata for consumers:

- file handle or web file object used by Dioxus/web-sys
- name
- size in bytes
- MIME type when available
- last modified timestamp when available

The component must not promise a stable local path. react-dropzone explicitly notes that dropped files do not include `path` or `fullPath`; browser file APIs generally hide local filesystem paths for privacy.

## Rejection Model

Rejected files should include the original file metadata plus one or more rejection errors.

Required rejection codes:

- `file-invalid-type`
- `file-too-large`
- `file-too-small`
- `too-many-files`
- `custom-validation-failed`

Each rejection error should include:

- stable machine-readable code
- human-readable message
- optional detail value, such as configured size limit or accepted type list

Validation should run in a deterministic order:

1. file type
2. minimum size
3. maximum size
4. max file count
5. custom validator

If a file fails more than one rule, collect all validation failures in a stable order on the file rejection. This matches react-dropzone's `FileRejection` model, where one rejected file can carry multiple `FileError` values, and gives consumers better feedback.

## Drag State Model

The primitive should track these states:

- `idle`: no active drag over the zone
- `drag_active`: one or more drag items are over the zone
- `drag_accept`: dragged items appear acceptable under current rules
- `drag_reject`: dragged items appear unacceptable under current rules
- `file_dialog_active`: native file picker has been requested and has not resolved
- `disabled`: interaction disabled by prop or loading state
- `loading`: consumer-controlled pending state

`drag_accept` and `drag_reject` are provisional during drag because browsers may not expose complete file metadata until drop. The component should revalidate after drop and treat the post-drop result as authoritative.

The styled wrapper should emit data attributes on the root:

- `data-idle`
- `data-drag-active`
- `data-accept`
- `data-reject`
- `data-file-dialog-active`
- `data-disabled`
- `data-loading`

These attributes give the styled crate the same kind of state hooks Mantine exposes with `data-loading`, `data-accept`, `data-reject`, and `data-idle`.

## Accessibility Requirements

The root must be keyboard and screen-reader accessible by default:

- focusable when interactive
- has button semantics unless the implementation uses a native button-compatible element
- supports an accessible label through attributes or explicit prop
- opens the file picker on Enter and Space when keyboard activation is enabled
- does not trap focus
- marks disabled state with appropriate ARIA and DOM attributes
- keeps the hidden input associated with the component

Consumers must be able to pass global attributes to the root. Per project guidance, Dioxus components should use `#[props(extends = GlobalAttributes)]` and merge CSS module classes into forwarded attributes rather than adding a plain manual `class` prop.

## Styling Requirements

The styled component should provide:

- neutral idle state
- visible focus state
- accept state
- reject state
- disabled state
- loading state
- layout that works for compact and full-width usage

The component should support custom content without requiring consumers to override internal behavior. State-aware helper components may mirror Mantine's `Dropzone.Accept`, `Dropzone.Reject`, and `Dropzone.Idle`, but they should be optional and implemented in a Dioxus-idiomatic way.

Interactive children require explicit support. The styled wrapper should document whether child pointer events are disabled by default and provide an ergonomic way to opt interactive children back in. This is required for custom buttons, links, and menus placed inside the drop zone.

## Preview Requirements

Preview should include examples for:

- default drop zone
- images only
- max file size
- max file count
- single-file mode
- disabled
- loading
- external open button
- rejected file feedback
- custom idle/accept/reject content

The examples should show file names and rejection reasons without uploading files.

## Testing And Validation

Primitive behavior should be covered by `cargo test -p dioxus-primitives` where feasible:

- accept rules
- extension matching
- min and max size validation
- max file count validation
- custom validator rejection
- accepted and rejected callback payloads
- disabled/loading no-op behavior
- drag state transitions

Styled crate validation:

- `cargo check -p dioxus-components`

Preview and visual validation:

- `scripts/preview-web.sh build`
- targeted Playwright tests for click selection, keyboard activation, drag accept/reject state, drop rejection display, disabled behavior, and external open button behavior

CSS validation, when preview CSS changes:

- from `preview/`, run `npx stylelint "src/**/*.css"`

## Browser And Security Notes

- The browser `accept` attribute is not a security boundary. Consumers must still validate files before upload or processing.
- File size and type validation are client-side convenience checks and must be repeated on the server for upload workflows.
- File dialog cancel detection is inherently best-effort and should not drive critical state.
- Browser file APIs do not expose reliable local filesystem paths.
- Mobile camera or microphone capture behavior through the input `capture` attribute is platform-specific and not part of the initial contract.
- File System Access API features, if ever added, must be optional because they require secure contexts and have different browser support.
- Directory dropping or folder selection is out of scope for the first version unless explicitly requested later.

## Deliberately Out Of Scope For First Version

The first version should not copy every react-dropzone option. These items are out of scope unless a later implementation plan explicitly adds them:

- asynchronous custom file extraction equivalent to `getFilesFromEvent`
- File System Access API picker mode
- directory selection or directory dropping
- mobile capture/camera picker behavior via the input `capture` attribute
- auto-focus on mount
- global drag state unrelated to this drop zone instance
- internally retained accepted/rejected file history after callbacks fire
- exposing browser-specific local file paths

## Open Questions

- Should full-screen drop behavior be a separate component after the base File DropZone lands?
