mod args;
use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView, ImageError };
use std::convert::TryInto;

// calls to unwrap in your code mean that, at runtime:
// our code may panic, and may not provide the most useful error
// it will be worth it to add proper error handling!

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        // you can use these two lines the same
        // let buffer_capacity = 3655744;
        // let buffer_capacity = 3_655_744;

        // the * 4 is for each rgba channel
        let buffer_capacity = height * width * 4;

        // reserve space for pixel data in memory using buffer
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());

        FloatingImage {
            width, 
            height, 
            data: buffer, 
            name
        }
    }

    // methods take a reference to self as their first value
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        // if the datas length is longer than the allotted capacity on the image, we cannot store it!
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_1, image_format_1) = find_image_from_path(args.image_1)?;
    let (image_2, image_format_2) = find_image_from_path(args.image_2)?;

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardize_size(image_1, image_2);

    // build output image
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);
    // propogate the Ok() Result from set_data up into this function with the question mark!
    output.set_data(combined_data)?;

    // save new image to file
    // notice we pass a reference to the data, not its value
    if let Err(e) = image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_format_1) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        Ok(())
    }
}


fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    match Reader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                return match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e))
                };
            } else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        },
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e))
    }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_num_1 = dim_1.0 * dim_1.1;
    let pix_num_2 = dim_2.0 * dim_2.1;

    return if pix_num_1 < pix_num_2 { dim_1 } else { dim_2 };
}

// this function resizes the images
fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    // println!("width: {}, height: {}", width, height);

    if image_2.dimensions() == (width, height) {
        // resize image 1, return both in a tuple
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        // resize image 2, return both in a tuple
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    // if vec_1.len() == 5, then this vec macro creates a vec of u8s that looks like this [0,0,0,0,0]
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {

        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }

        // loop through each rgba set
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let val: u8 = match vec.get(i) {
            // asterisk before the value dereferenceds the value
            Some(d) => *d,
            None => panic!("Index out of bounds")
        };
        rgba.push(val);
    }

    rgba
}