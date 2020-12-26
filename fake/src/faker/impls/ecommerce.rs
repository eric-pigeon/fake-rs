use crate::faker::ecommerce::raw::*;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::SliceRandom;
use rand::Rng;

impl<L: Data> Dummy<ProductName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ProductName<L>, rng: &mut R) -> Self {
        format!(
            "{} {} {}",
            *L::PRODUCT_NAME_ADJECTIVE.choose(rng).unwrap(),
            *L::PRODUCT_NAME_MATERIAL.choose(rng).unwrap(),
            *L::PRODUCT_NAME_PRODUCT.choose(rng).unwrap()
        )
    }
}
