The Tabs component turns a single page into a focused, sectioned workflow: only one panel is shown at a time, but users can jump between related content without page churn. This page’s demos show how triggers and panels stay tightly coupled by shared `value`, so each tab maps to a distinct state while preserving keyboard-friendly, predictable focus order.

## Component Structure

```rust
// The Tabs component wraps all tab triggers and contents and orders them based on their index.
Tabs {
    // The TabList component contains all the tab triggers
    TabList {
        // The TabTrigger component is used to create a clickable tab button that switches the active tab.
        TabTrigger {
            // The index of the tab trigger, used to determine the focus order of the tabs.
            index: 0,
            // The value of the tab trigger, which must be unique and is used to identify the active tab.
            value: "tab1",
            // The contents of the tab trigger button
            {children}
        }
    }
    // The TabContent component contains the content that is displayed when the corresponding tab is active.
    TabContent {
        // The index of the tab content, used to determine the focus order of the tabs.
        index: 0,
        // The value of the tab content, which must match the value of the corresponding TabTrigger to be displayed.
        value: "tab1",
        // The content of the tab, which is displayed when the tab is active.
        {children}
    }
}
```
