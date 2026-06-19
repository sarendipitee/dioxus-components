## Component Purpose

The Table of Contents component turns a page’s heading structure into a live, navigable outline. It watches scroll position and marks the section currently nearest to the target viewport offset, so users can see exactly where they are while moving through long content.

## Usage

Pass a heading selector through `scroll_spy_options` to generate links directly from rendered headings. The demos wire this component to real article markup so each heading becomes a navigable control with matching `data-depth` metadata.

Use `initial_data` when the markup must be visible during SSR before heading discovery runs in the browser.

After dynamic content changes (for example, headings loading after an async data fetch), call the hook state's `reinitialize` callback so the table stays aligned with the latest heading list.

## Styling

The root element has `data-table-of-contents="true"`. Each control receives `data-depth`, and the active control receives `data-active="true"`.

## Demo Notes

In this preview, the demos are not generic style examples—they show scroll-reactive behavior. The main demo demonstrates a long document with nested section levels, while the reinitialize demo shows how to refresh the registry when the heading set changes at runtime.
