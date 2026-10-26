//! Route components. One module per page; each is owned by one build agent (see AGENTS.md).

pub mod cart;
pub mod category;
pub mod checkout;
pub mod errors;
pub mod home;
pub mod order;
pub mod product;
pub mod search;
pub mod showcase;
pub mod ui_kit;

pub use cart::CartPage;
pub use category::CategoryPage;
pub use checkout::CheckoutPage;
pub use errors::NotFound;
pub use home::HomePage;
pub use order::OrderPage;
pub use product::ProductPage;
pub use search::SearchPage;
pub use showcase::ShowcasePage;
pub use ui_kit::UiKitPage;
