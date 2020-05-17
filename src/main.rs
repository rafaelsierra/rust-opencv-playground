extern crate opencv;

use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, types, videoio};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;

fn run() -> opencv::Result<()> {
    let window = "video capture";
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L)?;
    let mut bg_video = videoio::VideoCapture::from_file("video.mp4", videoio::CAP_ANY)?;
    let mut thrs: i32 = 60;
    let max_thrs: f64 = 255.0;
    let mut blur_radius: i32 = 5;

    let out_path = Path::new("/dev/video2");
    let mut output = match File::create(out_path) {
        Err(why) => panic!("couldn't open {}: {}", out_path.display(), why,),
        Ok(file) => file,
    };

    let bg_video_frames = bg_video.get(videoio::CAP_PROP_FRAME_COUNT)?;

    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open default camera!");
    }
    if !videoio::VideoCapture::is_opened(&bg_video)? {
        panic!("Unable to load video.mp4");
    }
    highgui::named_window(window, highgui::WINDOW_NORMAL)?;
    highgui::create_trackbar(&"Threshold", &window, &mut thrs, 255, None)?;
    highgui::create_trackbar(&"Blur radius", &window, &mut blur_radius, 255, None)?;

    loop {
        if blur_radius <= 0 {
            blur_radius = 1;
        }
        let mut frame = core::Mat::default()?;
        let mut blur = core::Mat::default()?;
        let mut bw = core::Mat::default()?;
        let mut threshold = core::Mat::default()?;

        let mut base_bg_img = core::Mat::default()?;

        bg_video.read(&mut base_bg_img)?;
        if bg_video.get(videoio::CAP_PROP_POS_FRAMES)? >= bg_video_frames {
            bg_video.set(videoio::CAP_PROP_POS_FRAMES, 0.0)?;
        }

        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            // Converts frame to gray scale
            imgproc::cvt_color(&frame, &mut bw, imgproc::COLOR_RGB2GRAY, 1)?;
            // Blurs the image
            imgproc::blur(
                &bw,
                &mut blur,
                core::Size::new(blur_radius, blur_radius),
                core::Point::new(-1, -1),
                core::BORDER_DEFAULT,
            )?;

            // Converts the image to black and white
            imgproc::threshold(
                &blur,
                &mut threshold,
                f64::from(thrs),
                max_thrs,
                imgproc::THRESH_BINARY,
            )?;
            // Make an inverted copy of the image
            let mut inv_threshold = core::Mat::clone(&threshold)?;
            core::bitwise_not(&threshold, &mut inv_threshold, &core::Mat::default()?)?;
            // Copy foreground into inv_threshold
            let foreground = core::Mat::clone(&inv_threshold)?;
            let mut frame_clone = core::Mat::default()?;
            core::bitwise_and(&frame, &frame, &mut frame_clone, &foreground)?;
            // Places background into background
            let mut bg_img = core::Mat::default()?;
            imgproc::resize(
                &base_bg_img,
                &mut bg_img,
                frame.size()?,
                1.0,
                1.0,
                imgproc::INTER_NEAREST,
            )?;
            core::bitwise_and(&bg_img, &bg_img, &mut frame_clone, &threshold)?;
            // Show result
            //output.write(frame_clone.data()?)?;
            let mut channels = types::VectorOfMat::with_capacity(3);
            core::split(&frame_clone, &mut channels)?;
            let mut flat_frame = vec::Vec::new();
            for c in channels.iter() {
                for x in 0..c.rows() {
                    for y in 0..c.cols() {
                        flat_frame.push(*c.at_2d::<u8>(x, y)?);
                    }
                }
            }
            println!("1");
            highgui::imshow(window, &mut frame_clone)?;
        }
        let key = highgui::wait_key(10)?;
        if key == 113 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
