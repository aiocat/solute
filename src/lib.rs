use sha2::{Sha224, Digest};

#[derive(Debug)]
pub struct Avatar {
    pub nickname: String,
    pub hash: String,
    pub avatar_color: (u8, u8, u8)
}

impl Avatar {
    pub fn new(nickname: &str) -> Avatar {
        let mut hasher = Sha224::new();
        hasher.update(nickname.as_bytes());
        let hash = hasher.finalize();

        Avatar {
            nickname: nickname.to_owned(),
            hash: format!("{:x}", hash),
            avatar_color: (0, 0, 0)
        }
    }

    pub fn draw(&mut self) {
        self.initialize_avatar_color();
    }

    fn initialize_avatar_color(&mut self) {
        println!("{}", self.hash);
        self.avatar_color = hex_2_rgb(&self.hash[0..6]);
    }
}

fn hex_2_rgb(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let b = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let g = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r, g, b)
}