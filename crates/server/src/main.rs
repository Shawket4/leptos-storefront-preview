use app::server::AppState;
use app::{App, shell};
use axum::{
    Router,
    extract::{Path, Query},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use std::collections::HashMap;
use tower_http::{compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,server=debug,app=debug".into()),
        )
        .init();

    let conf = get_configuration(None).expect("leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let state = AppState::load().await;

    // Media is content-addressed by file name, so it can be cached forever (Cloudflare R2 later).
    let media = Router::new()
        .fallback_service(ServeDir::new(media_dir()))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        ));

    let app = Router::new()
        .route("/__prefs", get(set_prefs))
        .route("/__tenant/{id}", get(set_tenant))
        .nest("/media", media)
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let state = state.clone();
                move || provide_context(state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler_with_context(
            {
                let state = state.clone();
                move || provide_context(state.clone())
            },
            shell,
        ))
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    tracing::info!("storefront listening on http://{addr}");
    tracing::info!("  local:   http://nile.localhost:{0}  http://souq.localhost:{0}", addr.port());
    axum::serve(listener, app.into_make_service()).await.expect("serve");
}

fn media_dir() -> String {
    std::env::var("MEDIA_DIR").unwrap_or_else(|_| "data/media".into())
}

/// Only same-site relative paths, so `next` can't be used as an open redirect.
fn safe_next(next: Option<&String>) -> String {
    match next {
        Some(n) if n.starts_with('/') && !n.starts_with("//") && !n.contains('\\') => n.clone(),
        _ => "/".into(),
    }
}

fn pref_cookie(name: &str, value: &str) -> HeaderValue {
    // No `Secure`: the preview is served over plain http on the tailnet.
    HeaderValue::from_str(&format!(
        "{name}={value}; Path=/; Max-Age=31536000; SameSite=Lax"
    ))
    .expect("cookie header")
}

/// `/__prefs?lang=ar&num=arab&theme=dark&next=/c/coffee` — set preference cookies, then go back.
/// A plain link, so it works without JS; with JS the islands router follows the redirect and
/// patches the page in place (no full reload).
async fn set_prefs(Query(q): Query<HashMap<String, String>>) -> Response {
    let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, safe_next(q.get("next")))]).into_response();
    let headers = res.headers_mut();
    if let Some(v) = q.get("lang").filter(|v| matches!(v.as_str(), "en" | "ar")) {
        headers.append(header::SET_COOKIE, pref_cookie(app::prefs::COOKIE_LANG, v));
    }
    if let Some(v) = q.get("num").filter(|v| matches!(v.as_str(), "latn" | "arab")) {
        headers.append(header::SET_COOKIE, pref_cookie(app::prefs::COOKIE_NUMERALS, v));
    }
    if let Some(v) = q.get("theme").filter(|v| matches!(v.as_str(), "system" | "light" | "dark")) {
        headers.append(header::SET_COOKIE, pref_cookie(app::prefs::COOKIE_THEME, v));
    }
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    res
}

/// `/__tenant/souq?next=/` — pick a tenant on hosts without a tenant subdomain (bare tailnet IP).
async fn set_tenant(Path(id): Path<String>, Query(q): Query<HashMap<String, String>>) -> Response {
    if !app::tenant::TENANT_IDS.contains(&id.as_str()) {
        return StatusCode::NOT_FOUND.into_response();
    }
    let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, safe_next(q.get("next")))]).into_response();
    res.headers_mut()
        .append(header::SET_COOKIE, pref_cookie(app::tenant::COOKIE_TENANT, &id));
    res.headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    res
}
