use opencv::{
	core::{Point, Size, VecN},
	highgui,
	imgproc::*,
	prelude::*,
	types::VectorOfVec3f,
	videoio, Result,
};

fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
	(x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

fn main() -> Result<()> {
	let window = "video capture";

	highgui::named_window(window, highgui::WINDOW_FULLSCREEN)?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}

	let mut prev_circle: Option<VecN<f32, 3>> = None;

	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;

		let mut grey_frame = Mat::default();
		cvt_color(&frame, &mut grey_frame, COLOR_BGR2GRAY, 0)?;

		let mut blur_frame = Mat::default();
		gaussian_blur(
			&grey_frame,
			&mut blur_frame,
			Size {
				width: 17,
				height: 17,
			},
			0.0,
			0.0,
			0,
		)?;

		let mut circles = VectorOfVec3f::new();
		hough_circles(
			&blur_frame,
			&mut circles,
			HOUGH_GRADIENT_ALT,
			1.5,
			1.0,
			50.0,
			0.9,
			50,
			0,
		)?;

		if !circles.is_empty() {
			let mut chosen: Option<VecN<f32, 3>> = None;
			for circle in circles {
				if chosen.is_none() {
					chosen = Some(circle);
				}
				let _chosen = chosen.unwrap();

				if prev_circle.is_some() {
					let _prev_circle = prev_circle.unwrap();

					if dist(_chosen[0], _chosen[1], _prev_circle[0], _prev_circle[1])
						<= dist(circle[0], circle[1], _prev_circle[0], _prev_circle[1])
					{
						chosen = Some(circle);
					}
				}
			}
			let _chosen = chosen.unwrap();

			circle(
				&mut frame,
				Point {
					x: _chosen[0].round() as i32,
					y: _chosen[1].round() as i32,
				},
				1,
				VecN([0.0, 100.0, 100.0, 1.0]),
				3,
				FILLED,
				0,
			)?;
			circle(
				&mut frame,
				Point {
					x: _chosen[0].round() as i32,
					y: _chosen[1].round() as i32,
				},
				_chosen[2].round() as i32,
				VecN([0.0, 255.0, 255.0, 1.0]),
				3,
				FILLED,
				0,
			)?;

			prev_circle = chosen;
		}

		highgui::imshow(window, &frame)?;

		let key = highgui::wait_key(10)?;
		if key == 27 {
			break;
		}
	}

	Ok(())
}
