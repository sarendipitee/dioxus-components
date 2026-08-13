# Menu grace triangle refactor

1. [x] Add Playwright coverage before changing implementation:
   - right-side trigger-to-submenu travel;
   - delayed travel through the gap;
   - left-side collision-flipped travel;
   - nested submenu gap travel;
   - outside dismissal after grace area exit.
2. [x] Run targeted menu tests and record/fix existing gap failure.
3. [x] Move grace triangle into each portaled submenu content node with static CSS/DOM geometry.
4. [x] Remove dynamic guard creation, rectangle measurement, and trajectory math from `primitives/src/menu.rs`.
5. [x] Run targeted tests, repository validation, and independent review.
