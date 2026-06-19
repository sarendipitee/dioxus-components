Badge gives you a tiny, inline status chip that can be anchored to nearby content so users can scan importance at a glance. It is most useful in dense interfaces where rows, cards, and toolbar actions need quick context such as “Active,” “Draft,” “Critical,” or “Archived” without adding extra spacing or extra visual weight.

Use this page’s demos to compare:
- a baseline badge label beside plain text,
- grouped badges in action clusters (for quick categorization or queue states),
- and stronger emphasis styles that stand out when status hierarchy matters.

## Component Structure

```rust
Badge {
    {children}
}
```

The snippet above shows the minimal shape of the component: the children you pass become the visible label text. Build variants by combining this shape with variant classes from the styled component package to reflect the same data model across notification pills, workflow tags, and list metadata.
