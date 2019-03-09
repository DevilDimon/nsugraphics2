use std::fmt;

pub struct ImageDetails {
    image_type: ImageType,
    illumination: Vec<ImageIllumination>,
}

impl fmt::Display for ImageDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut illumination_str = String::new();
        for illumination in self.illumination.iter() {
            illumination_str += &format!("{}", illumination);
        }

        write!(f, "image type: {}\nillumination: {}", self.image_type, illumination_str)
    }
}

enum ImageType {
    Pack8,
    Pseudo,
    Sharpie,
    CSharpie,
    PCA(PCAFlag),
    Color,
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageType::Pack8 => write!(f, "an 8-bit version of the 'raw' capture image"),
            ImageType::Pseudo => write!(f, "pseudo-color image"),
            ImageType::Sharpie => write!(f, "monochrome image derived from the pseudo-color image that 'strips away' the over text"),
            ImageType::CSharpie => write!(f, "\"color sharpie\" image; a color version of the monochrome sharpie that employs all three ultravoliet images taken with the color filter wheel"),
            ImageType::PCA(pca_flag) => {
                write!(f, "an image derived from a Principal Component Analysis, ").expect("Unexpected error");
                match pca_flag {
                    PCAFlag::C(_) => write!(f, "single component"),
                    PCAFlag::RGB(_,_,_) => write!(f, "a color image produced from three single component images"),
                }
            },
            ImageType::Color => write!(f, "a color image generated from five separate visible light images"),
        }
    }
}

enum PCAFlag {
    C(u8),
    RGB((bool, u8), (bool, u8), (bool, u8)),
}

enum ImageIllumination {
    LED(u16),
    RAIR,
    RABR,
    RAIL,
    RABL,
    CFUR,
    CFUG,
    CFUB,
    CFBR,
    CFBG,
    CFBB,
    CFUX,
}

impl fmt::Display for ImageIllumination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageIllumination::LED(nm) => write!(f, "{} nm LED illumination", nm),
            ImageIllumination::RAIR => write!(f, "raking infrared (940 nm) illumination from the right"),
            ImageIllumination::RABR => write!(f, "raking blue (470 nm) illumination from the right"),
            ImageIllumination::RAIL => write!(f, "raking infrared (940 nm) illumination from the left"),
            ImageIllumination::RABL => write!(f, "raking blue (470 nm) illumination the left"),
            ImageIllumination::CFUR => write!(f, "ultraviolet (365 nm) illumination with red color filter"),
            ImageIllumination::CFUG => write!(f, "ultraviolet (365 nm) illumination with green color filter"),
            ImageIllumination::CFUB => write!(f, "ultraviolet (365 nm) illumination with blue color filter"),
            ImageIllumination::CFBR => write!(f, "blue (450 nm) illumination with red color filter"),
            ImageIllumination::CFBG => write!(f, "blue (450 nm) illumination with green color filter"),
            ImageIllumination::CFBB => write!(f, "blue (450 nm) illumination with blue color filter"),
            ImageIllumination::CFUX => write!(f, "all three color filter ultraviolet images in combination (CFUR, CFUG, CFUB)"),
        }
    }
}

pub fn extract_image_details(img_name: &str) -> ImageDetails {
    ImageDetails {
        image_type: ImageType::Pack8,
        illumination: vec![]
    }
}