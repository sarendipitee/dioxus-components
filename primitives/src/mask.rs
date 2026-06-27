//! Headless input masking, ported from Mantine's [`use-mask`] hook.
//!
//! [`use_mask`] attaches masking behaviour to a native `<input>`: it formats
//! the value as the user types, keeps the caret in a sensible place, supports
//! custom undo/redo, and reports both the masked display value and the raw
//! unmasked value. [`crate::mask`] is renderer agnostic — the algorithm is pure
//! Rust and the imperative DOM glue (caret placement, value writes) is gated to
//! the web target. On non-web renderers the masked value is still computed and
//! surfaced through the returned signals.
//!
//! [`use-mask`]: https://mantine.dev/hooks/use-mask/

// Token predicates are plain `fn` pointers so options stay `Copy`/`PartialEq`
// without a regex dependency. Comparing them by address is imprecise but only
// influences re-render decisions (a stale comparison at worst causes a redundant
// re-render), so the imprecision is acceptable here.
#![allow(unpredictable_function_pointer_comparisons)]

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

/// Predicate deciding whether a character is allowed in a token slot.
///
/// This is the Rust equivalent of a Mantine token `RegExp`. Plain function
/// pointers are used so options stay `Copy`/`PartialEq` without pulling in a
/// regex dependency.
pub type CharPredicate = fn(char) -> bool;

/// Matches a single ASCII digit (`9` token).
pub fn token_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Matches an ASCII letter (`a` token).
pub fn token_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// Matches an ASCII uppercase letter (`A` token).
pub fn token_upper(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// Matches an ASCII letter or digit (`*` token).
pub fn token_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// Matches a sign or digit (`#` token, `[-+0-9]`).
pub fn token_signed_digit(c: char) -> bool {
    c == '-' || c == '+' || c.is_ascii_digit()
}

/// The default Mantine token map.
pub fn default_tokens() -> Vec<(char, CharPredicate)> {
    vec![
        ('9', token_digit as CharPredicate),
        ('a', token_alpha as CharPredicate),
        ('A', token_upper as CharPredicate),
        ('*', token_alphanumeric as CharPredicate),
        ('#', token_signed_digit as CharPredicate),
    ]
}

/// A single element of an array-style mask.
#[derive(Clone, PartialEq)]
pub enum MaskItem {
    /// A literal string; each character becomes its own fixed slot.
    Literal(String),
    /// An editable token slot validated by the given predicate.
    Token(CharPredicate),
}

/// A mask pattern, either a token string (`"(999) 999-9999"`) or an explicit
/// array of literals and token predicates.
#[derive(Clone, PartialEq)]
pub enum MaskPattern {
    /// Token-string form, e.g. `"+1 (999) 999-9999"`.
    String(String),
    /// Array form mixing literal strings and token predicates.
    Items(Vec<MaskItem>),
}

impl From<&str> for MaskPattern {
    fn from(value: &str) -> Self {
        MaskPattern::String(value.to_string())
    }
}

impl From<String> for MaskPattern {
    fn from(value: String) -> Self {
        MaskPattern::String(value)
    }
}

impl From<Vec<MaskItem>> for MaskPattern {
    fn from(value: Vec<MaskItem>) -> Self {
        MaskPattern::Items(value)
    }
}

/// Overrides returned from [`UseMaskOptions::modify`] to reshape the mask per
/// keystroke based on the current raw value.
#[derive(Clone, PartialEq, Default)]
pub struct MaskModify {
    /// Replacement mask pattern.
    pub mask: Option<MaskPattern>,
    /// Additional or overriding tokens.
    pub tokens: Option<Vec<(char, CharPredicate)>>,
    /// Replacement slot character.
    pub slot_char: Option<String>,
    /// Replacement `separate` flag (kept for parity; currently unused, matching
    /// upstream).
    pub separate: Option<bool>,
}

/// Options controlling [`use_mask`]. Mirrors Mantine's `UseMaskOptions`.
#[derive(Clone, PartialEq)]
pub struct UseMaskOptions {
    /// Mask pattern string or array of literals and token predicates.
    pub mask: MaskPattern,
    /// Extra tokens layered over (and overriding) [`default_tokens`].
    pub tokens: Vec<(char, CharPredicate)>,
    /// Called before masking on each keystroke; may override mask options for
    /// that keystroke based on the current raw value.
    pub modify: Option<Callback<String, Option<MaskModify>>>,
    /// Decouples raw and display values. Kept for parity; currently unused,
    /// matching upstream.
    pub separate: bool,
    /// Character shown in unfilled slots. Empty disables slot placeholders.
    /// A multi-character value is indexed per slot. `"_"` by default.
    pub slot_char: String,
    /// Show the mask pattern even when empty and unfocused.
    pub always_show_mask: bool,
    /// Show the mask placeholder on focus. `true` by default.
    pub show_mask_on_focus: bool,
    /// Transform each character before validation and insertion.
    pub transform: Option<Callback<char, char>>,
    /// Clear the value on blur when the mask is incomplete. `false` by default.
    pub auto_clear: bool,
    /// Sets `aria-invalid` on the input (consumed by the styled wrapper).
    pub invalid: bool,
    /// Called on every change with `(raw_value, masked_value)`.
    pub on_change_raw: Option<Callback<(String, String)>>,
    /// Called when all required mask slots are filled with `(masked, raw)`.
    pub on_complete: Option<Callback<(String, String)>>,
    /// Initial value seeded into the input on mount.
    pub initial_value: String,
}

impl Default for UseMaskOptions {
    fn default() -> Self {
        Self {
            mask: MaskPattern::String(String::new()),
            tokens: Vec::new(),
            modify: None,
            separate: false,
            slot_char: "_".to_string(),
            always_show_mask: false,
            show_mask_on_focus: true,
            transform: None,
            auto_clear: false,
            invalid: false,
            on_change_raw: None,
            on_complete: None,
            initial_value: String::new(),
        }
    }
}

/// Value returned from [`use_mask`].
#[derive(Clone, Copy)]
pub struct UseMask {
    /// Current masked display value.
    pub value: ReadSignal<String>,
    /// Current raw, unmasked value.
    pub raw_value: ReadSignal<String>,
    /// Whether all required mask slots are filled.
    pub is_complete: ReadSignal<bool>,
    /// Unique id; attach as `data-dx-mask-id` on the input so the hook can
    /// reach the element for caret management on the web target.
    pub mask_id: usize,
    /// Clears the value and resets internal state.
    pub reset: Callback<()>,
    /// Attach to the input's `onmounted`.
    pub onmounted: Callback<MountedEvent>,
    /// Attach to the input's `oninput`.
    pub oninput: Callback<FormEvent>,
    /// Attach to the input's `onkeydown`.
    pub onkeydown: Callback<KeyboardEvent>,
    /// Attach to the input's `onfocus`.
    pub onfocus: Callback<FocusEvent>,
    /// Attach to the input's `onblur`.
    pub onblur: Callback<FocusEvent>,
    /// Attach to the input's `onmousedown`.
    pub onmousedown: Callback<MouseEvent>,
    /// Attach to the input's `onmouseup`.
    pub onmouseup: Callback<MouseEvent>,
}

// ---------------------------------------------------------------------------
// Pure masking core
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum SlotKind {
    Token,
    Literal,
}

#[derive(Clone)]
struct MaskSlot {
    kind: SlotKind,
    ch: char,
    pattern: Option<CharPredicate>,
    optional: bool,
}

impl MaskSlot {
    fn is_literal(&self) -> bool {
        matches!(self.kind, SlotKind::Literal)
    }

    fn is_token(&self) -> bool {
        matches!(self.kind, SlotKind::Token)
    }

    fn matches(&self, c: char) -> bool {
        self.pattern.map(|p| p(c)).unwrap_or(false)
    }
}

fn lookup_token(tokens: &[(char, CharPredicate)], c: char) -> Option<CharPredicate> {
    tokens.iter().find(|(k, _)| *k == c).map(|(_, p)| *p)
}

fn merge_tokens(base: &mut Vec<(char, CharPredicate)>, extra: &[(char, CharPredicate)]) {
    for (c, p) in extra {
        if let Some(slot) = base.iter_mut().find(|(k, _)| k == c) {
            slot.1 = *p;
        } else {
            base.push((*c, *p));
        }
    }
}

fn parse_mask(mask: &MaskPattern, tokens: &[(char, CharPredicate)]) -> Vec<MaskSlot> {
    match mask {
        MaskPattern::Items(items) => {
            let mut slots = Vec::new();
            for item in items {
                match item {
                    MaskItem::Token(p) => slots.push(MaskSlot {
                        kind: SlotKind::Token,
                        ch: '_',
                        pattern: Some(*p),
                        optional: false,
                    }),
                    MaskItem::Literal(s) => {
                        for c in s.chars() {
                            slots.push(MaskSlot {
                                kind: SlotKind::Literal,
                                ch: c,
                                pattern: None,
                                optional: false,
                            });
                        }
                    }
                }
            }
            slots
        }
        MaskPattern::String(s) => {
            let chars: Vec<char> = s.chars().collect();
            let mut slots = Vec::new();
            let mut optional = false;
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];
                if c == '\\' && i + 1 < chars.len() {
                    i += 1;
                    slots.push(MaskSlot {
                        kind: SlotKind::Literal,
                        ch: chars[i],
                        pattern: None,
                        optional,
                    });
                    i += 1;
                    continue;
                }
                if c == '?' {
                    optional = true;
                    i += 1;
                    continue;
                }
                if let Some(pattern) = lookup_token(tokens, c) {
                    slots.push(MaskSlot {
                        kind: SlotKind::Token,
                        ch: c,
                        pattern: Some(pattern),
                        optional,
                    });
                } else {
                    slots.push(MaskSlot {
                        kind: SlotKind::Literal,
                        ch: c,
                        pattern: None,
                        optional,
                    });
                }
                i += 1;
            }
            slots
        }
    }
}

fn get_slot_char(slot_char: &str, index: usize) -> Option<char> {
    if slot_char.is_empty() {
        return None;
    }
    let chars: Vec<char> = slot_char.chars().collect();
    if chars.len() > 1 {
        Some(chars.get(index).copied().unwrap_or('_'))
    } else {
        Some(chars[0])
    }
}

fn apply_mask_to_raw(
    raw: &[char],
    slots: &[MaskSlot],
    transform: Option<Callback<char, char>>,
) -> Vec<char> {
    let mut result = Vec::new();
    let mut raw_index = 0;
    let mut slot_index = 0;

    while slot_index < slots.len() {
        let slot = &slots[slot_index];
        if slot.is_literal() {
            result.push(slot.ch);
            slot_index += 1;
            continue;
        }

        if raw_index >= raw.len() {
            break;
        }

        let ch = transform
            .map(|t| t.call(raw[raw_index]))
            .unwrap_or(raw[raw_index]);
        if slot.matches(ch) {
            result.push(ch);
            raw_index += 1;
            slot_index += 1;
        } else {
            // Skip the rejected raw character and retry the same slot.
            raw_index += 1;
        }
    }

    result
}

fn build_display_value(
    value: &[char],
    slots: &[MaskSlot],
    slot_char: &str,
    show_slots: bool,
) -> Vec<char> {
    if !show_slots {
        return value.to_vec();
    }

    let mut display = value.to_vec();
    for (i, slot) in slots.iter().enumerate().skip(value.len()) {
        if slot.is_literal() {
            display.push(slot.ch);
        } else {
            match get_slot_char(slot_char, i) {
                Some(sc) => display.push(sc),
                None => break,
            }
        }
    }
    display
}

fn extract_raw(masked: &[char], slots: &[MaskSlot]) -> Vec<char> {
    let mut raw = Vec::new();
    let end = masked.len().min(slots.len());
    for i in 0..end {
        if slots[i].is_token() {
            raw.push(masked[i]);
        }
    }
    raw
}

fn check_complete(masked: &[char], slots: &[MaskSlot]) -> bool {
    for (i, slot) in slots.iter().enumerate() {
        if slot.is_token() && !slot.optional {
            if i >= masked.len() {
                return false;
            }
            if !slot.matches(masked[i]) {
                return false;
            }
        }
    }
    true
}

fn process_input(input: &[char], slots: &[MaskSlot]) -> Vec<char> {
    let mut result = Vec::new();
    let mut input_index = 0;
    let mut slot_index = 0;

    while slot_index < slots.len() && input_index <= input.len() {
        let slot = &slots[slot_index];

        if slot.is_literal() {
            result.push(slot.ch);
            if input_index < input.len() && input[input_index] == slot.ch {
                input_index += 1;
            }
            slot_index += 1;
            continue;
        }

        if input_index >= input.len() {
            break;
        }

        while input_index < input.len() {
            let ch = input[input_index];
            input_index += 1;
            if slot.matches(ch) {
                result.push(ch);
                break;
            }
        }

        if result.len() <= slot_index {
            break;
        }
        slot_index += 1;
    }

    result
}

fn find_next_token_index(slots: &[MaskSlot], from: usize) -> usize {
    for (i, slot) in slots.iter().enumerate().skip(from) {
        if slot.is_token() {
            return i;
        }
    }
    slots.len()
}

fn find_prev_token_index(slots: &[MaskSlot], from: usize) -> i32 {
    let mut i = from as i32;
    while i >= 0 {
        if slots[i as usize].is_token() {
            return i;
        }
        i -= 1;
    }
    -1
}

fn find_next_editable_position(from: usize, slots: &[MaskSlot], value_len: usize) -> usize {
    let mut pos = from;
    while pos < slots.len() && pos < value_len && slots[pos].is_literal() {
        pos += 1;
    }
    pos
}

// Clamped slicing helpers mirroring JS `String.slice`, which never panics.
fn slice_range<T>(v: &[T], a: usize, b: usize) -> &[T] {
    let len = v.len();
    let a = a.min(len);
    let b = b.min(len);
    if a > b {
        &v[len..len]
    } else {
        &v[a..b]
    }
}

fn slice_from<T>(v: &[T], a: usize) -> &[T] {
    &v[a.min(v.len())..]
}

struct Resolved {
    slots: Vec<MaskSlot>,
    slot_char: String,
}

fn resolve(opts: &UseMaskOptions, raw: &str) -> Resolved {
    let mut tokens = default_tokens();
    merge_tokens(&mut tokens, &opts.tokens);
    let mut mask = opts.mask.clone();
    let mut slot_char = opts.slot_char.clone();

    if let Some(modify) = opts.modify {
        if let Some(overrides) = modify.call(raw.to_string()) {
            if let Some(m) = overrides.mask {
                mask = m;
            }
            if let Some(t) = overrides.tokens {
                merge_tokens(&mut tokens, &t);
            }
            if let Some(sc) = overrides.slot_char {
                slot_char = sc;
            }
            // `separate` is resolved upstream but unused; mirrored here.
        }
    }

    let slots = parse_mask(&mask, &tokens);
    Resolved { slots, slot_char }
}

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn string(v: &[char]) -> String {
    v.iter().collect()
}

// ---------------------------------------------------------------------------
// Public pure helpers (Mantine `formatMask` / `unformatMask` / `isMaskComplete`)
// ---------------------------------------------------------------------------

/// Format a raw value through the mask, returning the masked display string.
pub fn format_mask(raw: &str, options: &UseMaskOptions) -> String {
    let r = resolve(options, raw);
    string(&apply_mask_to_raw(&chars(raw), &r.slots, options.transform))
}

/// Extract the raw, unmasked value from a masked string.
pub fn unformat_mask(masked: &str, options: &UseMaskOptions) -> String {
    let r = resolve(options, "");
    string(&extract_raw(&chars(masked), &r.slots))
}

/// Whether a masked string fills all required mask slots.
pub fn is_mask_complete(masked: &str, options: &UseMaskOptions) -> bool {
    let r = resolve(options, "");
    check_complete(&chars(masked), &r.slots)
}

// ---------------------------------------------------------------------------
// DOM glue (web only)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod dom {
    use wasm_bindgen::JsCast;

    fn input(id: usize) -> Option<web_sys::HtmlInputElement> {
        let document = web_sys::window()?.document()?;
        let element = document
            .query_selector(&format!("input[data-dx-mask-id=\"{id}\"]"))
            .ok()??;
        element.dyn_into::<web_sys::HtmlInputElement>().ok()
    }

    pub fn get_value(id: usize) -> Option<String> {
        Some(input(id)?.value())
    }

    pub fn set_value(id: usize, value: &str) {
        if let Some(el) = input(id) {
            el.set_value(value);
        }
    }

    pub fn selection(id: usize) -> Option<(usize, usize)> {
        let el = input(id)?;
        let start = el.selection_start().ok().flatten().unwrap_or(0) as usize;
        let end = el.selection_end().ok().flatten().unwrap_or(0) as usize;
        Some((start, end))
    }

    pub fn set_cursor(id: usize, pos: usize) {
        if let Some(el) = input(id) {
            let _ = el.set_selection_range(pos as u32, pos as u32);
        }
    }

    pub fn is_active(id: usize) -> bool {
        (|| {
            let document = web_sys::window()?.document()?;
            let el = input(id)?;
            let active = document.active_element()?;
            Some(active.is_same_node(Some(el.as_ref())))
        })()
        .unwrap_or(false)
    }

    pub fn set_value_and_cursor(id: usize, value: &str, cursor: Option<usize>, max: usize) {
        if let Some(el) = input(id) {
            el.set_value(value);
            if let Some(c) = cursor {
                if is_active(id) {
                    let pos = c.min(max) as u32;
                    let _ = el.set_selection_range(pos, pos);
                }
            }
        }
    }
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
mod dom {
    pub fn get_value(_id: usize) -> Option<String> {
        None
    }
    pub fn set_value(_id: usize, _value: &str) {}
    pub fn selection(_id: usize) -> Option<(usize, usize)> {
        None
    }
    pub fn set_cursor(_id: usize, _pos: usize) {}
    pub fn set_value_and_cursor(_id: usize, _value: &str, _cursor: Option<usize>, _max: usize) {}
}

const MAX_UNDO_HISTORY: usize = 100;

#[derive(Clone, PartialEq)]
struct UndoState {
    raw: String,
    selection_start: usize,
}

struct ApplyArgs {
    reprocessed: Vec<char>,
    new_raw: Vec<char>,
    display_value: Vec<char>,
    resolved_slots: Vec<MaskSlot>,
    cursor: Option<usize>,
    notify: bool,
}

/// Attach input masking to a native `<input>`.
///
/// Wire the returned event handlers onto an `input` element and set
/// `data-dx-mask-id` to [`UseMask::mask_id`] so caret management can find the
/// element on the web target. The input should be left uncontrolled on the web
/// target (the hook writes its value imperatively); on other renderers, bind
/// the element value to [`UseMask::value`].
pub fn use_mask(options: UseMaskOptions) -> UseMask {
    static NEXT_MASK_ID: AtomicUsize = AtomicUsize::new(0);

    let mask_id = use_hook(|| NEXT_MASK_ID.fetch_add(1, Ordering::Relaxed));

    // Mirror of `optionsRef.current`: always the latest options, without
    // triggering re-renders when read inside handlers.
    let mut opts_store = use_hook(|| CopyValue::new(options.clone()));
    *opts_store.write() = options.clone();

    let mut display = use_signal(String::new); // display + masked value
    let mut raw = use_signal(String::new);
    let mut processed = use_signal(String::new);
    let mut complete = use_signal(|| false);
    let mut was_complete = use_signal(|| false);
    let mut is_focused = use_signal(|| false);
    let mut undo_stack = use_hook(|| CopyValue::new(Vec::<UndoState>::new()));
    let mut redo_stack = use_hook(|| CopyValue::new(Vec::<UndoState>::new()));

    let apply_value = use_callback(move |args: ApplyArgs| {
        let opts = opts_store.read().clone();
        let reprocessed_s = string(&args.reprocessed);
        let display_s = string(&args.display_value);
        let raw_s = string(&args.new_raw);

        processed.set(reprocessed_s);
        display.set(display_s.clone());
        raw.set(raw_s.clone());

        dom::set_value_and_cursor(mask_id, &display_s, args.cursor, args.reprocessed.len());

        if args.notify {
            if let Some(cb) = opts.on_change_raw {
                cb.call((raw_s.clone(), display_s.clone()));
            }
        }

        let comp = check_complete(&args.reprocessed, &args.resolved_slots);
        if args.notify && comp && !*was_complete.peek() {
            if let Some(cb) = opts.on_complete {
                cb.call((display_s.clone(), raw_s.clone()));
            }
        }
        was_complete.set(comp);
        complete.set(comp);
    });

    let update_value = use_callback(move |(new_masked, cursor): (Vec<char>, Option<usize>)| {
        let opts = opts_store.read().clone();

        let base = resolve(&opts, "");
        let raw_a = extract_raw(&new_masked, &base.slots);
        let resolved_a = resolve(&opts, &string(&raw_a));
        let raw_b = extract_raw(&new_masked, &resolved_a.slots);
        let r = resolve(&opts, &string(&raw_b));

        let reprocessed = process_input(&new_masked, &r.slots);
        let new_raw = extract_raw(&reprocessed, &r.slots);

        let show_slots = opts.always_show_mask || *is_focused.peek();
        let should_show = show_slots && (opts.show_mask_on_focus || !reprocessed.is_empty());
        let display_value = build_display_value(&reprocessed, &r.slots, &r.slot_char, should_show);

        apply_value.call(ApplyArgs {
            reprocessed,
            new_raw,
            display_value,
            resolved_slots: r.slots,
            cursor,
            notify: true,
        });
    });

    let push_undo = use_callback(move |_| {
        let selection_start = dom::selection(mask_id)
            .map(|(s, _)| s)
            .unwrap_or_else(|| raw.peek().chars().count());
        let state = UndoState {
            raw: raw.peek().clone(),
            selection_start,
        };
        let mut stack = undo_stack.write();
        if let Some(top) = stack.last() {
            if *top == state {
                return;
            }
        }
        stack.push(state);
        if stack.len() > MAX_UNDO_HISTORY {
            stack.remove(0);
        }
        drop(stack);
        redo_stack.write().clear();
    });

    let apply_history = use_callback(move |target: UndoState| {
        let opts = opts_store.read().clone();
        let r = resolve(&opts, &target.raw);
        let new_masked = apply_mask_to_raw(&chars(&target.raw), &r.slots, opts.transform);
        update_value.call((new_masked, Some(target.selection_start)));
    });

    let do_clamp = use_callback(move |_| {
        let (start, end) = match dom::selection(mask_id) {
            Some(v) => v,
            None => return,
        };
        if start != end {
            return;
        }
        let opts = opts_store.read().clone();
        let r = resolve(&opts, "");
        let proc: Vec<char> = processed.peek().chars().collect();
        let display_len = display.peek().chars().count();
        let end_pos = if !proc.is_empty() {
            find_next_editable_position(proc.len(), &r.slots, display_len)
        } else {
            find_next_token_index(&r.slots, 0)
        };
        let start_pos = find_next_token_index(&r.slots, 0);
        if start > end_pos || start < start_pos {
            dom::set_cursor(mask_id, end_pos);
        }
    });

    let oninput = use_callback(move |e: FormEvent| {
        let opts = opts_store.read().clone();
        let r = resolve(&opts, "");
        let prev: Vec<char> = display.peek().chars().collect();
        let curr: Vec<char> = e.value().chars().collect();

        let mut prefix = 0;
        let max_prefix = prev.len().min(curr.len());
        while prefix < max_prefix && prev[prefix] == curr[prefix] {
            prefix += 1;
        }

        let mut suffix = 0;
        let max_suffix = (prev.len() - prefix).min(curr.len() - prefix);
        while suffix < max_suffix
            && prev[prev.len() - 1 - suffix] == curr[curr.len() - 1 - suffix]
        {
            suffix += 1;
        }

        let inserted: Vec<char> = curr[prefix..curr.len() - suffix].to_vec();
        let removed_end = prev.len() - suffix;

        let before_raw = extract_raw(
            slice_range(&prev, 0, prefix),
            slice_range(&r.slots, 0, prefix),
        );
        let after_raw = extract_raw(
            slice_from(&prev, removed_end),
            slice_from(&r.slots, removed_end),
        );

        let mut combined = before_raw.clone();
        combined.extend(inserted.iter().copied());
        combined.extend(after_raw.iter().copied());
        let reformatted = apply_mask_to_raw(&combined, &r.slots, opts.transform);

        let mut prefix_combined = before_raw.clone();
        prefix_combined.extend(inserted.iter().copied());
        let masked_prefix = apply_mask_to_raw(&prefix_combined, &r.slots, opts.transform);

        if reformatted != prev {
            push_undo.call(());
        }
        update_value.call((reformatted, Some(masked_prefix.len())));
    });

    let onkeydown = use_callback(move |event: KeyboardEvent| {
        let opts = opts_store.read().clone();
        let r = resolve(&opts, &raw.peek().clone());
        let slots = &r.slots;

        let (start, end) = match dom::selection(mask_id) {
            Some(v) => v,
            None => return,
        };
        let proc: Vec<char> = processed.peek().chars().collect();

        let mods = event.modifiers();
        let shift = mods.shift();
        let modifier = mods.meta() || (mods.ctrl() && !mods.alt());
        let key = event.key();

        // Undo / redo.
        if modifier {
            if let Key::Character(s) = &key {
                let k = s.to_lowercase();
                if k == "z" && !shift {
                    event.prevent_default();
                    let prev = undo_stack.write().pop();
                    if let Some(prev) = prev {
                        redo_stack.write().push(UndoState {
                            raw: raw.peek().clone(),
                            selection_start: start,
                        });
                        apply_history.call(prev);
                    }
                    return;
                }
                if (k == "z" && shift) || (k == "y" && !shift) {
                    event.prevent_default();
                    let next = redo_stack.write().pop();
                    if let Some(next) = next {
                        undo_stack.write().push(UndoState {
                            raw: raw.peek().clone(),
                            selection_start: start,
                        });
                        apply_history.call(next);
                    }
                    return;
                }
            }
        }

        match &key {
            Key::Backspace => {
                event.prevent_default();

                // Ctrl/Cmd+Backspace: delete everything before the caret.
                if mods.meta() || (mods.ctrl() && !mods.alt()) {
                    let clamped_start = start.min(proc.len());
                    let after_raw = extract_raw(
                        slice_from(&proc, clamped_start),
                        slice_from(slots, clamped_start),
                    );
                    let new_value = apply_mask_to_raw(&after_raw, slots, opts.transform);
                    push_undo.call(());
                    update_value.call((new_value, Some(0)));
                    return;
                }

                if start != end {
                    let clamped_end = end.min(proc.len());
                    let before_raw =
                        extract_raw(slice_range(&proc, 0, start), slice_range(slots, 0, start));
                    let after_raw = extract_raw(
                        slice_from(&proc, clamped_end),
                        slice_from(slots, clamped_end),
                    );
                    let mut combined = before_raw;
                    combined.extend(after_raw);
                    let new_value = apply_mask_to_raw(&combined, slots, opts.transform);
                    push_undo.call(());
                    update_value.call((new_value, Some(start)));
                    return;
                }

                if start == 0 {
                    return;
                }

                let mut delete_pos = start as i32 - 1;
                while delete_pos >= 0
                    && (delete_pos as usize) < slots.len()
                    && slots[delete_pos as usize].is_literal()
                {
                    delete_pos -= 1;
                }
                if delete_pos < 0 {
                    return;
                }
                let delete_pos = delete_pos as usize;
                let before_raw = extract_raw(
                    slice_range(&proc, 0, delete_pos),
                    slice_range(slots, 0, delete_pos),
                );
                let after_raw = extract_raw(
                    slice_from(&proc, delete_pos + 1),
                    slice_from(slots, delete_pos + 1),
                );
                let mut combined = before_raw;
                combined.extend(after_raw);
                let new_value = apply_mask_to_raw(&combined, slots, opts.transform);
                push_undo.call(());
                update_value.call((new_value, Some(delete_pos)));
            }
            Key::Delete => {
                event.prevent_default();

                if start != end {
                    let clamped_end = end.min(proc.len());
                    let before_raw =
                        extract_raw(slice_range(&proc, 0, start), slice_range(slots, 0, start));
                    let after_raw = extract_raw(
                        slice_from(&proc, clamped_end),
                        slice_from(slots, clamped_end),
                    );
                    let mut combined = before_raw;
                    combined.extend(after_raw);
                    let new_value = apply_mask_to_raw(&combined, slots, opts.transform);
                    push_undo.call(());
                    update_value.call((new_value, Some(start)));
                    return;
                }

                let mut delete_pos = start;
                while delete_pos < slots.len() && slots[delete_pos].is_literal() {
                    delete_pos += 1;
                }
                if delete_pos >= proc.len() {
                    return;
                }
                let before_raw =
                    extract_raw(slice_range(&proc, 0, start), slice_range(slots, 0, start));
                let after_raw = extract_raw(
                    slice_from(&proc, delete_pos + 1),
                    slice_from(slots, delete_pos + 1),
                );
                let mut combined = before_raw;
                combined.extend(after_raw);
                let new_value = apply_mask_to_raw(&combined, slots, opts.transform);
                push_undo.call(());
                update_value.call((new_value, Some(start)));
            }
            Key::ArrowRight if !shift => {
                let value_len = display.peek().chars().count();
                let next_pos = find_next_editable_position(start + 1, slots, value_len);
                if next_pos != start + 1 {
                    event.prevent_default();
                    dom::set_cursor(mask_id, next_pos);
                }
            }
            Key::ArrowLeft if !shift => {
                if start > 0 {
                    let prev_token = find_prev_token_index(slots, start - 1);
                    if prev_token >= 0 && prev_token as usize != start - 1 {
                        event.prevent_default();
                        dom::set_cursor(mask_id, prev_token as usize + 1);
                    }
                }
            }
            Key::Character(s) if !mods.ctrl() && !mods.meta() && !mods.alt() => {
                let typed: Vec<char> = s.chars().collect();
                if typed.len() != 1 {
                    return;
                }
                event.prevent_default();

                let mut insert_pos = start.min(proc.len());
                while insert_pos < slots.len() && slots[insert_pos].is_literal() {
                    insert_pos += 1;
                }
                if insert_pos >= slots.len() {
                    return;
                }

                let ch = opts.transform.map(|t| t.call(typed[0])).unwrap_or(typed[0]);
                if !slots[insert_pos].matches(ch) {
                    return;
                }

                let before_raw = extract_raw(
                    slice_range(&proc, 0, insert_pos),
                    slice_range(slots, 0, insert_pos),
                );
                let after_raw = if start < end {
                    let from = end.min(proc.len());
                    extract_raw(slice_from(&proc, from), slice_from(slots, from))
                } else {
                    extract_raw(slice_from(&proc, insert_pos), slice_from(slots, insert_pos))
                };

                let mut combined = before_raw;
                combined.push(ch);
                combined.extend(after_raw);
                let new_value = apply_mask_to_raw(&combined, slots, opts.transform);
                let new_cursor =
                    find_next_editable_position(insert_pos + 1, slots, new_value.len());
                push_undo.call(());
                update_value.call((new_value, Some(new_cursor)));
            }
            _ => {}
        }
    });

    let onfocus = use_callback(move |_e: FocusEvent| {
        is_focused.set(true);
        let opts = opts_store.read().clone();
        let r = resolve(&opts, "");
        let proc: Vec<char> = processed.peek().chars().collect();

        if opts.show_mask_on_focus || opts.always_show_mask {
            let display_value = build_display_value(&proc, &r.slots, &r.slot_char, true);
            let display_s = string(&display_value);
            dom::set_value(mask_id, &display_s);
            display.set(display_s);
        }
        do_clamp.call(());
    });

    let onblur = use_callback(move |_e: FocusEvent| {
        is_focused.set(false);
        let opts = opts_store.read().clone();
        let r = resolve(&opts, &raw.peek().clone());
        let proc: Vec<char> = processed.peek().chars().collect();

        let expected = build_display_value(&proc, &r.slots, &r.slot_char, true);
        let curr: Vec<char> = dom::get_value(mask_id)
            .map(|s| chars(&s))
            .unwrap_or_else(|| display.peek().chars().collect());
        let processed2 = if curr == expected {
            proc.clone()
        } else {
            process_input(&curr, &r.slots)
        };
        let comp = check_complete(&processed2, &r.slots);

        let mut clear_all = || {
            dom::set_value(mask_id, "");
            processed.set(String::new());
            display.set(String::new());
            raw.set(String::new());
            was_complete.set(false);
            complete.set(false);
            if let Some(cb) = opts.on_change_raw {
                cb.call((String::new(), String::new()));
            }
        };

        if opts.auto_clear && !comp && !processed2.is_empty() {
            clear_all();
            if opts.always_show_mask {
                let empty = build_display_value(&[], &r.slots, &r.slot_char, true);
                let empty_s = string(&empty);
                dom::set_value(mask_id, &empty_s);
                display.set(empty_s);
            }
            return;
        }

        if !opts.always_show_mask && !comp {
            if extract_raw(&processed2, &r.slots).is_empty() {
                clear_all();
                return;
            }
            let display_value = build_display_value(&processed2, &r.slots, &r.slot_char, false);
            let display_s = string(&display_value);
            dom::set_value(mask_id, &display_s);
            display.set(display_s);
        }
    });

    let onmousedown = use_callback(move |_e: MouseEvent| {
        do_clamp.call(());
    });
    let onmouseup = use_callback(move |_e: MouseEvent| {
        do_clamp.call(());
    });

    let onmounted = use_callback(move |_e: MountedEvent| {
        let opts = opts_store.read().clone();
        let init = if !opts.initial_value.is_empty() {
            opts.initial_value.clone()
        } else {
            dom::get_value(mask_id).unwrap_or_default()
        };

        if !init.is_empty() {
            let base = resolve(&opts, "");
            let initial_processed = process_input(&chars(&init), &base.slots);
            let initial_raw = extract_raw(&initial_processed, &base.slots);
            let r = resolve(&opts, &string(&initial_raw));
            let reprocessed = process_input(&chars(&init), &r.slots);
            let new_raw = extract_raw(&reprocessed, &r.slots);
            let show_slots = opts.always_show_mask || *is_focused.peek();
            let should_show = show_slots && (opts.show_mask_on_focus || !reprocessed.is_empty());
            let display_value =
                build_display_value(&reprocessed, &r.slots, &r.slot_char, should_show);
            apply_value.call(ApplyArgs {
                reprocessed,
                new_raw,
                display_value,
                resolved_slots: r.slots,
                cursor: None,
                notify: false,
            });
        } else if opts.always_show_mask {
            let r = resolve(&opts, "");
            let display_value = build_display_value(&[], &r.slots, &r.slot_char, true);
            let display_s = string(&display_value);
            dom::set_value(mask_id, &display_s);
            display.set(display_s);
        }
    });

    let reset = use_callback(move |_| {
        let opts = opts_store.read().clone();
        processed.set(String::new());
        display.set(String::new());
        raw.set(String::new());
        undo_stack.write().clear();
        redo_stack.write().clear();
        was_complete.set(false);
        complete.set(false);

        if opts.always_show_mask {
            let r = resolve(&opts, "");
            let display_value = build_display_value(&[], &r.slots, &r.slot_char, true);
            let display_s = string(&display_value);
            dom::set_value(mask_id, &display_s);
            display.set(display_s);
        } else {
            dom::set_value(mask_id, "");
        }

        if let Some(cb) = opts.on_change_raw {
            cb.call((String::new(), String::new()));
        }
    });

    UseMask {
        value: display.into(),
        raw_value: raw.into(),
        is_complete: complete.into(),
        mask_id,
        reset,
        onmounted,
        oninput,
        onkeydown,
        onfocus,
        onblur,
        onmousedown,
        onmouseup,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn phone() -> UseMaskOptions {
        UseMaskOptions {
            mask: MaskPattern::String("(999) 999-9999".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn formats_partial_and_full_phone() {
        let opts = phone();
        assert_eq!(format_mask("1", &opts), "(1");
        assert_eq!(format_mask("123", &opts), "(123) ");
        assert_eq!(format_mask("1234567890", &opts), "(123) 456-7890");
    }

    #[test]
    fn ignores_invalid_characters() {
        let opts = phone();
        assert_eq!(format_mask("12a34", &opts), "(123) 4");
    }

    #[test]
    fn unformat_strips_literals() {
        let opts = phone();
        assert_eq!(unformat_mask("(123) 456-7890", &opts), "1234567890");
    }

    #[test]
    fn completeness_respects_required_slots() {
        let opts = phone();
        assert!(!is_mask_complete("(123) 456-789", &opts));
        assert!(is_mask_complete("(123) 456-7890", &opts));
    }

    #[test]
    fn optional_slots_are_not_required() {
        let opts = UseMaskOptions {
            mask: MaskPattern::String("99?99".to_string()),
            ..Default::default()
        };
        // Two required digits, two optional digits.
        assert!(is_mask_complete("12", &opts));
        assert!(is_mask_complete("1234", &opts));
        assert!(!is_mask_complete("1", &opts));
    }

    #[test]
    fn escaped_literal_is_not_a_token() {
        let opts = UseMaskOptions {
            // Escaped '9' is a literal, then a digit token.
            mask: MaskPattern::String("\\9 9".to_string()),
            ..Default::default()
        };
        assert_eq!(format_mask("5", &opts), "9 5");
        assert_eq!(unformat_mask("9 5", &opts), "5");
    }

    #[test]
    fn array_mask_with_predicates() {
        fn digit(c: char) -> bool {
            c.is_ascii_digit()
        }
        let opts = UseMaskOptions {
            mask: MaskPattern::Items(vec![
                MaskItem::Literal("+".to_string()),
                MaskItem::Token(digit as CharPredicate),
                MaskItem::Literal("-".to_string()),
                MaskItem::Token(digit as CharPredicate),
            ]),
            ..Default::default()
        };
        assert_eq!(format_mask("12", &opts), "+1-2");
        assert!(is_mask_complete("+1-2", &opts));
    }
}
