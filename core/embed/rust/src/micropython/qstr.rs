#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use crate::micropython::obj::Obj;

impl Qstr {
    pub const fn to_obj(self) -> Obj {
        // SAFETY:
        //  - Micropython compiled with `MICROPY_OBJ_REPR == MICROPY_OBJ_REPR_A`.
        //    micropython/py/obj.h #define MP_OBJ_NEW_QSTR(qst)
        //    ((mp_obj_t)((((mp_uint_t)(qst)) << 3) | 2))
        let bits = (self.0 << 3) | 2;
        unsafe { Obj::from_bits(bits as usize) }
    }

    pub const fn to_u16(self) -> u16 {
        // TODO: Change the internal representation of Qstr to u16.
        self.0 as _
    }
}

include!(concat!(env!("OUT_DIR"), "/qstr.rs"));
