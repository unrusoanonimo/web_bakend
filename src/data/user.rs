#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum PermissionBits {
    Admin = 1,
    Support = 2,
    Chat = 4,
}
impl PermissionBits {
    pub fn generate_permissions(permissions: &[PermissionBits]) -> u32 {
        let mut a = 0;
        for permission in permissions {
            a += permission.clone() as u32;
        }
        a
    }
}

pub trait PubUser {
    fn id(&self) -> &u64;
}
pub trait User: PubUser {
    fn token(&self) -> &str;
    fn permissions(&self) -> &u32;
    fn has_permission(&self, perm: PermissionBits) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    id: u64,
    token: String,
    permissions: u32,
}
impl UserData {
    pub fn new(id: u64, token: String, permissions: u32) -> Self {
        Self {
            id,
            token,
            permissions,
        }
    }
}
impl PubUser for UserData {
    fn id(&self) -> &u64 {
        &self.id
    }
}
impl User for UserData {
    fn token(&self) -> &str {
        &self.token
    }
    fn permissions(&self) -> &u32 {
        &self.permissions
    }
    fn has_permission(&self, perm: PermissionBits) -> bool {
        let n = self.permissions & perm as u32;
        n != 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    id: u64,
    mac: String,
}
impl LoginData {
    pub fn new(id: u64) -> Option<Self> {
        use mac_address::get_mac_address;
        Some(Self {
            id: id.into(),
            mac: get_mac_address().ok()??.to_string(),
        })
    }
    pub fn id(&self) -> &u64 {
        &self.id
    }
    pub fn mac(&self) -> &str {
        &self.mac
    }
}
