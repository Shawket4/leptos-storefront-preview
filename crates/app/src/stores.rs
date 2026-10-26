//! Global stores shared by every island. Provided per request on the server (inside
//! `request::init`) and once on the client (inside `client::init`), so `use_context` resolves the
//! same type in SSR and after hydration. Owners implement the `provide` functions in their files.

pub fn provide_all() {
    crate::components::toast::provide();
    crate::pages::cart::provide_store();
}
