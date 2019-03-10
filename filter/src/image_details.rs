use std::fmt;

pub struct ImageDetails {
    image_type: ImageType,
    illumination: Vec<ImageIllumination>,
}

impl fmt::Display for ImageDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut illumination_str = String::new();
        for illumination in self.illumination.iter() {
            illumination_str += &format!("{}, ", illumination);
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

impl ImageIllumination {
    fn from(string: &str) -> Option<Self> {
        match string.parse::<u16>() {
            Ok(n) => return Option::from(ImageIllumination::LED(n)),
            Err(_) => ()
        };

        match string {
            "RAIR" => Option::from(ImageIllumination::RAIR),
            "RABR" => Option::from(ImageIllumination::RABR),
            "RAIL" => Option::from(ImageIllumination::RAIL),
            "RABL" => Option::from(ImageIllumination::RABL),
            "CFUR" => Option::from(ImageIllumination::CFUR),
            "CFUG" => Option::from(ImageIllumination::CFUG),
            "CFUB" => Option::from(ImageIllumination::CFUB),
            "CFBR" => Option::from(ImageIllumination::CFBR),
            "CFBG" => Option::from(ImageIllumination::CFBG),
            "CFBB" => Option::from(ImageIllumination::CFBB),
            "CFUX" => Option::from(ImageIllumination::CFUX),
            _ => Option::None
        }
    }
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
    let mut iter = img_name.split('_');
    iter.next().expect("Malformed name"); // Skip troparia folio
    iter.next().expect("Malformed name"); // Skip shot sequence number

    let sequence_ext_or_processing = iter.next().expect("Malformed name");
    let processing_details;
    if vec!["A", "B", "C", "D", "E"].contains(&sequence_ext_or_processing) {
        processing_details = iter.next().expect("Malformed name");
    } else {
        processing_details = sequence_ext_or_processing;
    }

    if let Some(illumination) = ImageIllumination::from(processing_details) {
        return ImageDetails {
            image_type: ImageType::Pack8,
            illumination: vec![illumination]
        }
    }

    match processing_details {
        "color" => {
            return ImageDetails {
                image_type: ImageType::Color,
                illumination: vec![]
            }
        },
        "pca" => {
            let pca_flag = iter.next().expect("Malformed name");
            if pca_flag.len() == 2 {
                let num = pca_flag.trim_start_matches('C');
                return ImageDetails {
                    image_type: ImageType::PCA(PCAFlag::C(num.parse::<u8>().expect("Malformed name"))),
                    illumination: vec![]
                }
            }
            let rgb_components: Vec<&str> = pca_flag.split('-').collect();
            let (r, g, b) = (rgb_components[1], rgb_components[2], rgb_components[3]);
            let r_num = r.trim_start_matches("inv");
            let r_tuple = (r.starts_with("inv"), r_num.parse::<u8>().expect("Malformed name"));
            let g_num = g.trim_start_matches("inv");
            let g_tuple = (g.starts_with("inv"), g_num.parse::<u8>().expect("Malformed name"));
            let b_num = b.trim_start_matches("inv");
            let b_tuple = (b.starts_with("inv"), b_num.parse::<u8>().expect("Malformed name"));
            return ImageDetails {
                image_type: ImageType::PCA(PCAFlag::RGB(r_tuple, g_tuple, b_tuple)),
                illumination: vec![]
            }
        },

        _ => ()
    };

    let image_type = match processing_details {
        "pseudo" => ImageType::Pseudo,
        "sharpie" => ImageType::Sharpie,
        "csharpie" => ImageType::CSharpie,
        _ => panic!("Malformed name")
    };

    let illumination: Vec<ImageIllumination> = iter.next().expect("Malformed name")
        .split('-')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|e| ImageIllumination::from(e))
        .flat_map(|e| e)
        .collect::<Vec<ImageIllumination>>();

    ImageDetails {
        image_type, illumination
    }

}