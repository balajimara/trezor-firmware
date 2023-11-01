/*
 * This file is part of the Trezor project, https://trezor.io/
 *
 * Copyright (c) SatoshiLabs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include STM32_HAL_H

#include "flash_otp.h"
#include "common.h"
#include "flash.h"


#define FLASH_STATUS_ALL_FLAGS \
  (FLASH_NSSR_PGSERR | FLASH_NSSR_PGAERR | FLASH_NSSR_WRPERR | FLASH_NSSR_EOP)


void flash_otp_init() {
  // intentionally left empty
}

uint32_t flash_wait_and_clear_status_flags(void) {
  while (FLASH->NSSR & FLASH_NSSR_BSY)
    ;  // wait for all previous flash operations to complete

  uint32_t result =
      FLASH->NSSR & FLASH_STATUS_ALL_FLAGS;  // get the current status flags
  FLASH->NSSR |= FLASH_STATUS_ALL_FLAGS;     // clear all status flags

#if defined(__ARM_FEATURE_CMSE) && (__ARM_FEATURE_CMSE == 3U)
  while (FLASH->SECSR & FLASH_SECSR_BSY)
    ;  // wait for all previous flash operations to complete
  result |=
      FLASH->SECSR & FLASH_STATUS_ALL_FLAGS;  // get the current status flags
  FLASH->SECSR |= FLASH_STATUS_ALL_FLAGS;     // clear all status flags
#endif
  return result;
}

secbool flash_otp_read(uint8_t block, uint8_t offset, uint8_t *data,
                       uint8_t datalen) {
  if (block >= FLASH_OTP_NUM_BLOCKS ||
      offset + datalen > FLASH_OTP_BLOCK_SIZE) {
    return secfalse;
  }
  for (uint8_t i = 0; i < datalen; i++) {
    data[i] = *(__IO uint8_t *)(FLASH_OTP_BASE + block * FLASH_OTP_BLOCK_SIZE +
                                offset + i);
  }
  return sectrue;
}

secbool flash_otp_write(uint8_t block, uint8_t offset, const uint8_t *data,
                        uint8_t datalen) {
  if (datalen % 16 != 0) {
    return secfalse;
  }
  if (block >= FLASH_OTP_NUM_BLOCKS ||
      offset + datalen > FLASH_OTP_BLOCK_SIZE) {
    return secfalse;
  }
  ensure(flash_unlock_write(), NULL);
  for (uint8_t i = 0; i < datalen; i++) {
    uint32_t address =
        FLASH_OTP_BASE + block * FLASH_OTP_BLOCK_SIZE + offset + i;
    ensure(sectrue * (HAL_OK == HAL_FLASH_Program(FLASH_TYPEPROGRAM_QUADWORD,
                                                  address, (uint32_t)data)),
           NULL);
  }
  ensure(flash_lock_write(), NULL);
  return sectrue;
}

secbool flash_otp_lock(uint8_t block) {
  // check that all quadwords in the block have been written to
  volatile uint8_t *addr =
      (__IO uint8_t *)(FLASH_OTP_BASE + block * FLASH_OTP_BLOCK_SIZE);

  secbool qw_locked = secfalse;
  for (uint8_t i = 0; i < FLASH_OTP_BLOCK_SIZE; i++) {
    if (addr[i] != 0xFF) {
      qw_locked = sectrue;
    }
    if (i % 16 == 15 && qw_locked == secfalse) {
      return secfalse;
    }
  }
  return sectrue;
}

secbool flash_otp_is_locked(uint8_t block) {
  // considering block locked if any quadword in the block is non-0xFF
  volatile uint8_t *addr =
      (__IO uint8_t *)(FLASH_OTP_BASE + block * FLASH_OTP_BLOCK_SIZE);

  for (uint8_t i = 0; i < FLASH_OTP_BLOCK_SIZE; i++) {
    if (addr[i] != 0xFF) {
      return sectrue;
    }
  }
  return secfalse;
}
