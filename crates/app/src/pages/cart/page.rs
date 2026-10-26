//! `/cart` — a server component that works completely without JS.
//!
//! Every control is a stock `<ActionForm>` rendered on the server, i.e. a plain
//! `<form method="post" action="/api/cart_set_qty">`. Without JS the browser posts it, server_fn
//! answers with a redirect back to the Referer, and the page re-renders. With JS the islands
//! router intercepts the same submit, posts it, follows the redirect and diffs the new HTML into
//! the page — no island on this page carries cart state. The markup is shaped for that diff:
//! lines are a keyed `<For>`, anything that can appear or disappear is an `Option`/`Either`
//! (branch markers), and changing attributes are always present with a value (the diff sets
//! attributes, it never removes them).

use super::copy;
use crate::api::cart::{CartAdd, CartRemove, CartSetQty, cart_get};
use crate::api::types::{CartLineView, CartView, Rail};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::components::price::{Price, PriceSize};
use crate::components::product_card::CardData;
use crate::components::skeleton::Skeleton;
use crate::components::states::{SectionError, StateTone, StateView};
use crate::format;
use crate::media::Img;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use domain::Piastres;
use leptos::either::Either;
use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos::server_fn::error::ServerFnUrlError;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_query_map;

/// Art for empty states, resolved on the server (light and dark files, if the brand agent made
/// them).
#[derive(Clone, Default)]
pub struct Art {
    pub light: Option<Img>,
    pub dark: Option<Img>,
}

pub fn art(name: &str) -> Art {
    #[cfg(feature = "ssr")]
    {
        let m = crate::server::state();
        Art {
            light: m.media().try_img(&format!("brand/{name}-light")),
            dark: m.media().try_img(&format!("brand/{name}-dark")),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = name;
        Art::default()
    }
}

/// An illustration that follows the theme (both files are lazy, the hidden one never loads).
#[component]
pub fn ArtImage(
    art: Art,
    #[prop(optional)] fallback_icon: Option<&'static str>,
    lang: Lang,
) -> impl IntoView {
    match (art.light, art.dark) {
        (Some(light), dark) => Either::Left(view! {
            <span class="art" aria-hidden="true">
                <img class="art__light" src=light.src_near(240) srcset=light.srcset() sizes="160px" alt=""
                    width="240" height="240" loading="lazy" decoding="async"/>
                {dark.map(|d| view! {
                    <img class="art__dark" src=d.src_near(240) srcset=d.srcset() sizes="160px" alt=""
                        width="240" height="240" loading="lazy" decoding="async"/>
                })}
            </span>
        }),
        (None, _) => {
            let _ = lang;
            Either::Right(view! {
                <span class="art art--icon" aria-hidden="true"><Icon name=fallback_icon.unwrap_or("shopping-bag") size=40/></span>
            })
        }
    }
}

/// An error a no-JS form post brought back in the URL (`?__path=…&__err=…`, server_fn's own
/// encoding), as copy.
fn url_error(lang: Lang) -> Option<String> {
    let q = use_query_map();
    // Repeated failed posts each append a pair; the newest is last.
    let err = q
        .with_untracked(|q| q.get_all("__err"))
        .and_then(|all| all.last().cloned())?;
    let e = ServerFnUrlError::<ServerFnError>::decode_err(&err);
    Some(copy::message(&copy::code(&e), lang))
}

/// What an empty cart offers: a few things to start with.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Suggestions {
    pub picks: Vec<CardData>,
}

/// Quick-addable products (one variant, nothing required, in stock): bestsellers, then featured.
pub fn pick_quick_adds(
    pool: Vec<CardData>,
    exclude: &[domain::ProductId],
    n: usize,
) -> Vec<CardData> {
    let mut seen = Vec::new();
    pool.into_iter()
        .filter(|c| c.in_stock && c.single_variant && !exclude.contains(&c.id))
        .filter(|c| {
            !seen.contains(&c.id) && {
                seen.push(c.id);
                true
            }
        })
        .take(n)
        .collect()
}

/// For the drawer's empty state (fetched once, when the drawer first shows an empty cart).
#[server]
pub async fn cart_suggestions() -> Result<Suggestions, ServerFnError> {
    crate::request::init();
    crate::server::cache::private();
    let mut pool = crate::api::home_rail(Rail::Bestsellers, 24)
        .await
        .unwrap_or_default();
    pool.extend(
        crate::api::home_rail(Rail::Featured, 24)
            .await
            .unwrap_or_default(),
    );
    Ok(Suggestions {
        picks: pick_quick_adds(pool, &[], 3),
    })
}

#[component]
pub fn CartPage() -> impl IntoView {
    // Personal: never cached anywhere. Set before the first chunk streams.
    #[cfg(feature = "ssr")]
    crate::server::cache::private();
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let cart = Resource::new(|| (), |_| cart_get());
    // Quick-addable suggestions: bestsellers first, then featured (one variant, nothing required).
    let picks = Resource::new(
        || (),
        |_| async {
            let mut pool = crate::api::home_rail(Rail::Bestsellers, 24)
                .await
                .unwrap_or_default();
            pool.extend(
                crate::api::home_rail(Rail::Featured, 24)
                    .await
                    .unwrap_or_default(),
            );
            Ok::<_, ServerFnError>(pool)
        },
    );
    let set_qty = ServerAction::<CartSetQty>::new();
    let remove = ServerAction::<CartRemove>::new();
    let add = ServerAction::<CartAdd>::new();
    let notice = url_error(lang);
    let title = format!(
        "{} · {}",
        tr(lang, "Your cart", "سلتك"),
        rq.tenant.name.pick(lang.is_rtl())
    );

    view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <div class="container cart-page">
            <header class="cart-page__head">
                <h1 class="t-h1" id="cart-title" tabindex="-1" data-focus-fallback="">{tr(lang, "Your cart", "سلتك")}</h1>
            </header>
            {notice.map(|n| view! {
                <p class="notice notice--error" role="alert" tabindex="-1" data-autofocus="">
                    <Icon name="circle-alert" size=20/><span>{n}</span>
                </p>
            })}
            <Suspense fallback=move || view! { <CartPageSkeleton/> }>
                <ErrorBoundary fallback=move |errors| view! {
                    <SectionError errors what=tr(lang, "your cart", "السلة").to_owned() retry_href="/cart"/>
                }>
                    {move || Suspend::new(async move {
                        let v = cart.await?;
                        let picks = picks.await.unwrap_or_default();
                        Ok::<_, ServerFnError>(view! { <CartBody v picks lang num set_qty remove add/> })
                    })}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn CartPageSkeleton() -> impl IntoView {
    view! {
        <div class="cart-layout" aria-hidden="true">
            <ul class="clines card">
                {(0..2).map(|_| view! {
                    <li class="cline cline--page cline--skeleton">
                        <span class="cline__media"><Skeleton class="cline__skel-media"/></span>
                        <span class="cline__main">
                            <Skeleton width="60%" height="1.1em"/>
                            <Skeleton width="35%" height="0.9em"/>
                            <Skeleton width="25%" height="1em"/>
                        </span>
                    </li>
                }).collect_view()}
            </ul>
            <div class="cart-summary card">
                <Skeleton width="100%" height="3.25rem"/>
                <Skeleton width="100%" height="8rem"/>
                <Skeleton width="100%" height="3.25rem"/>
            </div>
        </div>
    }
}

#[component]
fn CartBody(
    v: CartView,
    picks: Vec<CardData>,
    lang: Lang,
    num: Numerals,
    set_qty: ServerAction<CartSetQty>,
    remove: ServerAction<CartRemove>,
    add: ServerAction<CartAdd>,
) -> impl IntoView {
    let in_cart: Vec<_> = v.lines.iter().map(|l| l.product).collect();
    let picks = pick_quick_adds(picks, &in_cart, 4);
    let empty = v.lines.is_empty();
    let count = v.count;
    let body = if empty {
        Either::Left(view! {
            <div class="card cart-empty--page">
                <StateView tone=StateTone::Empty art="empty-cart"
                    title=tr(lang, "Your cart is empty", "سلتك فاضية")
                    body=tr(lang, "Add something you like — it'll wait for you here.", "ضيف اللي يعجبك، وهيستناك هنا.")>
                    <Button href="/" kind=ButtonKind::Primary size=ButtonSize::Lg>{tr(lang, "Start shopping", "ابدأ التسوق")}</Button>
                </StateView>
            </div>
        })
    } else {
        let lines = v.lines.clone();
        Either::Right(view! {
            <div class="cart-layout">
                <section class="cart-lines card" aria-labelledby="cart-title">
                    <p class="cart-lines__count">{format::items(count, lang, num)}</p>
                    <ul class="clines">
                        <For each=move || lines.clone() key=|l| l.key.clone() let:line>
                            <PageLine line lang num set_qty remove/>
                        </For>
                    </ul>
                </section>
                <PageSummary v=v.clone() lang num/>
            </div>
        })
    };
    view! {
        {body}
        {(!picks.is_empty()).then(|| view! { <Picks picks lang num add/> })}
    }
}

#[component]
fn PageLine(
    line: CartLineView,
    lang: Lang,
    num: Numerals,
    set_qty: ServerAction<CartSetQty>,
    remove: ServerAction<CartRemove>,
) -> impl IntoView {
    let ar = lang.is_rtl();
    let key = line.key.clone();
    let q = line.qty;
    let href = format!("/p/{}", line.slug);
    let name = line.name.pick(ar).to_owned();
    let detail = line.variant_label.pick(ar).to_owned();
    let addons = line
        .addons
        .iter()
        .map(|a| a.pick(ar).to_owned())
        .collect::<Vec<_>>()
        .join(if ar { "، " } else { ", " });
    // "2 × EGP 1,899.00" under the line total explains it.
    let breakdown = (q > 1).then(|| {
        format!(
            "{} × {}",
            format::count(q, num),
            format::money(line.unit_price, lang, num)
        )
    });
    let dec_label = format!(
        "{}: {name}",
        if q <= 1 {
            tr(lang, "Remove", "شيل")
        } else {
            tr(lang, "One less", "قلّل واحد")
        }
    );
    let inc_label = format!("{}: {name}", tr(lang, "One more", "زوّد واحد"));
    let stock = super::math::stock(&line);
    let low =
        (stock == super::math::Stock::Ok && line.max_qty > 0 && line.max_qty <= 5).then(|| {
            match lang {
                Lang::En => format!("Only {} in stock", format::count(line.max_qty, num)),
                Lang::Ar => format!("المتاح {} بس", format::count(line.max_qty, num)),
            }
        });
    // Stock moved after this was added: explain, and offer the fix right here.
    let issue = super::math::stock_message(stock, lang, num).map(|msg| {
        let fix = match stock {
            super::math::Stock::Only(n) => Some((key.clone(), n)),
            _ => None,
        };
        (msg, fix)
    });
    let inc = if q >= line.max_qty {
        Either::Left(view! {
            <button type="button" class="qty__btn" disabled="" data-focus-id=format!("{key}:inc") aria-label=inc_label.clone()>
                <Icon name="plus" size=18/>
            </button>
        })
    } else {
        Either::Right(view! {
            <button type="submit" name="qty" value=(q + 1).to_string() class="qty__btn" data-focus-id=format!("{key}:inc") aria-label=inc_label.clone()>
                <Icon name="plus" size=18/>
            </button>
        })
    };
    let (key_qty, key_rm) = (key.clone(), key.clone());
    let (dec_id, rm_id) = (format!("{key}:dec"), format!("{key}:rm"));
    let rm_label = format!("{}: {name}", tr(lang, "Remove", "شيل"));
    view! {
        <li class="cline cline--page">
            <a class="cline__media" href=href.clone() tabindex="-1" aria-hidden="true">
                <Picture img=line.img.clone() sizes="96px" lang/>
            </a>
            <div class="cline__main">
                <a class="cline__name" href=href>{name.clone()}</a>
                {(!detail.is_empty()).then(|| view! { <p class="cline__detail">{detail}</p> })}
                {(!addons.is_empty()).then(|| view! { <p class="cline__addons"><Icon name="plus" size=12/><span>{addons}</span></p> })}
                <div class="cline__unit">
                    <Price amount=line.unit_price compare_at=line.compare_at size=PriceSize::Sm no_badge=true lang numerals=num/>
                </div>
                {low.map(|t| view! { <p class="cline__stock">{t}</p> })}
                {issue.map(|(msg, fix)| view! {
                    <div class="line-issue" role="status">
                        <Icon name="triangle-alert" size=16/>
                        <span>{msg}</span>
                        {fix.map(|(k, n)| view! {
                            <ActionForm action=set_qty attr:class="line-issue__fix" attr:data-pending-ui="" attr:data-keep-scroll="" attr:aria-busy="false">
                                <input type="hidden" name="key" value=k/>
                                <button type="submit" name="qty" value=n.to_string() class="link-btn">
                                    {match lang {
                                        Lang::En => format!("Change to {}", format::count(n, num)),
                                        Lang::Ar => format!("خليها {}", format::count(n, num)),
                                    }}
                                </button>
                            </ActionForm>
                        })}
                    </div>
                })}
                <ActionForm action=remove attr:class="cline__remove" attr:data-pending-ui="" attr:data-keep-scroll="" attr:aria-busy="false">
                    <input type="hidden" name="key" value=key_rm/>
                    <button type="submit" class="link-btn" data-focus-id=rm_id aria-label=rm_label>
                        {tr(lang, "Remove", "شيل")}
                    </button>
                </ActionForm>
            </div>
            <div class="cline__side">
                <ActionForm action=set_qty attr:class="qty" attr:data-pending-ui="" attr:data-keep-scroll="" attr:aria-busy="false">
                    <input type="hidden" name="key" value=key_qty/>
                    <button type="submit" name="qty" value=q.saturating_sub(1).to_string() class="qty__btn"
                        data-focus-id=dec_id aria-label=dec_label>
                        <Icon name=if q <= 1 { "trash-2" } else { "minus" } size=18/>
                    </button>
                    <output class="qty__val mono">{format::count(q, num)}</output>
                    {inc}
                </ActionForm>
                <span class="cline__total">
                    <Price amount=line.line_total size=PriceSize::Sm no_badge=true lang numerals=num/>
                    {breakdown.map(|b| view! { <span class="cline__breakdown">{b}</span> })}
                </span>
            </div>
        </li>
    }
}

/// The free-delivery meter, server-rendered (same markup as the drawer's live one).
#[component]
pub fn ShipMeter(v: CartView, lang: Lang, num: Numerals) -> impl IntoView {
    let left = (v.free_delivery_over.0 - v.subtotal.0).max(0);
    let p = super::math::free_delivery_progress(&v);
    let text = if left == 0 {
        tr(lang, "Delivery is on us.", "التوصيل علينا.").to_owned()
    } else {
        let m = format::money(Piastres(left), lang, num);
        match lang {
            Lang::En => format!("Add {m} more for free delivery."),
            Lang::Ar => format!("ضيف بـ {m} كمان والتوصيل يبقى مجاني."),
        }
    };
    view! {
        <div class="ship" data-done=(left == 0).to_string()>
            <p class="ship__text"><Icon name="truck" size=18/><span>{text.clone()}</span></p>
            <div class="ship__bar" role="progressbar" aria-valuemin="0" aria-valuemax="100"
                aria-valuenow=((p * 100.0).round() as u32).to_string() aria-valuetext=text>
                <span class="ship__fill" style=format!("--p:{p:.4}")></span>
            </div>
        </div>
    }
}

/// Totals as a definition list (shared by /cart and the checkout summary).
#[component]
pub fn Totals(
    v: CartView,
    lang: Lang,
    num: Numerals,
    /// An extra delivery charge (the express slot) on top of the cart's own fee.
    #[prop(optional)]
    extra_fee: Piastres,
) -> impl IntoView {
    let fee = v.delivery_fee + extra_fee;
    let total = v.total + extra_fee;
    let vat = match lang {
        Lang::En => format!(
            "Prices include 14% VAT ({}).",
            format::money(v.tax_included, lang, num)
        ),
        Lang::Ar => format!(
            "الأسعار شاملة ضريبة القيمة المضافة {}٪ ({}).",
            format::count(14, num),
            format::money(v.tax_included, lang, num)
        ),
    };
    view! {
        <dl class="totals">
            <div class="totals__row">
                <dt>{tr(lang, "Subtotal", "المجموع")}</dt>
                <dd><Price amount=v.subtotal size=PriceSize::Sm no_badge=true lang numerals=num/></dd>
            </div>
            {(v.savings.0 > 0).then(|| view! {
                <div class="totals__row totals__row--save">
                    <dt>{tr(lang, "You save", "وفّرت")}</dt>
                    <dd><Price amount=v.savings size=PriceSize::Sm no_badge=true lang numerals=num/></dd>
                </div>
            })}
            <div class="totals__row">
                <dt>{tr(lang, "Delivery", "التوصيل")}</dt>
                <dd>{if fee.0 == 0 {
                    Either::Left(view! { <span class="totals__free">{tr(lang, "Free", "مجاني")}</span> })
                } else {
                    Either::Right(view! { <Price amount=fee size=PriceSize::Sm no_badge=true lang numerals=num/> })
                }}</dd>
            </div>
            <div class="totals__row totals__row--total">
                <dt>{tr(lang, "Total", "الإجمالي")}</dt>
                <dd><Price amount=total size=PriceSize::Lg no_badge=true lang numerals=num/></dd>
            </div>
        </dl>
        <p class="totals__vat">{vat}</p>
    }
}

#[component]
fn PageSummary(v: CartView, lang: Lang, num: Numerals) -> impl IntoView {
    let blocked = super::math::stock_blocked(&v);
    view! {
        <aside class="cart-summary card" aria-label=tr(lang, "Order summary", "ملخص الطلب")>
            <ShipMeter v=v.clone() lang num/>
            <Totals v lang num/>
            <div class="cart-summary__actions">
                {if blocked {
                    Either::Left(view! {
                        <p class="cart-summary__blocked" id="checkout-blocked">
                            {tr(lang, "Fix the items marked above to check out.", "عدّل المنتجات المحددة فوق علشان تكمّل الطلب.")}
                        </p>
                        <Button kind=ButtonKind::Primary size=ButtonSize::Lg full_width=true disabled=true
                            attr:aria-describedby="checkout-blocked">
                            {tr(lang, "Checkout", "كمّل الطلب")}
                        </Button>
                    })
                } else {
                    Either::Right(view! {
                        <Button href="/checkout" kind=ButtonKind::Primary size=ButtonSize::Lg full_width=true icon_end="arrow-right">
                            {tr(lang, "Checkout", "كمّل الطلب")}
                        </Button>
                    })
                }}
                <Button href="/" kind=ButtonKind::Ghost full_width=true>{tr(lang, "Continue shopping", "كمّل تسوق")}</Button>
            </div>
            <ul class="assure" role="list">
                <li><Icon name="banknote" size=18/><span>{tr(lang, "Pay cash on delivery", "ادفع كاش عند الاستلام")}</span></li>
                <li><Icon name="shield-check" size=18/><span>{tr(lang, "Prices are checked again when you order", "الأسعار بتتراجع تاني وقت الطلب")}</span></li>
            </ul>
        </aside>
    }
}

/// "Popular right now": a few bestsellers not in the cart, each with a plain add form.
#[component]
fn Picks(
    picks: Vec<CardData>,
    lang: Lang,
    num: Numerals,
    add: ServerAction<CartAdd>,
) -> impl IntoView {
    let ar = lang.is_rtl();
    view! {
        <section class="picks" aria-labelledby="picks-title">
            <h2 class="t-h3" id="picks-title">{tr(lang, "Popular right now", "الأكثر طلبًا دلوقتي")}</h2>
            <ul class="picks__list" role="list">
                <For each=move || picks.clone() key=|c| c.id let:c>
                    <li class="pick card">
                        <a class="pick__media" href=format!("/p/{}", c.slug) tabindex="-1" aria-hidden="true">
                            <Picture img=c.img.clone() sizes="88px" lang/>
                        </a>
                        <div class="pick__main">
                            <a class="pick__name" href=format!("/p/{}", c.slug)>{c.name.pick(ar).to_owned()}</a>
                            <Price amount=c.price compare_at=c.compare_at size=PriceSize::Sm lang numerals=num/>
                        </div>
                        <ActionForm action=add attr:class="pick__add" attr:data-pending-ui="" attr:data-keep-scroll="" attr:aria-busy="false">
                            <input type="hidden" name="product" value=c.id.0/>
                            <input type="hidden" name="variant" value=c.default_variant.0/>
                            <button type="submit" class="icon-btn icon-btn--outline" data-focus-id=format!("pick:{}", c.id.0)
                                aria-label=format!("{}: {}", tr(lang, "Add to cart", "أضف للسلة"), c.name.pick(ar))>
                                <Icon name="plus" size=20/>
                            </button>
                        </ActionForm>
                    </li>
                </For>
            </ul>
        </section>
    }
}
