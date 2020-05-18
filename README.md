# Rust+OpenCV Playground

Requires:
* OpenCV 4.3
* Webcam
* Linux?

Usage:
* Have a file named `video.mp4` at the project root (not included) to be used as the background
* Execute `cargo run` from project root

## Turning it into a webcam

* Install latest (v4l2loopback)[https://github.com/umlaeute/v4l2loopback/]
  * You may use the following command to start a new virtual webcam: `insmod v4l2loopback.ko devices=1 max_buffers=16 exclusive_caps=1 card_label="VirtualCam #0"` 
* Make sure you have `gst-launch-1.0` and `wmctrl` available
* Start this app
  * Two windows will open, one with threshold control and another named `WEBCAM` that you shall not change
* Call `wmctrl -l` to get the `WEBCAM` window ID
* Call `gst-launch-1.0 ximagesrc use-damage=false xid=0x04c0000d starty=65 endy=485 ! videoconvert ! "video/x-raw,format=YUY2" ! v4l2sink device=/dev/videoN` to start streaming the content from `WEBCAM` into your virtual webcam
* ???
* Profit!
