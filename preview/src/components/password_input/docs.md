`PasswordInput` is a password-entry field with a built-in eye toggle that reveals
or hides the value. It is built on the shared [`InputBase`] shell and therefore
inherits the label, description, error, variant, size, radius, and loading
affordances of the other inputs.

The native control renders as `type="password"` and switches to `type="text"`
while the value is revealed.

## Basic usage

```rust
rsx! {
    PasswordInput {
        label: "Password",
        placeholder: "Enter your password",
    }
}
```

## Controlled visibility

By default the bundled eye toggle owns the reveal state. Pass `visible` together
with `on_visibility_change` to control it from the outside — for example to drive
two fields from a single toggle:

```rust
let mut shown = use_signal(|| false);

rsx! {
    PasswordInput {
        label: "Password",
        visible: shown(),
        on_visibility_change: move |v| shown.set(v),
    }
}
```

Use `default_visible: true` to start revealed in uncontrolled mode.

## Hiding the toggle

Set `visibility_toggle: false` to render a plain password field with no eye
button. Any `right_section` content is rendered before the toggle, so the two can
coexist.

## Toggle labels

`show_label` and `hide_label` set the toggle's `aria-label` for each state. They
default to `"Show password"` and `"Hide password"`.

## States

`disabled`, `required`/`with_asterisk`, `error`, and `loading` behave exactly as
they do on [`TextInput`]. While `loading` is set the spinner replaces the toggle
in the trailing section.
