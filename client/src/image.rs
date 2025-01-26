use image::{
	codecs::{gif::GifDecoder, png::PngDecoder, webp::WebPDecoder},
	imageops::FilterType,
	AnimationDecoder, DynamicImage, ImageFormat, ImageReader,
};
use std::{
	error::Error,
	fs::File,
	io::{BufRead, BufReader, Seek},
	time::Duration,
};

pub fn check_format<T: BufRead + Seek>(filedata: T) -> Result<ImageFormat, Box<dyn Error>>
{
	ImageReader::new(filedata).with_guessed_format()?.format().ok_or("failed to determine image type".into())
}

pub fn read_image(filename: String, pair: bool) -> Result<Vec<(Vec<u8>, Duration)>, Box<dyn Error>>
{
	let mut filedata = BufReader::new(File::open(&filename)?);

	match check_format(&mut filedata)?
	{
		ImageFormat::Png =>
		{
			let decoder = PngDecoder::new(&mut filedata)?;

			if decoder.is_apng().unwrap_or_default()
			{
				read_animated(decoder.apng()?, pair)
			}
			else
			{
				filedata.rewind()?;
				read_unanimated(filedata, pair)
			}
		},
		ImageFormat::WebP => read_animated(WebPDecoder::new(filedata)?, pair),
		ImageFormat::Gif => read_animated(GifDecoder::new(filedata)?, pair),
		_ => read_unanimated(filedata, pair),
	}
}

fn read_unanimated<T: BufRead + Seek>(filedata: T, pair: bool) -> Result<Vec<(Vec<u8>, Duration)>, Box<dyn Error>>
{
	let width = match pair
	{
		true => 18,
		false => 9,
	};

	Ok(vec![(
		to_column_major(
			ImageReader::new(filedata).with_guessed_format()?
			                          .decode()?
			                          .resize_exact(width, 34, FilterType::Nearest)
			                          .grayscale()
			                          .to_luma8()
			                          .to_vec(),
			width as usize,
			34,
		),
		Duration::default(),
	)])
}

fn read_animated<'a, T: AnimationDecoder<'a>>(decoder: T,
                                              pair: bool)
                                              -> Result<Vec<(Vec<u8>, Duration)>, Box<dyn Error>>
{
	let width = match pair
	{
		true => 18,
		false => 9,
	};

	let frames = decoder.into_frames().collect_frames()?;
	let mut result: Vec<(Vec<u8>, Duration)> = Vec::with_capacity(frames.len());
	for frame in frames
	{
		let (numerator, denominator) = frame.delay().numer_denom_ms();
		let duration = Duration::from_millis((numerator as u64) / (denominator as u64));

		result.push((
			to_column_major(
				DynamicImage::ImageRgba8(frame.into_buffer()).resize_exact(width, 34, FilterType::Nearest)
				                                             .grayscale()
				                                             .to_luma8()
				                                             .to_vec(),
				width as usize,
				34,
			),
			duration,
		));
	}

	Ok(result)
}

fn to_column_major(frame: Vec<u8>, width: usize, height: usize) -> Vec<u8>
{
	let mut result: Vec<u8> = vec![0; width * 34];

	for (index, val) in frame.iter().enumerate()
	{
		let col = index % width;
		let row = index / width;

		result[(col * height) + row] = *val;
	}

	result
}
