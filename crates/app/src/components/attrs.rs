//! Extra attributes on a component's repeated parts (a menu item, a chip, a row): `data-*`,
//! `aria-*`, `title`, `rel`. The item types collect them with an `.attr(name, value)` builder and
//! the component spreads them onto the element: `<a … {..spread(item.attrs)}>`. (Attributes for
//! a component's root go on the component itself: `<Menu attr:data-router="off" …/>`.)

use leptos::attr::custom::custom_attribute;
use leptos::tachys::html::attribute::any_attribute::{AnyAttribute, IntoAnyAttribute};

/// `(name, value)` pairs, in order.
pub type Attrs = Vec<(&'static str, String)>;

/// The pairs as attributes to spread onto an element.
pub fn spread(attrs: Attrs) -> Vec<AnyAttribute> {
    attrs
        .into_iter()
        .map(|(k, v)| custom_attribute(k, v).into_any_attr())
        .collect()
}
