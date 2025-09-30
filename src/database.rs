// use crate::auth_utils::Credentials;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::auth_utils::modules::Credentials;

enum Status{
    Connected,
    Disconnected,
}

fn connect_database() ->Status{
    Status::Connected
}

pub fn query_user_in_database(cert: &Credentials) ->Result<String,String>{
    if cert.username().is_empty() {
        return Err(String::from("Username is empty"));
    }else if cert.password().is_empty() {
        return Err(String::from("Password is empty"));
    }
    // Ok(cert.username().to_string())
    Ok(simple_hash(cert.password()))
}

fn simple_hash(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    //format!("{:x}", hasher.finish())
    format!("{:016x}", hasher.finish())
}
