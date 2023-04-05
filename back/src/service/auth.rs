use crate::model::user::User;
use log::info;
use sanitizer::Sanitize;
use uuid::Uuid;
use validator::Validate;

use crate::repository::user as user_repository;

pub mod parameter {
    use sanitizer::prelude::*;
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Validate, Sanitize, Deserialize)]
    pub struct Register {
        #[sanitize(trim)]
        #[validate(length(min = 3, max = 32))]
        pub username: String,

        #[sanitize(trim)]
        #[validate(email)]
        pub email: String,

        #[sanitize(trim)]
        #[validate(length(min = 6, max = 32))]
        pub password: String,
    }

    pub struct Login {
        username: String,
        password: String,
    }
}

pub async fn register(mut register: parameter::Register) {
    register.sanitize();

    if let Err(_e) = register.validate() {
        //return HttpResponse::BadRequest().body(e.to_string());
    }

    let user = User {
        id: Uuid::new_v4(),
        username: register.username.clone(),
        email: register.email.clone(),
        password: register.password.clone(),
    };

    let _user = user_repository::insert(user).await;

    info!("Registering user: {}", register.username);
}
