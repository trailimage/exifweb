use crate::config::PhotoConfig;
use serde::{Deserialize, Serialize};

/// Suffixes added to resized photo files
pub mod suffix {
    pub static ORIGINAL: &'static str = "o";
    pub static LARGE: &'static str = "l";
    pub static MEDIUM: &'static str = "m";
    pub static SMALL: &'static str = "s";
    pub static THUMB: &'static str = "t";
}

/// Photo display sizes. These values may be less than the rendered sizes to
/// accomodate high density screens.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SizeCollection {
    #[serde(skip)]
    pub original: Size,

    /// Size shown when image is enlarged
    pub large: Size,
    /// Main size within post body
    pub medium: Size,
    /// Size shown in post summary on category page
    pub small: Size,
    /// Size shown in search results
    pub thumb: Size,
}

impl SizeCollection {
    /// Create size collection from original size
    pub fn from(
        width: u16,
        height: u16,
        index: u8,
        config: &PhotoConfig,
    ) -> Self {
        let name = |end: &'static str| {
            format!("{:03}_{}{}", index, end, config.output_ext)
        };
        let original = Size::new(width, height, name(suffix::ORIGINAL));
        let size = &config.size.display;

        SizeCollection {
            large: original.limit_to(size.large, name(suffix::LARGE)),
            medium: original.limit_to(size.medium, name(suffix::MEDIUM)),
            small: original.limit_to(size.small, name(suffix::SMALL)),
            thumb: Size {
                width: size.thumb,
                height: size.thumb,
                name: name(suffix::THUMB),
            },
            original,
        }
    }

    /// Whether photo is in portrait orientation (taller than wide)
    pub fn is_portrait(&self) -> bool {
        self.original.width < self.original.height
    }

    /// Whether photo is in landscape orientation (wider than tall)
    pub fn is_landscape(&self) -> bool {
        self.original.width > self.original.height
    }

    /// Image width divided by height
    pub fn aspect_ratio(&self) -> f32 {
        self.original.width as f32 / self.original.height as f32
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
    pub name: String,
}

impl Size {
    pub fn new(width: u16, height: u16, name: String) -> Self {
        Size {
            width,
            height,
            name,
        }
    }

    /// Find coordinates necessary to crop a center square
    ///  - *returns* x, y, edge length
    pub fn center_square(&self) -> (u16, u16, u16) {
        if self.width > self.height {
            // crop left and right
            ((self.width - self.height) / 2, 0, self.height)
        } else {
            // crop top and bottom
            (0, (self.height - self.width) / 2, self.width)
        }
    }

    /// Update dimensions so long edge does not exceed `long_edge`. This will
    /// not enlarge the image.
    pub fn limit_to(&self, long_edge: u16, new_name: String) -> Size {
        if long_edge > self.width && long_edge > self.height {
            let mut copy = self.clone();
            copy.name = new_name;
            copy
        } else if self.height > self.width {
            let width = (self.width as f32
                * (long_edge as f32 / self.height as f32))
                .round() as u16;
            Size::new(width, long_edge, new_name)
        } else {
            let height = (self.height as f32
                * (long_edge as f32 / self.width as f32))
                .round() as u16;
            Size::new(long_edge, height, new_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize() {
        let name = |end: &'static str| format!("001_{}.wepb", end);

        let source = Size::new(1024, 768, name(suffix::LARGE));
        let target = Size::new(800, 600, name(suffix::MEDIUM));

        assert_eq!(source.limit_to(800, name(suffix::MEDIUM)), target);

        let source = Size::new(768, 1024, name(suffix::LARGE));
        let target = Size::new(600, 800, name(suffix::MEDIUM));

        assert_eq!(source.limit_to(800, name(suffix::MEDIUM)), target);
    }
}
