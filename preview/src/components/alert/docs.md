The Alert component is for in-context status messaging when the interface needs to tell the user something happened and suggest the next step. A single alert can carry a severity tone, explain the impact, and optionally offer a direct action such as “retry,” “review,” or “dismiss.”

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

This page demonstrates how each piece maps to a common workflow: the outer container sets tone, the icon telegraphs intent, the title announces the state, and the description tells the user what to do next. The demos below show these roles independently so you can compose a notice that is both specific and actionable.

## Components

- `Alert`: The outer container for a message block. It controls layout and tone with `default`, `destructive`, `info`, and `success` variants.
- `AlertIcon`: Leading visual slot for a status symbol that matches the alert’s intent.
- `AlertTitle`: Short, assertive heading that captures the situation in one line.
- `AlertDescription`: Secondary text area for why the alert is shown and what it affects.
- `AlertAction`: Optional trailing region for primary calls-to-action such as links or buttons.
