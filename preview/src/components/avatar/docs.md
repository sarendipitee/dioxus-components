The Avatar component family is for identity tiles: user cards, team rosters, message threads, and comment feeds where a profile photo may be empty, slow to load, or unavailable. Use the composable `Avatar`, `AvatarImage`, and `AvatarFallback` set when you need explicit control over how each stage is rendered, or `ImageAvatar` when you want a single convenience component that handles image loading and fallback automatically.

## Component Structure

```rust
Avatar {
    aria_label: "Jordan Lee",
    AvatarImage {
        src: "https://example.com/jordan-lee.png",
        alt: "Jordan Lee",
    }
    AvatarFallback { "JL" }
}
```

```rust
ImageAvatar {
    src: "https://example.com/jordan-lee.png",
    alt: "Jordan Lee",
    on_state_change: |state: AvatarState| { /* image is loading / loaded / failed */ },
    "JL"
}
```

Use this page's demos to compare:
- a static avatar with explicit `AvatarImage` + `AvatarFallback`,
- a single-line `ImageAvatar` with loading-state callbacks, and
- fallback-first layouts where initials remain visible until an image fetch succeeds.
