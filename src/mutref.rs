use crate::Cat;

/// It's not easy to generate cats out of thin air, hence it can fail.
#[derive(Debug, thiserror::Error)]
pub enum CatBuildError {
    #[error("Cat generation spell doesnt work")]
    Generic,
}

/// Build [Cat] using Owned types and propogates using mutable reference
#[derive(Debug, Default)]
pub struct CatMutRefBuilderOwnedTypes {
    name: Option<String>,
    username: Option<String>,
    number: Option<i64>,
    friends: Vec<String>,
}

impl CatMutRefBuilderOwnedTypes {
    pub fn new() -> CatMutRefBuilderOwnedTypes {
        Self {
            ..Default::default()
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.to_owned());
        self
    }

    pub fn number(&mut self, number: i64) -> &mut Self {
        self.number = Some(number);
        self
    }

    pub fn friend(&mut self, friend: &str) -> &mut Self {
        self.friends.push(friend.into());
        self
    }

    pub fn friends(&mut self, friends: &[&str]) -> &mut Self {
        self.friends
            .extend(friends.iter().map(|str| str.to_string()));
        self
    }

    pub fn build(&self) -> Result<Cat, CatBuildError> {
        Ok(Cat {
            name: self.name.as_deref().ok_or(CatBuildError::Generic)?.into(),
            username: self
                .username
                .as_deref()
                .ok_or(CatBuildError::Generic)?
                .into(),
            number: self.number,
            friends: self.friends.clone(),
        })
    }
}

/// Build [Cat] using borrowed types and propogates using mutable reference
#[derive(Debug, Default)]
pub struct CatMutRefBuilderBorrowTypes<'build> {
    name: Option<&'build str>,
    username: Option<&'build str>,
    number: Option<i64>,
    friends: Vec<&'build str>,
}

impl<'build> CatMutRefBuilderBorrowTypes<'build> {
    pub fn new() -> CatMutRefBuilderBorrowTypes<'build> {
        Self {
            ..Default::default()
        }
    }

    pub fn name(&mut self, name: &'build str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn username(&mut self, username: &'build str) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn number(&mut self, number: i64) -> &mut Self {
        self.number = Some(number);
        self
    }

    pub fn friend(&mut self, friend: &'build str) -> &mut Self {
        self.friends.push(friend);
        self
    }

    pub fn friends(&mut self, friends: &'build [&'build str]) -> &mut Self {
        self.friends.extend_from_slice(friends);
        self
    }

    pub fn build(&self) -> Result<Cat, CatBuildError> {
        Ok(Cat {
            name: self.name.ok_or(CatBuildError::Generic)?.into(),
            username: self.username.ok_or(CatBuildError::Generic)?.into(),
            number: self.number,
            friends: self.friends.iter().map(ToString::to_string).collect(),
        })
    }
}
