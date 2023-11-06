import json
import struct
from hashlib import sha256
from pathlib import Path
from typing import Any, Dict, List, TextIO, Tuple

import requests

DELIMITER = b"\x00"
MAGIC = b"TRTR"

TranslationData = Dict[str, Dict[str, str]]
HeaderData = Dict[str, str]
FontData = Dict[str, str]


def blob_from_file(file: TextIO, model: str) -> bytes:
    data = json.load(file)
    file_dir = file.name.rsplit("/", 1)[0]
    font_dir = Path(file_dir).absolute() / "fonts"
    return blob_from_dict(data, font_dir, model)


def blob_from_url(url: str) -> bytes:
    r = requests.get(url)
    r.raise_for_status()
    return r.content


def blob_from_dict(data: Dict[str, Any], font_dir: Path, model: str) -> bytes:
    header: HeaderData = data["header"]
    translations: TranslationData = data["translations"]
    font = data["font"]
    if model not in font:
        raise ValueError(f"Font for model {model} not found")
    model_font: FontData = font[model]
    return _blob_from_data(header, translations, model_font, font_dir)


def _blob_from_data(
    header: HeaderData, translations: TranslationData, font: FontData, font_dir: Path
) -> bytes:
    translations_blob, translations_num = _create_translations_blob(translations)
    font_blob = _create_font_blob(font, font_dir)

    data_blob = translations_blob + font_blob

    header_blob = _create_header_blob(
        magic=MAGIC,
        lang=header["language"],
        version=header["version"],
        data_length=len(data_blob),
        translations_length=len(translations_blob),
        translations_num=translations_num,
        data_hash=sha256(data_blob).digest(),
    )
    assert len(header_blob) == 256

    return header_blob + data_blob


def _create_font_blob(font: FontData, font_dir: Path) -> bytearray:
    """Example structure of the font dict:
    (The beginning number corresponds to the C representation of each font)
    {
      "1_FONT_NORMAL": "font_tthoves_regular_21_cs.json",
      "2_FONT_BOLD": "font_tthoves_bold_17_cs.json",
      "3_FONT_MONO": "font_robotomono_medium_20_cs.json",
      "4_FONT_BIG": null,
      "5_FONT_DEMIBOLD": "font_tthoves_demibold_21_cs.json"
    }
    """
    num_fonts: list[tuple[int, Path]] = []
    for font_name, file_name in font.items():
        if not file_name:
            continue
        file_path = font_dir / file_name
        font_num = int(font_name.split("_")[0])
        num_fonts.append((font_num, file_path))

    data_length = len(num_fonts)

    blob = bytearray()

    # Data length (2 bytes)
    blob += struct.pack("H", data_length)

    # Initialize Index Table
    # Each item has 2 bytes for font_num + 2 bytes for offset
    index_table_pos = len(blob)
    index_table_item_size = 2 + 2
    blob.extend(bytearray(index_table_item_size * data_length))

    # Append specific fonts and fill Index Table
    offset = len(blob)
    for font_num, file_path in sorted(num_fonts):
        data = _font_blob_from_file(file_path)

        # Update index table
        struct.pack_into("HH", blob, index_table_pos, font_num, offset)

        # Append character data
        blob.extend(data)

        # Update offset and index_table_pos
        offset += len(data)
        index_table_pos += index_table_item_size

    return blob


def _font_blob_from_file(json_file: Path) -> bytearray:
    json_content = json.loads(json_file.read_text())
    data_length = len(json_content)

    blob = bytearray()

    # Data length (2 bytes)
    blob += struct.pack("H", data_length)

    # Initialize Index Table
    # Each item has 2 bytes for char_code + 2 bytes for offset
    index_table_pos = len(blob)
    index_table_item_size = 2 + 2
    blob.extend(bytearray(index_table_item_size * data_length))

    # Append Character Data and fill Index Table
    offset = len(blob)
    for obj in json_content:
        utf8_char_str = obj["utf8"]
        assert len(utf8_char_str) == 4
        char_code = int(utf8_char_str, 16)
        data = bytes.fromhex(obj["data"])

        # Update index table
        struct.pack_into("HH", blob, index_table_pos, char_code, offset)

        # Append character data
        blob.extend(data)

        # Update offset and index_table_pos
        offset += len(data)
        index_table_pos += index_table_item_size

    return blob


def _create_translations_blob(translations: TranslationData) -> Tuple[bytes, int]:
    items_to_write: List[Tuple[str, str]] = []
    for section_name, section in translations.items():
        for k, v in section.items():
            if DELIMITER.decode() in v:
                raise ValueError(f"Delimiter '{DELIMITER}' found in {k}")
            name = f"{section_name}__{k}"
            items_to_write.append((name, v))

    # Sorting alphabetically according the key
    # TODO: maintain a stable order in future versions - write new entries
    # to the end
    items_to_write.sort(key=lambda x: x[0])

    data_blob = b""
    for _key, value in items_to_write:
        data_blob += value.encode()
        data_blob += DELIMITER

    # TODO: try to apply some compression of the data_blob
    return data_blob, len(items_to_write)


def _create_header_blob(
    magic: bytes,
    lang: str,
    version: str,
    data_length: int,
    translations_length: int,
    translations_num: int,
    data_hash: bytes,
) -> bytes:
    header = b""

    # Magic (4 bytes)
    header += struct.pack("4s", magic)

    # Version (16 bytes)
    header += struct.pack("16s", version.encode())

    # Language name (32 bytes)
    header += struct.pack("32s", lang.encode())

    # Data length (2 bytes)
    header += struct.pack("H", data_length)

    # Translations length (2 bytes)
    header += struct.pack("H", translations_length)

    # Translations amount (2 bytes)
    header += struct.pack("H", translations_num)

    # Data hash (32 bytes)
    header += struct.pack("32s", data_hash)

    # Fill rest with zeros
    while not len(header) == 256:
        header += struct.pack("B", 0)

    return header
