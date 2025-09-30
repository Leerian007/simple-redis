use rand::prelude::*;
use rand::{random, random_range, thread_rng};

pub mod database;

pub mod auth_utils;

use auth_utils::modules::Credentials;
use auth_utils::auth_user;

pub fn auth_user_info(cert: &Credentials) ->Option<String>{
    // let timeout = thread_rng().gen_range(100..500);
    let timeout = random_range(100..500);
    println!("timeout num is {}", timeout);
    auth_user(cert)
}