use thiserror::Error;

#[derive(Error, Debug)]
enum LoginError {
    //    #[error("database error")]
    //    DatabaseError(#[from] SqlError),
    #[error("password expired")]
    PasswordExpired,

    #[error("user not found")]
    NetworkError(#[from] std::io::Error), // #[from] convert std::io::Error ✅

    #[error("wrong password")]
    WrongPassword,
}

fn login(user: &str, password: &str) -> Result<String, LoginError> {
    if true {
        Err(LoginError::PasswordExpired)
    } else {
        Ok("session id".to_string())
    }
    // let user_id = get_user_id(user)?;
    // if try_password(user_id, password)? {
    //     let session: Result<String, SqlError> = get_session(user_id)?;
    //     Ok(session)
    // } else {
    //     Err(LoginError::WrongPassword)
    // }
}

fn main() {
    login("kumagai", "123");
}
