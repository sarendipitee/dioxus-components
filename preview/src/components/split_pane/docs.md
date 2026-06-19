The SplitPane component is the layout primitive for workspace-like interfaces that need two or more regions that can be reallocated at runtime. It is best used when a user must quickly tune visible real estate between related views, such as an explorer/working area/details stack or top/bottom code and output columns.

## Component Structure

```rust
SplitPane {
    direction: SplitPaneDirection::Horizontal,
    step: 24.0,

    Pane {
        default_size: SplitPaneSize::percent(35.0),
        min_size: SplitPaneSize::px(160.0),
        "Left pane"
    }
    SplitPaneDivider {}
    Pane {
        min_size: SplitPaneSize::px(220.0),
        "Right pane"
    }
}
```

The root `SplitPane` container must be given a concrete size before pane math works reliably. For a horizontal divider layout, the row needs a definite width; for a vertical divider layout, it needs a definite height. In a demo, that usually means sizing the parent container (for example, `width: 100%` inside a fixed-width layout region, `height: 320px`, or `height: 100%` within a tall frame) and then letting each pane fill within those bounds.

## Sizing

Use `default_size` for initial, uncontrolled pane setup and `size` when pane width should be driven by external state. You can mix unit systems in the same row: pair `SplitPaneSize::percent` and `SplitPaneSize::px` with `min_size` and `max_size` to enforce hard stops for narrow sidebars, fixed content zones, or detail panels.

```rust
let mut sidebar = use_signal(|| Some(SplitPaneSize::percent(30.0)));

Pane {
    size: sidebar,
    min_size: SplitPaneSize::px(180.0),
    max_size: SplitPaneSize::percent(50.0),
    "Controlled sidebar"
}
```

## Interaction

Dividers are interactive through pointer drag and keyboard controls, with focus-visible feedback for accessibility. Configure `step` when you want predictable arrow-key increments, and add `snap_points` plus `snap_tolerance` to make the divider settle at familiar ratios while still allowing precise manual movement.

```rust
SplitPane {
    snap_points: vec![SplitPaneSize::percent(25.0), SplitPaneSize::percent(50.0)],
    snap_tolerance: 18.0,
    step: 16.0,
}
```

Add more than two regions by alternating `Pane` and `SplitPaneDivider` children. This pattern is intentionally explicit so you can build asymmetric layouts like "navigation + editor + details" without custom math.

```rust
SplitPane {
    direction: SplitPaneDirection::Horizontal,
    Pane { default_size: SplitPaneSize::percent(22.0), "Nav" }
    SplitPaneDivider {}
    Pane { default_size: SplitPaneSize::percent(48.0), "Editor" }
    SplitPaneDivider {}
    Pane { min_size: SplitPaneSize::px(220.0), "Inspector" }
}
```

## Persistence

`use_split_pane_persistence` gives you a pair of values to initialize from stored measurements and to persist final divider positions after user interaction. Use this for restoring a preferred workspace layout the next time a project is reopened.

```rust
let (stored_sizes, persist_sizes) =
    use_split_pane_persistence("workspace-layout", SplitPaneStorage::Local);

SplitPane {
    on_resize_end: persist_sizes,
    Pane {
        default_size: stored_sizes().and_then(|sizes| sizes.get(0).cloned()),
        "Restored pane"
    }
}
```

## Styling

Pass `divider_class`, `divider_style`, or per-divider `class` and `style` values to brand the resize handles for your app’s visual language. In `Horizontal` mode, the handle behaves like a vertical splitter (`col-resize`), while `Vertical` mode exposes a horizontal splitter (`row-resize`). Both pane and root attributes are forwarded to the primitive, so you can still inject semantic labels, utility classes, and size constraints from the wrapper layer.
