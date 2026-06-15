The Alert component presents concise contextual messaging with an optional icon, title, description, and trailing action area.

## Component Structure

```rust
Alert { variant: AlertVariant::Info,
    AlertIcon { "i" }
    AlertTitle { "Heads up" }
    AlertDescription {
        "This change affects active sessions immediately."
    }
    AlertAction {
        Button { "Review" }
    }
}
```

### Components

- `Alert`: The outer container. Supports `default`, `destructive`, `info`, and `success` variants.
- `AlertIcon`: Leading visual slot for an icon or small symbol.
- `AlertTitle`: Primary heading content for the alert.
- `AlertDescription`: Supporting body content.
- `AlertAction`: Trailing action area for buttons or links.
