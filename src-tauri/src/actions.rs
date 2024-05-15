use base64::encode;
use opencv::core::Rect;
use opencv::{core, highgui, imgcodecs, imgproc, objdetect, prelude::*, types, videoio};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::thread;
use std::time::Duration;
use tauri::Window;
use winapi::um::winuser::{FindWindowW, ShowWindow, SW_MINIMIZE};

use crate::config::{Config, CONFIG};

#[tauri::command]
pub fn set_config(config: Config) -> Result<String, String> {
    println!("配置{:?}", config);

    let mut old = CONFIG.lock().unwrap();
    old.one=config.one;
    old.two=config.two;
    old.three=config.three;
    Ok("".to_string())
}

pub fn link_start(window: &Window) {
    let mut face_cascade =
        objdetect::CascadeClassifier::new("haarcascade_frontalface_default.xml").unwrap();
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    cam.set(videoio::CAP_PROP_FPS, 10.0).unwrap();

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame).unwrap();

        if frame.size().unwrap().width > 0 {
            let mut gray = Mat::default();
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

            let mut faces = types::VectorOfRect::new();
            face_cascade
                .detect_multi_scale(
                    &gray,
                    &mut faces,
                    1.3,
                    5,
                    0,
                    core::Size::new(0, 0),
                    core::Size::new(0, 0),
                )
                .unwrap();

            for face in &faces {
                let rect = Rect::new(face.x, face.y, face.x + face.width, face.y + face.height);
                imgproc::rectangle(
                    &mut frame,
                    rect,
                    core::Scalar::new(255.0, 0.0, 0.0, 0.0),
                    2,
                    8,
                    0,
                )
                .unwrap();
            }

            if faces.len() > 0 {
                println!("找到 {} 个脸!", faces.len());
                hand_face(faces.len());
            }
            let mut buf = types::VectorOfu8::new();
            imgcodecs::imencode(".png", &frame, &mut buf, &types::VectorOfi32::new()).unwrap();
            let base64_string = encode(&buf);
            let _ = window.emit("update_img", Some(base64_string));
        }
        thread::sleep(Duration::from_millis(100));
    }

    // let _ = cam.release();
}

fn hand_face(num: usize) {
    let config=CONFIG.lock().unwrap();
    match num {
        _ if num >= 3 => {
          config.three.iter().for_each(|ci|{
            if ci.window_name.len()>0{

            }
          })
        }
        _ if num >= 2 => {
            config.three.iter().for_each(|ci|{
                if ci.window_name.len()>0{
    
                }
              })
        }
        _ if num >= 1 => {
            config.one.iter().for_each(|ci|{
                println!("{:?}",ci);
                if ci.window_name.len()>0{
                    println!("对{}执行{}",ci.window_name,ci.oparetion);
                    minimize_window(ci.window_name.as_str());
                }
              })
        }
        _ => {}
    }
}

fn minimize_window(window_name: &str) {
    // let class_name_w = to_wide_chars(class_name);
    let window_name_w = to_wide_chars(window_name);

    unsafe {
        let hwnd = FindWindowW(ptr::null(), window_name_w.as_ptr());
        if hwnd != ptr::null_mut() {
            ShowWindow(hwnd, SW_MINIMIZE);
        } else {
            println!("Window not found!");
        }
    }
}

fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}
