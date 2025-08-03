use sysinfo::{Users};
use crate::users::types::MyUser;

pub fn get_user_by_id(user_id: sysinfo::Uid) -> Option<MyUser> {
    let users = Users::new_with_refreshed_list();
    for user in users.list() {
        if user.id() == &user_id {
            return Some(MyUser {
                id: user.id().clone(),
                name: user.name().to_string(),
                groups: user.groups().iter().map(|g| g.name().to_string()).collect(),
            });
        }
    }
    None
}