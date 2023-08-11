// NOTE: things needing micropython do so mostly because of translations
// (we cannot have them inside bootloader)

#[cfg(feature = "micropython")]
mod address_details;
mod button;
#[cfg(feature = "micropython")]
mod coinjoin_progress;
mod dialog;
mod fido;
#[rustfmt::skip]
mod fido_icons;
mod error;
mod frame;
#[cfg(feature = "micropython")]
mod hold_to_confirm;
#[cfg(feature = "dma2d")]
mod homescreen;
mod horizontal_page;
mod keyboard;
mod loader;
#[cfg(feature = "micropython")]
mod number_input;
#[cfg(feature = "micropython")]
mod page;
mod progress;
mod result;
mod scroll;
mod swipe;
mod welcome_screen;

#[cfg(feature = "micropython")]
pub use address_details::AddressDetails;
pub use button::{
    Button, ButtonContent, ButtonMsg, ButtonStyle, ButtonStyleSheet, CancelConfirmMsg,
    CancelInfoConfirmMsg, IconText, SelectWordMsg,
};
#[cfg(feature = "micropython")]
pub use coinjoin_progress::CoinJoinProgress;
pub use dialog::{Dialog, DialogMsg, IconDialog};
pub use error::ErrorScreen;
pub use fido::{FidoConfirm, FidoMsg};
pub use frame::{Frame, FrameMsg};
#[cfg(feature = "micropython")]
pub use hold_to_confirm::{HoldToConfirm, HoldToConfirmMsg};
#[cfg(feature = "dma2d")]
pub use homescreen::{Homescreen, HomescreenMsg, Lockscreen};
pub use horizontal_page::HorizontalPage;
pub use keyboard::{
    bip39::Bip39Input,
    mnemonic::{MnemonicInput, MnemonicKeyboard, MnemonicKeyboardMsg},
    passphrase::{PassphraseKeyboard, PassphraseKeyboardMsg},
    pin::{PinKeyboard, PinKeyboardMsg},
    slip39::Slip39Input,
    word_count::{SelectWordCount, SelectWordCountMsg},
};
pub use loader::{Loader, LoaderMsg, LoaderStyle, LoaderStyleSheet};
#[cfg(feature = "micropython")]
pub use number_input::{NumberInputDialog, NumberInputDialogMsg};
#[cfg(feature = "micropython")]
pub use page::{SwipeHoldPage, SwipePage};
pub use progress::Progress;
pub use result::{ResultFooter, ResultScreen, ResultStyle};
pub use scroll::ScrollBar;
pub use swipe::{Swipe, SwipeDirection};
pub use welcome_screen::WelcomeScreen;

use super::theme;
