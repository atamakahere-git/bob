use crate::Cat;

/// It's not easy to generate cats out of thin air, hence it can fail.
#[derive(Debug, thiserror::Error)]
pub enum CatBuildError {
    #[error("Cat generation spell doesnt work")]
    Generic,
}

// Builder pattern for cat creation using mutable reference
#[derive(Debug, Default)]
pub struct CatOwnedBuilderOwnedTypes {
    name: Option<String>,
    username: Option<String>,
    number: Option<i64>,
    friends: Vec<String>,
}

impl CatOwnedBuilderOwnedTypes {
    pub fn new() -> CatOwnedBuilderOwnedTypes {
        Self {
            ..Default::default()
        }
    }

    pub fn name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            ..self
        }
    }

    pub fn username(self, username: &str) -> Self {
        Self {
            username: Some(username.to_owned()),
            ..self
        }
    }

    pub fn number(self, number: i64) -> Self {
        Self {
            number: Some(number),
            ..self
        }
    }

    pub fn friend(self, friend: &str) -> Self {
        let mut friends = self.friends;
        friends.push(friend.into());
        Self { friends, ..self }
    }

    pub fn friends(self, friends: &[&str]) -> Self {
        let mut friends_inner = self.friends;
        friends_inner.extend(friends.into_iter().map(|str| str.to_string()));
        Self {
            friends: friends_inner,
            ..self
        }
    }

    pub fn build(self) -> Result<Cat, CatBuildError> {
        Ok(Cat {
            name: self.name.ok_or(CatBuildError::Generic)?,
            username: self.username.ok_or(CatBuildError::Generic)?,
            number: self.number,
            friends: self.friends,
        })
    }
}

#[derive(Debug, Default)]
pub struct CatOwnedBuilderBorrowTypes<'build> {
    name: Option<&'build str>,
    username: Option<&'build str>,
    number: Option<i64>,
    friends: Vec<&'build str>,
}

impl<'build> CatOwnedBuilderBorrowTypes<'build> {
    pub fn new() -> CatOwnedBuilderBorrowTypes<'build> {
        Self {
            ..Default::default()
        }
    }

    pub fn name(self, name: &'build str) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }

    pub fn username(self, username: &'build str) -> Self {
        Self {
            username: Some(username),
            ..self
        }
    }

    pub fn number(self, number: i64) -> Self {
        Self {
            number: Some(number),
            ..self
        }
    }

    pub fn friend(self, friend: &'build str) -> Self {
        let mut friends = self.friends;
        friends.push(friend.into());
        Self { friends, ..self }
    }

    pub fn friends(self, friends: &'build [&'build str]) -> Self {
        let mut friends_inner = self.friends;
        friends_inner.extend_from_slice(friends);
        Self {
            friends: friends_inner,
            ..self
        }
    }

    pub fn build(self) -> Result<Cat, CatBuildError> {
        Ok(Cat {
            name: self.name.ok_or(CatBuildError::Generic)?.into(),
            username: self.username.ok_or(CatBuildError::Generic)?.into(),
            number: self.number,
            friends: self.friends.iter().map(ToString::to_string).collect(),
        })
    }
}
