The Progress component communicates how far a bounded operation has advanced, so users can judge completion at a glance. Use it for workflows with an expected endpoint such as uploads, installers, migrations, and multi-stage data syncs.

This page’s demos focus on two practical patterns: a bounded progress bar where `value` and `max` describe exact completion, and a loading state where progress is implicit while work is still in progress.

## Component Structure

```rust
Progress {
    // The current progress value (0.0 to max)
    value: 0.5,
    // The maximum value of the progress (default is 100.0)
    max: 1.0,
    // Elements that will be displayed inside the progress bar
    {children}
}
```
