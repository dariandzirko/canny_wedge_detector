use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, Rgba,
};
use num::{pow, Float};
use std::{f32::consts::E, vec};

//Going to start with assuming rectangular kernels
pub struct Kernel {
    //Maybe ndarray will be a lot faster here
    //TODO look into ndarray
    matrix: Vec<Vec<f32>>,
    // Create a window with default options and display the image.
    dimensions: (usize, usize), //row, col
}

impl Kernel {
    //How do I return nothing from this
    pub fn new() -> Kernel {
        let matrix = vec![];
        // Should keep the dimensions as (width, height) to match the library
        let dimensions = (0, 0);
        Kernel { matrix, dimensions }
    }

    pub fn print_kernel(&self) {
        let (columns, rows) = self.dimensions;

        println!();
        for row in 0..rows {
            for col in 0..columns {
                print!("{} | ", self.matrix[row][col]);
            }
            println!();
        }
    }

    pub fn sum_kernel(&self) -> f32 {
        let mut sum = 0.0;

        self.matrix
            .iter()
            .flat_map(|row| row.iter())
            .for_each(|item| sum += *item);

        return sum;
    }

    pub fn gaussian_1d(radius: f32) {
        let mut dummy_filt = Kernel::new();

        let lim = 3.0 * (radius.floor() as f32);
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = vec![vec![0.0; length]; 1];
        let dimensions = (1, length);

        let mut sum = 0.0;

        for i in 0..length + 1 {
            let x = i as f32 - (lim);
            let val = E.powf(-(x.powf(2.0)) / (2.0 * radius.powf(2.0)));
            matrix[0][i] = val;
            sum += val;
        }

        matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item = *item / sum);

        dummy_filt.matrix = matrix;
        dummy_filt.dimensions = dimensions;
    }

    pub fn gaussian_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::new();

        let lim = (3.0 * radius).floor() as f32;
        let length = (2.0 * lim + 1.0) as usize;

        let mut matrix = vec![vec![0.0; length]; length];
        let dimensions = (length, length);

        let mut sum = 0.0;

        for row in 0..length {
            let x = row as f32 - (lim);
            for col in 0..length {
                let y = col as f32 - (lim);
                let val = E.powf(-(x.powf(2.0) + y.powf(2.0)) / (2.0 * radius.powf(2.0)));
                matrix[row][col] = val;
                sum += val;
            }
        }

        matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item = *item / sum);

        dummy_filt.matrix = matrix;
        dummy_filt.dimensions = dimensions;

        return dummy_filt;
    }

    pub fn highpass_2d(radius: f32) -> Kernel {
        let mut dummy_filt = Kernel::gaussian_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt
            .matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item = -(*item));

        dummy_filt.matrix[lim][lim] += 1.0;

        return dummy_filt;
    }

    pub fn sharpening_2d(radius: f32, beta: f32) -> Kernel {
        let mut dummy_filt = Kernel::highpass_2d(radius);

        let lim = (dummy_filt.dimensions.0 - 1) / 2;

        dummy_filt
            .matrix
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|item| *item /= beta);

        dummy_filt.matrix[lim][lim] += 1.0;

        return dummy_filt;
    }

    pub fn sobel_y_dir() -> Kernel {
        let mut dummy_filt = Kernel::new();

        dummy_filt.matrix = vec![
            vec![1.0, 2.0, 1.0],
            vec![0.0, 0.0, 0.0],
            vec![-1.0, -2.0, -1.0],
        ];

        dummy_filt.dimensions = (3, 3);

        return dummy_filt;
    }

    pub fn sobel_x_dir() -> Kernel {
        let mut dummy_filt = Kernel::new();

        dummy_filt.matrix = vec![
            vec![1.0, 0.0, -1.0],
            vec![2.0, 0.0, -2.0],
            vec![1.0, 0.0, -1.0],
        ];

        dummy_filt.dimensions = (3, 3);

        return dummy_filt;
    }

    // Dimensions is only one of 2 things because kernels are 2d
    //TODO flip should probably just flip in 1 dimension with an option to flip in both. So like dimension = 0 would be like along the
    //TODO x or columns, dimension = 1 would be along y or rows and dimension = 2 would be both or some shit. Or even make a 2d flip idk
    pub fn flip(&self, dimension: usize) -> Kernel {
        let mut mat = self.matrix.clone();
        let dims = self.dimensions;

        mat.iter_mut().for_each(|row| row.reverse());
        if dimension > 0 {
            mat.reverse();
        }

        //TODO implement some sort of copy or clone trait or some shit idk
        Kernel {
            matrix: mat,
            dimensions: dims,
        }
    }
}

//TODO return floating point representation of image, new struct that can convert to imagers image struct
pub fn conv_2d(kernel: &Kernel, base: &GrayImage, same_size: bool) -> GrayImage {
    //TODO I think my rows and columns are backwards
    //(width, height) everywhere
    let (base_cols, base_rows) = base.dimensions();
    let (kernel_cols, kernel_rows) = (kernel.dimensions.0 as u32, kernel.dimensions.1 as u32);

    let mut zero_pad_base = GrayImage::new(
        base_cols + 2 * (kernel_cols - 1),
        base_rows + 2 * (kernel_rows - 1),
    );

    image::imageops::overlay(
        &mut zero_pad_base,
        base,
        (kernel_cols - 1) as i64,
        (kernel_rows - 1) as i64,
    );

    //Swap the bounds and the zero_padded_elem commented lines to either get the "true"
    //convolution or the same size convolution.
    let result_cols = if same_size {
        base_cols
    } else {
        base_cols + kernel_cols - 1
    };

    let result_rows = if same_size {
        base_rows
    } else {
        base_rows + kernel_rows - 1
    };

    let mut min_value = 1000.0;
    let mut max_value = -1000.0;

    let mut result = GrayImage::new(result_cols, result_rows);

    //* The dimension does nothing as of now
    let flipped_kernel = kernel.flip(2);

    //* Right now the convolution returns an image the same size as the original image
    for row in 0..result_rows {
        for col in 0..result_cols {
            let mut sum = 0.0;
            //Going through the kernel math which only includes pixels in the kernel window
            //TODO include all pixel channels so this will work on RGB images
            for kernel_row in 0..kernel_rows {
                for kernel_col in 0..kernel_cols {
                    let flipped_kernel_elem =
                        flipped_kernel.matrix[kernel_row as usize][kernel_col as usize];
                    //*This has to be a fucking war crime

                    let zero_padded_elem: u8 = if same_size {
                        *zero_pad_base
                            .get_pixel(
                                col + kernel_col + kernel_cols / 2,
                                row + kernel_row + kernel_rows / 2,
                            )
                            .channels()
                            .get(0)
                            .unwrap()
                    } else {
                        *zero_pad_base
                            .get_pixel(col + kernel_col, row + kernel_row)
                            .channels()
                            .get(0)
                            .unwrap()
                    };

                    sum = sum + flipped_kernel_elem * zero_padded_elem as f32;
                }
            }

            // Scaling is fucking cursed
            if sum > max_value {
                max_value = sum;
            }
            if sum < min_value {
                min_value = sum;
            }
            // TODO Fix this. This is a horrible solution to negatives

            //Conv img
            //min -200
            //max 155

            sum = num::clamp(sum, 0.0, 255.0);
            let filtered_pixel: image::Luma<u8> = image::Luma::<u8>([sum as u8]);
            //let filtered_pixel = Pixel::from_channels(sum as u8, 0, 0, 0);
            //let test = Rgba::new(0,0,0,0);
            // let t = Pixel::from_channels(
            //     NumCast::from(clamp(t1, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t2, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t3, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t4, 0.0, max)).unwrap()
            // );
            result.put_pixel(col, row, filtered_pixel);
        }
    }

    // This is a bunch of whore code written by a whore
    //
    // Okay so uint8 is just rounding all the fucking negatives to 0, so attempting to scale negative
    // Numbers by a minimum convolution summation value (ie. -121) will just result in an image
    // with no real information because half of the values were set to 0 in the convolution calculation.
    // So there will be no "depth" or "content" most of that was already thrown out
    // For now I will just add by 128 above if therer is a negative.
    #[cfg(ignore)]
    {
        let mut bottom_offset = 0;
        let mut top_offset = 0;
        let mut offset_flag = false;
        // This is scuffed scaling. Will most likely be a source of a bunch of issues in the future
        if min_value < 0.0 {
            bottom_offset = min_value.abs() as u8;
            offset_flag = true;
        }

        if max_value > 256.0 {
            top_offset = (max_value - 256.0) as u8;
            offset_flag = true;
        }

        if offset_flag {
            println!("bottom_offset: {}", bottom_offset);
            println!("top_offset: {}", top_offset);
            for row in 0..result_rows {
                for col in 0..result_cols {
                    //println!("{:?}", result.get_pixel(col, row).channels().get(0));
                    let pixel = result.get_pixel(col, row).map(|v| v.wrapping_add(128));
                    result.put_pixel(col, row, pixel);
                }
            }
        }
    }
    // if max_value-min_value > 256.0 {
    //     unimplemented!("Need to do scaling thing here");
    // }

    return result;
}