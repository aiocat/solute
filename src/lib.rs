use draw::{Drawing, Style, Shape, render, RGB, SvgRenderer, Canvas};
use sha2::{Digest, Sha224};
use std::str;

/// Main struct for solute.
pub struct Avatar {
    pub nickname: String,
    pub hash: String,
    pub avatar_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
}

impl Avatar {
    /// Create a new avatar struct.
    ///
    /// # Arguments
    ///
    /// * `nickname` (`&str`) - User nickname.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut avatar = solute::Avatar::new("aiocat");
    /// ```
    pub fn new(nickname: &str) -> Avatar {
        let mut hasher = Sha224::new();
        hasher.update(nickname.as_bytes());
        let hash = hasher.finalize();

        Avatar {
            nickname: nickname.to_owned(),
            hash: format!("{:x}", hash),
            avatar_color: (0, 0, 0),
            background_color: (255, 255, 255),
        }
    }

    /// Set a background for avatar.
    ///
    /// # Arguments
    ///
    /// * `r` (`u8`) - Red color.
    /// * `g` (`u8`) - Green color.
    /// * `b` (`u8`) - Blue color.
    ///
    /// # Examples
    ///
    /// ```
    /// avatar.set_background_color(25, 50, 75);
    /// ```
    pub fn set_background_color(&mut self, r: u8, b: u8, g: u8) {
        self.background_color = (r, g, b)
    }

    /// Draw the avatar and save the svg to the path.
    ///
    /// # Arguments
    ///
    /// * `path` (`&str`) - /path/to/save the svg file.
    ///
    /// # Examples
    ///
    /// ```
    /// avatar.draw("test.svg");
    /// ```
    pub fn draw(&mut self, path: &str) {
        self.initialize_avatar_color();
        let chunked_substrs = split_string(&self.hash, 3);
        let mut canvas = Canvas::new(200, 200);

        let values = chunked_substrs.iter().enumerate().map(|(i, val)| {
            let width = (i % 5) * 50;
            let height = (i / 5) * 50;

            if i32::from_str_radix(val, 16).unwrap() % 2 == 0 {
                return Drawing::new()
                    .with_shape(Shape::Rectangle {
                        width: width as u32 + 50,
                        height: height as u32 + 50,
                    })
                    .with_xy(width as f32, height as f32)
                    .with_style(Style::filled(RGB {
                        r: self.avatar_color.0,
                        g: self.avatar_color.1,
                        b: self.avatar_color.2,
                    }));
            } else {
                return Drawing::new()
                    .with_shape(Shape::Rectangle {
                        width: width as u32 + 50,
                        height: height as u32 + 50,
                    })
                    .with_xy(width as f32, height as f32)
                    .with_style(Style::filled(RGB {
                        r: self.background_color.0,
                        g: self.background_color.1,
                        b: self.background_color.2,
                    }));
            }
        });

        for val in values {
            canvas.display_list.add(val);
        }

        render::save(&canvas, path, SvgRenderer::new()).expect("Failed to save");
    }

    fn initialize_avatar_color(&mut self) {
        self.avatar_color = hex_2_rgb(&self.hash[0..6]);
    }
}

fn hex_2_rgb(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let b = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let g = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r, g, b)
}

fn split_string(text: &String, chuck: usize) -> Vec<&str> {
    let subs = text
        .as_bytes()
        .chunks(chuck)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    subs
}
