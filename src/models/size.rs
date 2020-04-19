use crate::config::SizeConfig;

#[derive(Debug, Default)]
pub struct SizeCollection {
    original: Size,
    /// Size shown when image is enlarged
    pub large: Size,
    /// Main size within post body
    pub regular: Size,
    /// Size shown in post summary on category page
    pub small: Size,
    /// Size shown in search results
    pub thumb: Size,
}

impl SizeCollection {
    /// Create size collection from original size
    pub fn from(width: u16, height: u16, config: &SizeConfig) -> Self {
        let original = Size::new(width, height);

        SizeCollection {
            large: original.limit_to(config.large),
            regular: original.limit_to(config.regular),
            small: original.limit_to(config.small),
            thumb: Size {
                width: config.thumb,
                height: config.thumb,
                url: String::new(),
            },
            original,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
    // TODO: handle image URLs
    pub url: String,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size {
            width,
            height,
            url: String::new(),
        }
    }

    /// Update dimensions so long edge does not exceed `long_edge`. This will
    /// not enlarge the image.
    pub fn limit_to(&self, long_edge: u16) -> Size {
        if long_edge > self.width && long_edge > self.height {
            self.clone()
        } else if self.height > self.width {
            let width = (self.width as f32
                * (long_edge as f32 / self.height as f32))
                .round() as u16;
            Size::new(width, long_edge)
        } else {
            let height = (self.height as f32
                * (long_edge as f32 / self.width as f32))
                .round() as u16;
            Size::new(long_edge, height)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize() {
        let source = Size::new(1024, 768);
        let target = Size::new(800, 600);

        assert_eq!(source.limit_to(800), target);

        let source = Size::new(768, 1024);
        let target = Size::new(600, 800);

        assert_eq!(source.limit_to(800), target);
    }
}
