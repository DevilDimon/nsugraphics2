pub struct ImageDetails {
    image_type: ImageType,
    illumination: Vec<ImageIllumination>,
}

enum ImageType {
    Pack8,
    Pseudo,
    Sharpie,
    CSharpie,
    PCA(PCAFlag),
    Color,
}

enum PCAFlag {
    C(u8),
    RGB((bool, u8), (bool, u8), (bool, u8)),
}

enum ImageIllumination {
    LED0365,
    LED0450,
    LED470,
    LED505,
    LED535,
    LED570,
    LED615,
    LED630,
    LED700,
    LED735,
    LED780,
    LED870,
    LED940,
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

pub fn extract_image_details(img_name: &str) -> ImageDetails {
    ImageDetails {
        image_type: ImageType::Pack8,
        illumination: vec![]
    }
}