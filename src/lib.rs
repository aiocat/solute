#[derive(Debug)]
pub struct Avatar {
    pub nickname: String
}

impl Avatar {
    pub fn new(nickname: &str) -> Avatar {
        Avatar {
            nickname: nickname.to_owned()
        }
    }
}