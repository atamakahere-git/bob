use rand::distributions::DistString;

use crate::{
    mutref::{CatMutRefBuilderBorrowTypes, CatMutRefBuilderOwnedTypes},
    owned::{CatOwnedBuilderBorrowTypes, CatOwnedBuilderOwnedTypes},
};

pub mod mutref;
pub mod owned;

/// Generate random string of given length
pub(crate) fn rand_str_gen(len: usize) -> String {
    rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

/// A cat in my cat collection
#[derive(Debug)]
pub struct Cat {
    /// Cat name, can be same fore two
    name: String,
    /// Cat username, they are modern and live in internet. These are unique
    username: String,
    /// A number assigned to cat for easy reference,
    /// new kittens are not assigned yet so it can be None
    number: Option<i64>,
    /// Cats have friends, like us. They keep their username with them
    friends: Vec<String>,
}

/// Build [Cat] with random data
pub trait RandomBuilder {
    fn random_build() -> Cat;
}

/// Build [Cat] with predetermined data
pub trait DefaultBuilder {
    fn default_build() -> Cat;
}

/// Implement [RandomBuilder] on a struct
macro_rules! impl_random_builder {
    ($($struct_name:ident $(<$($lifetime:tt),*>)?),*) => {
        $(impl RandomBuilder for $struct_name $(<$($lifetime),*>)? {
            fn random_build() -> Cat {
                Self::new()
                    .name(&rand_str_gen(10))
                    .username(&rand_str_gen(10))
                    .number(rand::random())
                    .friend(&rand_str_gen(10))
                    .friend(&rand_str_gen(10))
                    .friend(&rand_str_gen(10))
                    .build()
                    .expect("Unable to build")
            }
        })*
    };
}

/// Implement [DefaultBuilder] on a struct
macro_rules! impl_default_builder {
    ($($struct_name:ident $(<$($lifetime:tt),*>)?),*) => {
        $(impl DefaultBuilder for $struct_name $(<$($lifetime),*>)? {
            fn default_build() -> Cat {
                Self::new()
                    .name("goodkitten")
                    .username("goodkitten")
                    .number(123)
                    .friend("goodkitten1")
                    .friend("goodkitten2")
                    .friend("goodkitten3")
                    .build()
                    .expect("Unable to build")
            }
        })*
    };
}

impl_random_builder!(
    CatOwnedBuilderOwnedTypes,
    CatMutRefBuilderOwnedTypes,
    CatMutRefBuilderBorrowTypes<'_>,
    CatOwnedBuilderBorrowTypes<'_>
);

impl_default_builder!(
    CatOwnedBuilderOwnedTypes,
    CatMutRefBuilderOwnedTypes,
    CatMutRefBuilderBorrowTypes<'_>,
    CatOwnedBuilderBorrowTypes<'_>
);
