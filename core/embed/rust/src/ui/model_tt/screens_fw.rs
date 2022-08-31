#[cfg(feature = "micropython")]
use crate::micropython::buffer::StrBuffer;

#[cfg(feature = "sd_card")]
use crate::ui::{
    component::{text::paragraphs::Paragraphs, PageMsg},
    layout::native::RustLayout,
    model_tt::{
        component::{Button, CancelConfirmMsg, Frame, SwipePage},
        theme,
    },
};

#[cfg(all(feature = "sd_card", not(feature = "sd_card_hotswap")))]
use crate::ui::component::image::BlendedImage;
#[cfg(all(feature = "sd_card", not(feature = "sd_card_hotswap")))]
use crate::ui::{component::base::ComponentExt, model_tt::component::ButtonMsg};
use crate::ui::{
    component::text::paragraphs::Paragraph,
    display,
    layout::simplified::show,
    model::{
        component::{FrameMsg, WelcomeScreen},
        theme::BACKLIGHT_NORMAL,
    },
};

#[cfg(all(feature = "sd_card", not(feature = "sd_card_hotswap")))]
use crate::ui::model_tt::component::IconDialog;

#[cfg(all(feature = "sd_card", feature = "sd_card_hotswap"))]
pub fn insert_sd_card() -> bool {
    let source = [
        Paragraph::new(&theme::TEXT_BOLD, StrBuffer::from("SD card required.")),
        Paragraph::new(
            &theme::TEXT_NORMAL,
            StrBuffer::from("Please insert your SD card."),
        ),
    ];
    let paragraphs = Paragraphs::new(source);

    let buttons = Button::cancel_confirm_text(Some("Abort"), Some("Retry"));
    let mut layout = RustLayout::new(Frame::centered(
        theme::label_title(),
        "SD card protection",
        SwipePage::new(paragraphs, buttons, theme::BG),
    ));
    let res = layout.process();

    !matches!(
        res,
        FrameMsg::Content(PageMsg::Controls(CancelConfirmMsg::Cancelled))
    )
}

#[cfg(all(feature = "sd_card", not(feature = "sd_card_hotswap")))]
pub fn insert_sd_card() -> bool {
    let mut dialog = RustLayout::new(
        IconDialog::new(
            BlendedImage::new(
                theme::IMAGE_BG_CIRCLE,
                theme::IMAGE_FG_ERROR,
                theme::ERROR_COLOR,
                theme::FG,
                theme::BG,
            ),
            StrBuffer::from("Please unplug the device and insert your SD card."),
            theme::button_bar(
                Button::with_text("CLOSE")
                    .styled(theme::button_default())
                    .map(|msg| {
                        (matches!(msg, ButtonMsg::Clicked)).then(|| CancelConfirmMsg::Confirmed)
                    }),
            ),
        )
        .with_description(StrBuffer::from("SD card required.")),
    );

    dialog.process();

    false
}

#[cfg(all(feature = "sd_card", feature = "sd_card_hotswap"))]
pub fn retry_wrong_card() -> bool {
    let source = [
        Paragraph::new(&theme::TEXT_BOLD, StrBuffer::from("Wrong SD card.")),
        Paragraph::new(
            &theme::TEXT_NORMAL,
            StrBuffer::from("Please retry your SD card."),
        ),
    ];
    let paragraphs = Paragraphs::new(source);

    let buttons = Button::cancel_confirm_text(Some("Abort"), Some("Retry"));
    let mut layout = RustLayout::new(Frame::centered(
        theme::label_title(),
        "SD card protection",
        SwipePage::new(paragraphs, buttons, theme::BG),
    ));

    let res = layout.process();

    !matches!(
        res,
        FrameMsg::Content(PageMsg::Controls(CancelConfirmMsg::Cancelled))
    )
}

#[cfg(all(feature = "sd_card", not(feature = "sd_card_hotswap")))]
pub fn retry_wrong_card() -> bool {
    let mut dialog = RustLayout::new(
        IconDialog::new(
            BlendedImage::new(
                theme::IMAGE_BG_CIRCLE,
                theme::IMAGE_FG_ERROR,
                theme::ERROR_COLOR,
                theme::FG,
                theme::BG,
            ),
            StrBuffer::from("Please unplug the device and insert the correct SD card."),
            theme::button_bar(
                Button::with_text("CLOSE")
                    .styled(theme::button_default())
                    .map(|msg| {
                        (matches!(msg, ButtonMsg::Clicked)).then(|| CancelConfirmMsg::Confirmed)
                    }),
            ),
        )
        .with_description(StrBuffer::from("Wrong SD card.")),
    );

    dialog.process();

    false
}

#[cfg(feature = "sd_card")]
pub fn retry_sd_card() -> bool {
    let source = [
        Paragraph::new(&theme::TEXT_BOLD, StrBuffer::from("SD card fail.")),
        Paragraph::new(
            &theme::TEXT_NORMAL,
            StrBuffer::from("Please retry your SD card."),
        ),
    ];
    let paragraphs = Paragraphs::new(source);

    let buttons = Button::cancel_confirm_text(Some("Abort"), Some("Retry"));
    let mut layout = RustLayout::new(Frame::centered(
        theme::label_title(),
        "SD card protection",
        SwipePage::new(paragraphs, buttons, theme::BG),
    ));
    let res = layout.process();

    !matches!(
        res,
        FrameMsg::Content(PageMsg::Controls(CancelConfirmMsg::Cancelled))
    )
}

#[no_mangle]
extern "C" fn screen_welcome_model() {
    let mut frame = WelcomeScreen::new(false);
    show(&mut frame, false);
    display::set_backlight(BACKLIGHT_NORMAL);
}
