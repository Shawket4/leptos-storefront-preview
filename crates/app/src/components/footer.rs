//! The site footer: the shop's promise, link columns, the display preferences in the open, a
//! switch to the other shop on the platform, and "Powered by Madar".

use super::assets::{MadarBadge, TenantLogo};
use super::bottom_nav::{ACCOUNT_HREF, DEALS_HREF};
use super::header::NavItem;
use super::icons::Icon;
use super::prefs_controls::{PrefsControls, PrefsVariant};
use crate::format;
use crate::prefs::{Lang, Numerals, Theme, tr};
use crate::request::RequestCtx;
use crate::tenant::{TenantVia, swap_tenant_label};
use domain::L10n;
use leptos::prelude::*;

/// The other shop on the platform, as (name, tagline, href).
fn other_shop(rq: &RequestCtx) -> Option<(L10n, L10n, String)> {
    #[cfg(feature = "ssr")]
    {
        let state = use_context::<crate::server::AppState>()?;
        let other = state
            .tenants()
            .iter()
            .find(|t| t.id != rq.tenant.id)?
            .clone();
        let href = match rq.via {
            TenantVia::Subdomain => {
                let scheme = rq
                    .origin
                    .split_once("://")
                    .map(|(s, _)| s)
                    .unwrap_or("http");
                swap_tenant_label(&rq.host, &other.id)
                    .map(|h| format!("{scheme}://{h}/"))
                    .unwrap_or_else(|| format!("/__tenant/{}?next=/", other.id))
            }
            _ => format!("/__tenant/{}?next=/", other.id),
        };
        Some((other.name, other.tagline, href))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (rq, TenantVia::Default, swap_tenant_label);
        None
    }
}

fn this_year() -> u32 {
    #[cfg(feature = "ssr")]
    {
        let ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        format::civil_from_days(ms.div_euclid(86_400_000)).0 as u32
    }
    #[cfg(not(feature = "ssr"))]
    {
        2026
    }
}

#[component]
pub fn SiteFooter(rq: RequestCtx, categories: Vec<NavItem>) -> impl IntoView {
    let lang: Lang = rq.prefs.lang;
    let numerals: Numerals = rq.prefs.numerals;
    let theme: Theme = rq.prefs.theme;
    let ar = lang.is_rtl();
    let tenant = rq.tenant.clone();
    let tenant_name = tenant.name.pick(ar).to_owned();
    let tagline = tenant.tagline.pick(ar).to_owned();
    let other = other_shop(&rq);

    let shop_links = categories
        .iter()
        .take(6)
        .map(|c| (format!("/c/{}", c.slug), c.name.pick(ar).to_owned()))
        .chain([(
            DEALS_HREF.to_owned(),
            tr(lang, "Today's deals", "عروض النهارده").to_owned(),
        )])
        .collect::<Vec<_>>();
    let help_links = [
        ("/cart", tr(lang, "Your cart", "سلتك")),
        ("/checkout", tr(lang, "Checkout", "إتمام الطلب")),
        (ACCOUNT_HREF, tr(lang, "Track an order", "تتبع طلبك")),
        ("/search", tr(lang, "Search", "البحث")),
    ];
    let about_links = [
        (
            "/showcase",
            tr(lang, "How this store works", "إزاي المتجر ده شغال"),
        ),
        ("/ui", tr(lang, "Design system", "نظام التصميم")),
        ("/nojs", tr(lang, "Without JavaScript", "من غير جافاسكريبت")),
    ];
    let fourteen = format::count(14, numerals);
    let promises = [
        (
            "truck",
            tr(lang, "Delivery across Egypt", "توصيل لكل مصر").to_owned(),
        ),
        (
            "banknote",
            tr(lang, "Cash on delivery", "الدفع عند الاستلام").to_owned(),
        ),
        (
            "rotate-ccw",
            match lang {
                Lang::En => format!("{fourteen}-day returns"),
                Lang::Ar => format!("استرجاع خلال {fourteen} يوم"),
            },
        ),
    ];
    let year = format::count(this_year(), numerals);
    let legal = match lang {
        Lang::En => format!("© {year} {tenant_name}. Prices include VAT."),
        Lang::Ar => format!("© {year} {tenant_name}. الأسعار شاملة الضريبة."),
    };
    let col = |title: &'static str, links: Vec<(String, String)>| {
        view! {
            <div class="site-footer__col">
                <h2 class="site-footer__heading">{title}</h2>
                <ul class="site-footer__links" role="list">
                    {links
                        .into_iter()
                        .map(|(href, label)| view! { <li><a href=href>{label}</a></li> })
                        .collect_view()}
                </ul>
            </div>
        }
    };
    let to_owned = |v: &[(&str, &str)]| {
        v.iter()
            .map(|(h, l)| (h.to_string(), l.to_string()))
            .collect::<Vec<_>>()
    };

    view! {
        <footer class="site-footer" data-region="footer">
            <div class="container site-footer__top">
                <div class="site-footer__brand">
                    <a class="site-footer__logo" href="/" aria-label=tenant_name.clone()>
                        <TenantLogo tenant=tenant.clone() lang=lang theme=theme height=28/>
                    </a>
                    <p class="site-footer__tagline">{tagline}</p>
                    <ul class="site-footer__promises" role="list">
                        {promises
                            .into_iter()
                            .map(|(icon, text)| view! { <li><Icon name=icon size=18/><span>{text}</span></li> })
                            .collect_view()}
                    </ul>
                </div>
                <nav class="site-footer__cols" aria-label=tr(lang, "Footer", "روابط")>
                    {col(tr(lang, "Shop", "تسوّق"), shop_links)}
                    {col(tr(lang, "Help", "مساعدة"), to_owned(&help_links))}
                    {col(tr(lang, "This preview", "عن المعاينة"), to_owned(&about_links))}
                </nav>
            </div>
            <div class="container site-footer__middle">
                <PrefsControls variant=PrefsVariant::Inline/>
                {other.map(|(name, tagline, href)| {
                    let label = tr(lang, "Also on this platform", "كمان على نفس المنصة");
                    view! {
                        <a class="tenant-switch" href=href data-press="">
                            <span class="tenant-switch__kicker">{label}</span>
                            <span class="tenant-switch__name">{name.pick(ar).to_owned()}</span>
                            <span class="tenant-switch__tagline">{tagline.pick(ar).to_owned()}</span>
                            <Icon name="arrow-right" size=18 class="tenant-switch__go flip-rtl"/>
                        </a>
                    }
                })}
            </div>
            <div class="container site-footer__base">
                <p class="site-footer__legal">{legal}</p>
                <MadarBadge lang=lang tenant_id=tenant.id.clone()/>
            </div>
        </footer>
    }
}
