use cstr_core::CStr;
use heapless::String;
use crate::error::Error;
use crate::micropython::map::Map;
use crate::micropython::obj::Obj;
use crate::micropython::util;
use crate::micropython::qstr::Qstr;
use crate::ui::constant::screen;
use crate::ui::display;
use crate::ui::geometry::{Point, Rect};
use crate::ui::model_tt::{constant, theme};

static mut PREV_SECONDS: u32 = 0xFFFFFFFF;
static mut PREV_PROGRESS: u32 = 0xFFFFFFFF;
static mut KEEPALIVE_CALLBACK: Option<Obj> = None;


pub unsafe extern "C" fn show_pin_timeout(seconds: u32, progress: u32, message: *const u8) -> u32 {
    unsafe {

        if let Some(callback) = KEEPALIVE_CALLBACK {
            //todo
            unwrap!(callback.call_with_n_args(&[]));
            //keepalive_callback()
        };


        if progress == 0 {
            if progress != PREV_PROGRESS {
                display::rect_fill(screen(), theme::BG);
                PREV_SECONDS = 0xFFFFFFFF;
            }

            let msg = CStr::from_ptr(message as _).to_str().unwrap();
            display::text_center(Point::new(screen().center().x, 37), msg, theme::FONT_BOLD, theme::FG, theme::BG);
        }

        if progress != PREV_PROGRESS {
            display::loader(progress as _, 0, theme::FG, theme::BG, None);
        }

        let mut s : String<16> = String::new();

        if seconds != PREV_SECONDS {
            match seconds {
                0 => {unwrap!(s.push_str("Done"))}
                1 => {unwrap!(s.push_str("1 second left"))}
                _ => {
                    let sec: String<16> = String::from(seconds);
                    unwrap!(s.push_str(sec.as_str()));
                    unwrap!(s.push_str(" seconds left"))
                }
            };

            display::rect_fill(Rect::new(
                Point::new(0, constant::HEIGHT - 42),
                Point::new(constant::WIDTH, constant::HEIGHT - 42 + 25)),
                               theme::BG,
            );
            display::text_center(Point::new(screen().center().x, constant::HEIGHT - 22),
                                 s.as_str(),
                                 theme::FONT_BOLD,
                                 theme::FG,
                                 theme::BG,
            );
        }

        display::pixeldata_dirty();

        PREV_SECONDS = seconds;
        PREV_PROGRESS = progress;

    }
    0
}


pub extern "C" fn set_keepalive_callback(n_args: usize, args: *const Obj, kwargs: *mut Map) -> Obj {

    let block = move |_args: &[Obj], kwargs: &Map| unsafe {
        let cb = kwargs.get(Qstr::MP_QSTR_keepalive_callback)?.try_into()?;
        KEEPALIVE_CALLBACK = Some(cb);
        Ok(cb)
    };
    unsafe { util::try_with_args_and_kwargs(n_args, args, kwargs, block) }

}

// pub extern "C" fn remove_keepalive_callback()  {
//     let block = || unsafe {
//         KEEPALIVE_CALLBACK = None;
//         Ok(())
//     };
//     unsafe { util::try_or_raise(block) }
// }