use crate::config::{PhotoConfig, SizeConfig};

/// Suffixes added to resized photo files
mod suffix {
    pub static ORIGINAL: &'static str = "o";
    pub static LARGE: &'static str = "l";
    pub static MEDIUM: &'static str = "l";
    pub static SMALL: &'static str = "s";
    pub static THUMB: &'static str = "t";
}

#[derive(Debug, Default)]
pub struct SizeCollection {
    original: Size,

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
    pub fn from(width: u16, height: u16, config: &SizeConfig) -> Self {
        let original = Size::new(width, height, suffix::ORIGINAL);

        SizeCollection {
            large: original.limit_to(config.large, suffix::LARGE),
            medium: original.limit_to(config.medium, suffix::MEDIUM),
            small: original.limit_to(config.small, suffix::SMALL),
            thumb: Size {
                width: config.thumb,
                height: config.thumb,
                suffix: suffix::THUMB,
                url: String::new(),
            },
            original,
        }
    }

    /// Build root-relative URLs for all photo sizes
    pub fn build_urls(
        &mut self,
        post_path: &str,
        photo_index: u8,
        config: &PhotoConfig,
    ) {
        self.large.build_url(post_path, photo_index, config);
        self.medium.build_url(post_path, photo_index, config);
        self.small.build_url(post_path, photo_index, config);
        self.thumb.build_url(post_path, photo_index, config);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
    pub url: String,
    /// Suffix added to image name for size
    pub suffix: &'static str,
}

impl Size {
    pub fn new(width: u16, height: u16, suffix: &'static str) -> Self {
        Size {
            width,
            height,
            suffix,
            url: String::new(),
        }
    }

    /// Build root-relative URL for photo size
    pub fn build_url(
        &mut self,
        post_path: &str,
        photo_index: u8,
        config: &PhotoConfig,
    ) {
        self.url = format!(
            "{}/{:03}_{}{}",
            &post_path, photo_index, self.suffix, config.output_ext
        );
    }

    /// Update dimensions so long edge does not exceed `long_edge`. This will
    /// not enlarge the image.
    pub fn limit_to(&self, long_edge: u16, suffix: &'static str) -> Size {
        if long_edge > self.width && long_edge > self.height {
            let mut copy = self.clone();
            copy.suffix = suffix;
            copy
        } else if self.height > self.width {
            let width = (self.width as f32
                * (long_edge as f32 / self.height as f32))
                .round() as u16;
            Size::new(width, long_edge, suffix)
        } else {
            let height = (self.height as f32
                * (long_edge as f32 / self.width as f32))
                .round() as u16;
            Size::new(long_edge, height, suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Photo;

    #[test]
    fn test_resize() {
        let source = Size::new(1024, 768, suffix::LARGE);
        let target = Size::new(800, 600, suffix::MEDIUM);

        assert_eq!(source.limit_to(800, suffix::MEDIUM), target);

        let source = Size::new(768, 1024, suffix::LARGE);
        let target = Size::new(600, 800, suffix::MEDIUM);

        assert_eq!(source.limit_to(800, suffix::MEDIUM), target);
    }
}
