extern crate opencv;

use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, video, videoio};
use std;

fn run() -> opencv::Result<()> {
    let window = "video capture";

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;

    let mut thrs: f64 = 60.0;
    let mut max_thrs: f64 = 255.0;

    let red = core::Scalar::new(0.0, 0.0, 255.0, 0.0);
    let blue = core::Scalar::new(255.0, 0.0, 0.0, 0.0);
    let green = core::Scalar::new(0.0, 255.0, 0.0, 0.0);

    let roboto = highgui::font_qt(
        "Roboto",
        12,
        blue,
        highgui::QT_FONT_NORMAL,
        highgui::QT_STYLE_NORMAL,
        0,
    )?;
    //highgui::named_window(window, 1)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    highgui::named_window(window, highgui::WINDOW_NORMAL)?;
    loop {
        let mut frame = core::Mat::default()?;
        let mut contours = types::VectorOfMat::new();
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            let mut inv_bw = core::Mat::default()?;
            imgproc::cvt_color(&frame, &mut inv_bw, imgproc::COLOR_RGB2GRAY, 1)?;
            let mut blur = core::Mat::default()?;
            let mut bw = core::Mat::copy(&inv_bw)?;
            core::bitwise_not(&inv_bw, &mut bw, &core::Mat::default()?)?;
            imgproc::blur(
                &bw,
                &mut blur,
                core::Size::new(3, 3),
                core::Point::new(-1, -1),
                core::BORDER_DEFAULT,
            )?;
            let mut bw = core::Mat::copy(&inv_bw)?;
            core::bitwise_not(&inv_bw, &mut bw, &core::Mat::default()?)?;
            highgui::imshow("BW", &mut bw)?;
            highgui::imshow("INV BW", &mut blur)?;

            let mut threshold = core::Mat::default()?;
            imgproc::threshold(
                &blur,
                &mut threshold,
                thrs,
                max_thrs,
                imgproc::THRESH_BINARY,
            )?;
            imgproc::find_contours(
                &mut threshold,
                &mut contours,
                imgproc::RETR_TREE,
                imgproc::CHAIN_APPROX_NONE,
                core::Point::new(0, 0),
            )?;

            let mut hulls = types::VectorOfMat::new();
            //highgui::imshow(window, &mut frame)?;
            for contour in contours.iter() {
                let mut hull = core::Mat::default()?;
                imgproc::convex_hull(&contour, &mut hull, false, true)?;
                hulls.push(hull);
            }
            //for (i, contour) in contours.iter().enumerate() {
                imgproc::draw_contours(
                    &mut frame,
                    &contours,
                    -1,
                    red,
                    1,
                    8,
                    &core::Mat::default()?,
                    std::i32::MAX,
                    core::Point::new(0, 0),
                )?;

                imgproc::draw_contours(
                    &mut frame,
                    &hulls,
                    -1,
                    green,
                    1,
                    8,
                    &core::Mat::default()?,
                    std::i32::MAX,
                    core::Point::new(2, 2),
                )?;
           // }

            //highgui::imshow("TS", &mut threshold)?;

            highgui::add_text(
                &mut frame,
                format!("Threshold: {}", thrs).as_str(),
                core::Point::new(10, 30),
                &roboto,
            )?;
            highgui::add_text(
                &mut frame,
                format!("Max Threshold: {}", max_thrs).as_str(),
                core::Point::new(10, 60),
                &roboto,
            )?;
            highgui::imshow(window, &mut frame)?;
            //highgui::imshow("BW", &mut bw)?;
        }
        let key = highgui::wait_key(10)?;
        if key == 113 {
            break;
        }
        if key == 82 {
            thrs += 1.0;
        }
        if key == 84 {
            thrs -= 1.0;
        }
        if key == 81 {
            max_thrs -= 1.0;
        }
        if key == 83 {
            max_thrs += 1.0;
        }

        if key >= 0 {
            println!("Key: {}", key);
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
