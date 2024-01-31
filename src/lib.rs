use crate::{
    mutref::{CatMutRefBuilderBorrowTypes, CatMutRefBuilderOwnedTypes},
    owned::{CatOwnedBuilderBorrowTypes, CatOwnedBuilderOwnedTypes},
};

mod mutref;
mod owned;

/// A cat in my cat collection
#[derive(Debug)]
pub struct Cat {
    /// Cat name, can be same fore two
    name: String,
    /// Cat username, they are modern and live in internet era of cats. These are unique
    username: String,
    /// A number assigned to cat for easy reference, new kittens are not assigned yet so it can be
    /// None
    number: Option<i64>,
    /// Cats have friends, like us. They keep their username with them
    friends: Vec<String>,
}

#[test]
fn test() {
    let cat1 = CatMutRefBuilderOwnedTypes::new()
        .name("good kittie")
        .username("kiten1024")
        .number(12)
        .friend("goodkitten12")
        .friend("goodkitten13")
        .friend("goodkitten14")
        .friends(&["kitten1", "kitten1", "kitten1", "kitten1", "kitten1"])
        .build();

    let cat2 = CatMutRefBuilderBorrowTypes::new()
        .name("good kittie")
        .username("kiten1024")
        .number(12)
        .friend("goodkitten12")
        .friend("goodkitten13")
        .friend("goodkitten14")
        .friends(&["kitten1", "kitten1", "kitten1", "kitten1", "kitten1"])
        .build();

    let cat3 = CatOwnedBuilderOwnedTypes::new()
        .name("good kittie")
        .username("kiten1024")
        .number(12)
        .friend("goodkitten12")
        .friend("goodkitten13")
        .friend("goodkitten14")
        .friends(&["kitten1", "kitten1", "kitten1", "kitten1", "kitten1"])
        .build();

    let cat4 = CatOwnedBuilderBorrowTypes::new()
        .name("good kittie")
        .username("kiten1024")
        .number(12)
        .friend("goodkitten12")
        .friend("goodkitten13")
        .friend("goodkitten14")
        .friends(&["kitten1", "kitten1", "kitten1", "kitten1", "kitten1"])
        .build();

    dbg!(cat1);
    dbg!(cat2);
    dbg!(cat3);
    dbg!(cat4);
}
