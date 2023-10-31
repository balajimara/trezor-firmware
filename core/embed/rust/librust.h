#include "py/obj.h"

#include "librust_qstr.h"

#ifdef TREZOR_EMULATOR
mp_obj_t protobuf_debug_msg_type();
mp_obj_t protobuf_debug_msg_def_type();
#endif

extern mp_obj_module_t mp_module_trezorproto;
extern mp_obj_module_t mp_module_trezorui2;
extern mp_obj_module_t mp_module_trezortranslate;

#ifdef TREZOR_EMULATOR
mp_obj_t ui_debug_layout_type();
#endif

typedef struct {
  const uint8_t* ptr;
  uint32_t len;
} PointerData;

PointerData get_utf8_glyph(uint16_t char_code);
