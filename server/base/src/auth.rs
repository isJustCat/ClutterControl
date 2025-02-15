use std::{collections::hash_set, str::from_utf8};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHashString, Result, Salt, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier
};


fn hash_password(password: &str)  -> Option<String> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();

    // crimes <:
   match from_utf8(argon2.hash_password(password.as_bytes(), &salt).unwrap().serialize().as_bytes()) {
    Ok(hash_str) => {
        Some(String::from(hash_str))
    },
    Err(e) => {
        // lulz
        None
    }

   }
}