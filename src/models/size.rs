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
        let original = Size { width, height };

        SizeCollection {
            large: original.limit_to(config.large),
            regular: original.limit_to(config.regular),
            small: original.limit_to(config.small),
            thumb: Size {
                width: config.thumb,
                height: config.thumb,
            },
            original,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
impl Size {
    // TODO: handle image URLs
    /// Update dimensions so long edge does not exceed `long_edge`. This will
    /// not enlarge the image.
    pub fn limit_to(&self, long_edge: u16) -> Size {
        if long_edge > self.width && long_edge > self.height {
            self.clone()
        } else if self.height > self.width {
            Size {
                width: (self.width as f32
                    * (long_edge as f32 / self.height as f32))
                    .round() as u16,
                height: long_edge,
            }
        } else {
            Size {
                width: long_edge,
                height: (self.height as f32
                    * (long_edge as f32 / self.width as f32))
                    .round() as u16,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize() {
        let source = Size {
            width: 1024,
            height: 768,
        };
        let target = Size {
            width: 800,
            height: 600,
        };

        assert_eq!(source.limit_to(800), target);

        let source = Size {
            width: 768,
            height: 1024,
        };
        let target = Size {
            width: 600,
            height: 800,
        };

        assert_eq!(source.limit_to(800), target);
    }
}
