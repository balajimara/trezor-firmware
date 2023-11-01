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

#define FLASH_OTP_LOCK_BASE 0x1FFF7A00U

#define FLASH_STATUS_ALL_FLAGS                                            \
  (FLASH_SR_RDERR | FLASH_SR_PGSERR | FLASH_SR_PGPERR | FLASH_SR_PGAERR | \
   FLASH_SR_WRPERR | FLASH_SR_SOP | FLASH_SR_EOP)



void flash_otp_init() {
  // intentionally left empty
}

uint32_t flash_wait_and_clear_status_flags(void) {
  while (FLASH->SR & FLASH_SR_BSY)
    ;  // wait for all previous flash operations to complete
  const uint32_t result =
      FLASH->SR & FLASH_STATUS_ALL_FLAGS;  // get the current status flags
  FLASH->SR |= FLASH_STATUS_ALL_FLAGS;     // clear all status flags
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
  if (block >= FLASH_OTP_NUM_BLOCKS ||
      offset + datalen > FLASH_OTP_BLOCK_SIZE) {
    return secfalse;
  }
  ensure(flash_unlock_write(), NULL);
  for (uint8_t i = 0; i < datalen; i++) {
    uint32_t address =
        FLASH_OTP_BASE + block * FLASH_OTP_BLOCK_SIZE + offset + i;
    ensure(sectrue * (HAL_OK == HAL_FLASH_Program(FLASH_TYPEPROGRAM_BYTE,
                                                  address, data[i])),
           NULL);
  }
  ensure(flash_lock_write(), NULL);
  return sectrue;
}

secbool flash_otp_lock(uint8_t block) {
  if (block >= FLASH_OTP_NUM_BLOCKS) {
    return secfalse;
  }
  ensure(flash_unlock_write(), NULL);
  HAL_StatusTypeDef ret = HAL_FLASH_Program(FLASH_TYPEPROGRAM_BYTE,
                                            FLASH_OTP_LOCK_BASE + block, 0x00);
  ensure(flash_lock_write(), NULL);
  return sectrue * (ret == HAL_OK);
}

secbool flash_otp_is_locked(uint8_t block) {
  return sectrue * (0x00 == *(__IO uint8_t *)(FLASH_OTP_LOCK_BASE + block));
}
