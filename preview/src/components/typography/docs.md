Typography provides shared `Text` and `Heading` components for reusable styled copy. Use it when component content needs consistent size, tone, weight, wrapping, alignment, and semantic element control without taking over primitive accessibility behavior.

`Heading` defaults to `HeadingLevel::H2`. Choose a different `level` when page structure needs it, and choose `size` independently when visual scale should differ from the semantic heading level.

## Component Structure

```rust
Heading {
    level: HeadingLevel::H2,
    size: TypographySize::Lg,
    weight: TypographyWeight::Bold,
    "Settings"
}

Text {
    size: TypographySize::Sm,
    tone: TypographyTone::Muted,
    "Manage how components render shared text."
}
```

`Text` renders a paragraph by default and can render `span`, `div`, or `label` through `TextElement`. Both components accept global attributes, so classes and DOM attributes merge with their typography classes.
