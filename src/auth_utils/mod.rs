use crate::auth_utils::modules::Credentials;
use crate::database::query_user_in_database;

pub mod modules;

pub fn auth_user(cert: &Credentials) ->Option<String>{
    query_user_in_database(cert).ok()
}

fn login(cert: &Credentials){

}

fn logout(cert: &Credentials){}