//! The design system: primitives and composite components shared by every page.
//! Owned by the design-system agent (see AGENTS.md). Styles live in `style/components/*.css`.

pub mod assets;
pub mod attrs;
pub mod badge;
pub mod bottom_nav;
pub mod breadcrumbs;
pub mod button;
pub mod choice_chips;
pub mod choice_list;
pub mod drawer;
pub mod empty_state;
pub mod facet_group;
pub mod filter_chips;
pub mod footer;
pub mod form;
pub mod header;
pub mod icons;
pub mod image;
pub mod layout;
pub mod link_chips;
pub mod menu;
pub mod notice;
pub mod offline;
pub mod pager;
pub mod prefs_controls;
pub mod price;
pub mod product_card;
pub mod rating;
pub mod segmented;
pub mod shell;
pub mod skeleton;
pub mod states;
pub mod status_pill;
pub mod stepper;
pub mod toast;
// Shared components from the cart's patterns (core agent): totals, item rows, key-value lists,
// the step indicator, the timeline and the live status line.
pub mod item_row;
pub mod kv;
pub mod live_status;
pub mod steps;
pub mod timeline;
pub mod totals;
