use serde::{Deserialize, Serialize};
pub mod app;
pub mod config;
#[derive(Clone, Serialize)]
enum CDN {
    Twitch,
    Youtube,
    Vimeo,
}

#[derive(Clone, Serialize)]
struct CDNConfig {
    cdn: CDN,
}

impl CDNConfig {
    pub fn new(cdn: CDN) -> Self {
        Self { cdn }
    }
}

#[derive(Clone, Serialize)]
pub struct User {
    username: String,
    email: String,
    is_live: bool,
    following: Vec<(Box<User>, bool)>,
    cdn_config: Option<CDNConfig>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl User {
    pub fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            is_live: false,
            following: vec![],
            cdn_config: None,
        }
    }

    pub fn id(&self) -> String {
        self.username.clone()
    }
    pub fn go_live(&mut self) {
        self.is_live = true
    }

    pub fn go_offline(&mut self) {
        self.is_live = false
    }

    pub fn is_live(&self) -> bool {
        self.is_live
    }

    pub fn follow(&mut self, target: Box<User>, sub: bool) -> Result<(), String> {
        for (user, _) in &self.following {
            if user.id() == target.id() {
                return Err(format!(
                    "{} is already following {}",
                    user.username, target.username
                ));
            }
        }
        self.following.push((target, sub));
        Ok(())
    }

    pub fn subscribe(&mut self, target: Box<User>) -> Result<(), String> {
        self.follow(target, true)
    }
    pub fn unfollow(&mut self, target: &User) -> Result<(), String> {
        for (i, (user, subbed)) in self.following.iter().enumerate() {
            if user.id() == target.id() {
                if *subbed {
                    return Err(format!(
                        "{} is still subbed to {}",
                        self.username, target.username
                    ));
                } else {
                    self.following.remove(i);
                    return Ok(());
                }
            }
        }

        return Err(format!(
            "{} is not following {}",
            self.username, target.username
        ));
    }

    pub fn unsubscribe(&mut self, target: &User) -> Result<(), String> {
        for (user, subbed) in &mut self.following {
            if user.id() == target.id() {
                *subbed = false;
                return Ok(());
            }
        }
        return Err(format!(
            "{} is not subbed to {}",
            self.username, target.username
        ));
    }
}
