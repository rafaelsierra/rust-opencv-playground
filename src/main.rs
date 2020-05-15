extern crate opencv;

use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, video, videoio};

fn run() -> opencv::Result<()> {
    let window = "video capture";
    let mask_window = "bg mask";
    let bla = "data/haarcascades/haarcascade_upperbody.xml";

    let mut cam = videoio::VideoCapture::new_default(0)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    //let mut bg = video::create_background_subtractor_mog2(200, 32.0, false)?;
    let mut bg_learning = -1.0;

    //highgui::named_window(window, 1)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        //let mut mask = core::Mat::default()?;
        let mut edges = core::Mat::default()?;
        let mut threshold = core::Mat::default()?;
        imgproc::threshold(&edges, &mut threshold, 127.0, 255.0, 0)?;
        let mut contours = types::VectorOfMat::new();
        cam.read(&mut frame)?;
        //bg.apply(&frame, &mut mask, bg_learning)?;
        imgproc::canny(&frame, &mut edges, 50.0, 100.0, 3, true)?;
        imgproc::find_contours(&mut edges, &mut contours, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_SIMPLE, core::Point::new(0, 0))?;
        if frame.size()?.width > 0 {
            //highgui::imshow(window, &mut frame)?;
            //highgui::imshow(mask_window, &mut mask)?;
            highgui::imshow("Edges", &mut edges)?;

            //let mut im_cont = core::Mat::default()?;
            imgproc::draw_contours(
                &mut frame,
                &contours,
                -1,
                core::Scalar::new(0.0, 0.0, 255.0, 255.0),
                -1,
                0,
                &core::Mat::default()?,
                0,
                core::Point::new(0, 0),
            )?;
            highgui::imshow("Contour", &mut frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key == 113 {
            break;
        }
        if key == 98 {
            if bg_learning > 0.0 {
                bg_learning = 0.0;
            } else {
                bg_learning = 0.5
            }
            println!("Background learning is now {}", bg_learning);
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
