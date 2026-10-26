//! Brand assets: tenant lockups and Madar's mark, looked up in the media manifests on the server.
//!
//! Lockups (`brand/<tenant>-lockup-<en|ar>[-dark]`) are framed to the same height relative to
//! their mark, so one CSS height gives the same mark size for every tenant and language.

use crate::media::Img;
use crate::prefs::{Lang, Theme};
use crate::tenant::Tenant;
use leptos::prelude::*;

/// A media key's image, if the manifests have it. Server only (`None` in the browser and when no
/// app state is around, e.g. while the route list is generated).
pub fn media_img(key: &str) -> Option<Img> {
    #[cfg(feature = "ssr")]
    {
        use_context::<crate::server::AppState>().and_then(|s| s.media().try_img(key))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = key;
        None
    }
}

/// `<img>` attributes for a logo-ladder asset at a CSS height (1x/2x/3x from the ladder).
fn logo_img(img: &Img, css_h: u16, alt: String, class: &'static str) -> impl IntoView + use<> {
    let w = (css_h as f32 * img.aspect).round() as u32;
    let srcset = img.srcset();
    let sizes = format!("{w}px");
    let src = img.src_near((w * 2).min(512) as u16);
    view! {
        <img
            class=class
            src=src
            srcset=srcset
            sizes=sizes
            width=w
            height=css_h
            alt=alt
            decoding="async"
        />
    }
}

/// The tenant's lockup for the language; light and dark variants swap with the theme (only the
/// one the theme needs is rendered unless the theme follows the device). Falls back to a typeset
/// wordmark when the asset isn't there.
#[component]
pub fn TenantLogo(
    tenant: Tenant,
    lang: Lang,
    theme: Theme,
    /// CSS height in px.
    #[prop(optional)]
    height: Option<u16>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let h = height.unwrap_or(32);
    let name = tenant.name.pick(lang.is_rtl()).to_owned();
    let key = format!("brand/{}-lockup-{}", tenant.id, lang.code());
    let light = media_img(&key);
    let dark = media_img(&format!("{key}-dark")).or_else(|| light.clone());
    let cls = if class.is_empty() {
        "logo".to_owned()
    } else {
        format!("logo {class}")
    };
    let style = format!("--logo-h:{h}px");
    if light.is_none() {
        return view! {
            <span class=format!("{cls} logo--wordmark") style=style>{name}</span>
        }
        .into_any();
    }
    let want_light = theme != Theme::Dark;
    let want_dark = theme != Theme::Light;
    view! {
        <span class=cls style=style>
            {light.filter(|_| want_light).map(|i| logo_img(&i, h, name.clone(), "logo__img logo__img--light"))}
            {dark.filter(|_| want_dark).map(|i| logo_img(&i, h, name.clone(), "logo__img logo__img--dark"))}
        </span>
    }
    .into_any()
}

/// "Powered by Madar" — the platform's signature at the end of every page: Madar's mark (its own
/// colours, in both themes) and the words, linking to Madar's site in a new tab.
#[component]
pub fn MadarBadge(lang: Lang, tenant_id: String) -> impl IntoView {
    let href = format!(
        "https://get.madar-pos.cloud/{}/?utm_source={}&utm_medium=powered_by&utm_campaign=storefront",
        lang.code(),
        tenant_id
    );
    let mark = media_img("brand/madar-mark");
    let mark_dark = media_img("brand/madar-mark-dark").or_else(|| mark.clone());
    let (by, name, new_tab) = match lang {
        Lang::En => ("Powered by", "Madar", "(opens in a new tab)"),
        Lang::Ar => ("مدعوم من", "مدار", "(يفتح في تبويب جديد)"),
    };
    view! {
        // The badge is an inverted (ink) surface: on a light page it is dark, so it wears the
        // mark drawn for dark grounds, and the other way round.
        <a class="madar-badge" href=href target="_blank" rel="noopener" data-press="">
            {mark_dark.map(|i| logo_img(&i, 22, String::new(), "madar-badge__mark logo__img--light"))}
            {mark.map(|i| logo_img(&i, 22, String::new(), "madar-badge__mark logo__img--dark"))}
            <span class="madar-badge__text">
                <span class="madar-badge__by">{by}</span>
                <span class="madar-badge__name">{name}</span>
            </span>
            <super::icons::Icon name="arrow-up-right" size=16 class="madar-badge__go flip-rtl"/>
            <span class="sr-only">{new_tab}</span>
        </a>
    }
}
