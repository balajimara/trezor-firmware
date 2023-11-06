mod en;
#[cfg(feature = "micropython")]
mod export;
mod general;
#[cfg(feature = "micropython")]
mod micropython;

use en::EN_TRANSLATIONS;

use crate::trezorhal::translations::{get_pointer_with_offset, translations_get, PointerData};
use core::str;

// Translations strings are delimited by a star
const DELIMITER_BYTE: u8 = 0x00;
const TERMINATE_BYTE: u8 = 0xFF;
const HEADER_LEN: usize = 256;

/// Translation function for Rust.
pub fn tr(key: &str) -> &'static str {
    translate(key).unwrap_or_default()
}

// TODO: somehow make it a singleton object, so it is loaded just once

struct TranslationsHeader {
    pub language: &'static str,
    pub version: &'static str,
    pub data_length: u16,
    pub translations_length: u16,
    pub translations_num: u16,
    pub data_hash: [u8; 32],
}

impl TranslationsHeader {
    const MAGIC: [u8; 4] = [84, 82, 84, 82]; // b"TRTR"
    const LANG_LEN: usize = 32;
    const VERSION_LEN: usize = 16;
    const DATA_HASH_LEN: usize = 32;

    pub fn from_flash() -> Result<Self, &'static str> {
        let header_data = &translations_get()[..HEADER_LEN];
        Self::from_bytes(header_data)
    }

    pub fn from_bytes(data: &'static [u8]) -> Result<Self, &'static str> {
        if data.len() != HEADER_LEN {
            return Err("Invalid header length");
        }

        let (magic, rest) = data.split_at(4);
        if magic != Self::MAGIC {
            return Err("Invalid header magic");
        }

        let (version, rest) = rest.split_at(Self::VERSION_LEN);
        let version = core::str::from_utf8(version)
            .or(Err("Invalid version string"))?
            .trim_end_matches(0 as char);

        let (language, rest) = rest.split_at(Self::LANG_LEN);
        let language = core::str::from_utf8(language)
            .or(Err("Invalid language string"))?
            .trim_end_matches(0 as char);

        let (data_length_bytes, rest) = rest.split_at(2);
        let data_length = u16::from_le_bytes(
            data_length_bytes
                .try_into()
                .or(Err("Invalid data length bytes"))?,
        );

        let (translations_length_bytes, rest) = rest.split_at(2);
        let translations_length = u16::from_le_bytes(
            translations_length_bytes
                .try_into()
                .or(Err("Invalid translations length bytes"))?,
        );

        let (translations_num_bytes, rest) = rest.split_at(2);
        let translations_num = u16::from_le_bytes(
            translations_num_bytes
                .try_into()
                .or(Err("Invalid translations num bytes"))?,
        );

        let (data_hash, rest) = rest.split_at(Self::DATA_HASH_LEN);
        let data_hash: [u8; 32] = data_hash.try_into().or(Err("Invalid data hash length"))?;

        // Rest must be empty bytes
        for byte in rest {
            if *byte != 0 {
                return Err("Invalid header data");
            }
        }

        Ok(Self {
            language,
            version,
            data_length,
            translations_length,
            translations_num,
            data_hash,
        })
    }
}

/// Get the language name.
fn get_language_name() -> Option<&'static str> {
    TranslationsHeader::from_flash()
        .ok()
        .map(|header| header.language)
}

/// Try to find the translation in flash (for a non-english language).
/// If not found, fallback to english.
fn translate(key: &str) -> Option<&'static str> {
    let mut translation: Option<&'static str> = None;

    if are_there_translations() {
        if let Some(index) = EN_TRANSLATIONS.get_position(key) {
            translation = get_translation_by_index(index);
        }
    }

    if translation.is_none() {
        translation = EN_TRANSLATIONS.get_text(key);
    }

    translation
}

/// Quickly checks whether there are some valid translations data
fn are_there_translations() -> bool {
    translations_get()[0] != TERMINATE_BYTE
}

/// Loops through all the translations data in flash and
/// returns the translation at the given index.
fn get_translation_by_index(index: usize) -> Option<&'static str> {
    let mut current_index = 0;
    let mut chunk_start = 0;

    let translations_data = &translations_get()[HEADER_LEN..];

    for (i, &byte) in translations_data.iter().enumerate() {
        if byte == TERMINATE_BYTE {
            return None;
        }
        if byte == DELIMITER_BYTE {
            if current_index == index {
                return str::from_utf8(&translations_data[chunk_start..i]).ok();
            }
            chunk_start = i + 1;
            current_index += 1;
            continue;
        }
    }
    None
}

pub fn get_font_data() -> &'static [u8] {
    if let Ok(header) = TranslationsHeader::from_flash() {
        let translations_data = &translations_get()[HEADER_LEN..];
        &translations_data[header.translations_length as usize..header.data_length as usize]
    } else {
        &[]
    }
}

pub struct FontData {
    pub offset: u16,
    pub len: u16,
}

fn get_font_offset(char_code: u16) -> Option<FontData> {
    let font_data = get_font_data();
    let data_size = font_data.len() as u16;

    let (font_amount_bytes, data) = font_data.split_at(2);
    let font_amount = u16::from_le_bytes(unwrap!(font_amount_bytes.try_into()));

    const FONT_ENTRY_SIZE: usize = 4;

    for i in 0..font_amount {
        let current_data = &data[(i as usize * FONT_ENTRY_SIZE)..];
        let char_code_bytes = &current_data[..2];
        let code = u16::from_le_bytes(unwrap!(char_code_bytes.try_into()));

        if code == char_code {
            let font_size_bytes = &current_data[2..4];
            let font_offset = u16::from_le_bytes(unwrap!(font_size_bytes.try_into()));
            let len = if i == font_amount {
                data_size - font_offset
            } else {
                let next_data = &data[((i + 1) as usize * FONT_ENTRY_SIZE)..];
                let next_font_offset = u16::from_le_bytes(unwrap!(next_data[2..4].try_into()));
                next_font_offset - font_offset
            };
            return Some(FontData {
                offset: font_offset,
                len,
            });
        }
    }
    None
}

#[no_mangle]
pub extern "C" fn get_utf8_glyph(char_code: cty::uint16_t) -> PointerData {
    let font_data = get_font_offset(char_code);
    if let Some(font_data) = font_data {
        let font_start_offset = TranslationsHeader::from_flash()
            .map_or(0, |header| HEADER_LEN as u16 + header.translations_length);

        get_pointer_with_offset(font_start_offset + font_data.offset, font_data.len)
    } else {
        PointerData {
            ptr: core::ptr::null(),
            len: 0,
        }
    }
}

// TODO: add some tests?
