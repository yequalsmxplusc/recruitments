// use yew::functional::Reducible;
// use serde::{Serialize, Deserialize};
// use gloo_storage::{LocalStorage, Storage};
// use yew::UseReducerHandle;
// use std::rc::Rc;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AuthContext {
//     pub token: Option<String>,
//     pub username: Option<String>,
// }

// impl Default for AuthContext {
//     fn default() -> Self {
//         if let Ok(auth) = LocalStorage::get("auth") {
//             auth
//         } else {
//             Self {
//                 token: None,
//                 username: None,
//             }
//         }
//     }
// }

// impl Reducible for AuthContext {
//     type Action = AuthAction;

//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         let new_self = match action {
//             AuthAction::Login(token, username) => {
//                 let auth = Self {
//                     token: Some(token.clone()),
//                     username: Some(username.clone()),
//                 };
//                 LocalStorage::set("auth", &auth).unwrap();
//                 auth
//             }
//             AuthAction::Logout => {
//                 LocalStorage::delete("auth");
//                 Self {
//                     token: None,
//                     username: None,
//                 }
//             }
//         };
//         Rc::new(new_self)
//     }
// }

// pub enum AuthAction {
//     Login(String, String),
//     Logout,
// }

// #[derive(Clone, PartialEq)]
// pub struct AuthContextHandle {
//     pub inner: UseReducerHandle<AuthContext>,
// }


// impl AuthContextHandle {
//     pub fn is_authenticated(&self) -> bool {
//         self.inner.token.is_some()
//     }

//     pub fn token(&self) -> String {
//         self.inner.token.clone().unwrap_or_default()
//     }

//     pub fn username(&self) -> String {
//         self.inner.username.clone().unwrap_or_default()
//     }

//     pub fn dispatch(&self, action: AuthAction) {
//         self.inner.dispatch(action);
//     }
// }

use std::rc::Rc;
use yew::functional::Reducible;
use serde::{Serialize, Deserialize};
use gloo_storage::{LocalStorage, Storage};
use yew::UseReducerHandle;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub username: Option<String>,
}

impl Default for AuthContext {
    fn default() -> Self {
        if let Ok(auth) = LocalStorage::get("auth") {
            auth
        } else {
            Self {
                token: None,
                username: None,
            }
        }
    }
}

impl Reducible for AuthContext {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_self = match action {
            AuthAction::Login(token, username) => {
                let auth = Self {
                    token: Some(token.clone()),
                    username: Some(username.clone()),
                };
                LocalStorage::set("auth", &auth).unwrap();
                auth
            }
            AuthAction::Logout => {
                LocalStorage::delete("auth");
                Self {
                    token: None,
                    username: None,
                }
            }
        };
        Rc::new(new_self)
    }
}

pub enum AuthAction {
    Login(String, String),
    Logout,
}

#[derive(Clone, PartialEq)]
pub struct AuthContextHandle {
    pub inner: UseReducerHandle<AuthContext>,
}

impl AuthContextHandle {
    pub fn is_authenticated(&self) -> bool {
        self.inner.token.is_some()
    }

    pub fn token(&self) -> String {
        self.inner.token.clone().unwrap_or_default()
    }
    
    // pub fn username(&self) -> String {
    //     self.inner.username.clone().unwrap_or_default()
    // }

    pub fn dispatch(&self, action: AuthAction) {
        self.inner.dispatch(action);
    }
    pub fn logout(&self) {
        self.dispatch(AuthAction::Logout);
    }
}