//! Prices and tax through Madar's own rules (madar-shared, used by path).
//!
//! A storefront variant maps onto a Madar *size* (absolute price) and a modifier group onto a
//! Madar choice group whose options add a surcharge, so `madar_catalog::price_line` prices a cart
//! line exactly as Madar's server would. Tax-inclusive VAT comes from `madar_money::tax::compute`.

use domain::{Piastres, Product, Variant};
use madar_catalog::{CatalogView, GroupOption, GroupView, ItemView, OptionView, Pick, Selection, SizeView};
use rust_decimal::Decimal;

/// Egypt's VAT, prices tax-inclusive (Madar `OnlineTaxPolicy { tax_rate: 0.14, tax_inclusive: true }`).
fn policy() -> madar_money::tax::TaxPolicy {
    madar_money::tax::TaxPolicy {
        tax_rate: Decimal::new(14, 2),
        tax_inclusive: true,
        service_charge_rate: Decimal::ZERO,
        service_charge_taxable: false,
    }
}

/// The product as Madar's catalogue view: one size per variant (label = variant id).
pub fn catalog_view(p: &Product) -> CatalogView {
    let item = ItemView {
        id: p.id.0.to_string(),
        sizes: p
            .variants
            .iter()
            .map(|v| SizeView {
                label: v.id.0.to_string(),
                price: Some(v.price.0),
                is_active: true,
                branch_price: None,
            })
            .collect(),
        groups: p
            .modifier_groups
            .iter()
            .map(|g| GroupView {
                id: g.id.clone(),
                min: g.min as i64,
                is_required: g.required,
                effect: "adds".into(),
                options: g
                    .options
                    .iter()
                    .map(|o| GroupOption { id: o.id.clone(), price: o.price.0, is_active: true, ..Default::default() })
                    .collect(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };
    let options = p
        .modifier_groups
        .iter()
        .flat_map(|g| {
            g.options.iter().map(move |o| OptionView {
                id: o.id.clone(),
                name: o.name.en.clone(),
                kind: "extra".into(),
                price: o.price.0,
                group_id: Some(g.id.clone()),
                effect: Some("adds".into()),
                ..Default::default()
            })
        })
        .collect();
    CatalogView { item, options }
}

/// One unit of `variant` with the picked add-ons, priced by `madar_catalog::price_line`.
pub fn unit_price(p: &Product, v: &Variant, addons: &[String]) -> Result<Piastres, String> {
    let selection = Selection {
        size_label: Some(v.id.0.to_string()),
        options: addons.iter().map(|id| Pick { id: id.clone(), quantity: 1 }).collect(),
        optionals: vec![],
    };
    madar_catalog::price_line(&catalog_view(p), &selection)
        .map(|line| Piastres(line.per_unit()))
        .map_err(|e| format!("{e:?}"))
}

/// The VAT contained in a tax-inclusive amount.
pub fn vat_included(amount: Piastres) -> Piastres {
    Piastres(madar_money::tax::compute(amount.0, 0, &policy()).tax)
}
