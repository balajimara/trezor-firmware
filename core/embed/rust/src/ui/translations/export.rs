//! generated from cs.rs.mako
//! (by running `make templates` in `core`)
//! do not edit manually!

use crate::micropython::{map::Map, module::Module, qstr::Qstr};

use super::translate_obj;

#[no_mangle]
#[rustfmt::skip]
pub static mp_module_trezortranslate: Module = obj_module! {
    // TODO: add function to get the language ID (e.g. "cs"/"fr")
    // - it should be either the first or last index in the list

    // TODO: add function to get all the translations keys in order
    // - so that client can validate it is sending correct keys in correct order

    /// def tr(key: Literal[
    ///     "addr_mismatch__contact_support",
    ///     "addr_mismatch__key_mismatch",
    ///     "addr_mismatch__mismatch",
    ///     "addr_mismatch__support_url",
    ///     "addr_mismatch__title",
    ///     "addr_mismatch__title_key_mismatch",
    ///     "addr_mismatch__wrong_derication_path",
    ///     "addr_mismatch__xpub_mismatch",
    ///     "address__address",
    ///     "address__public_key",
    ///     "address__title_cosigner",
    ///     "address__title_receive_address",
    ///     "address__title_yours",
    ///     "address_details__account",
    ///     "address_details__derivation_path",
    ///     "address_details__title_receive_address",
    ///     "address_details__title_receiving_to",
    ///     "authenticate__confirm_template",
    ///     "authenticate__header",
    ///     "auto_lock__change_template",
    ///     "auto_lock__title",
    ///     "backup__can_back_up_anytime",
    ///     "backup__it_should_be_backed_up",
    ///     "backup__it_should_be_backed_up_now",
    ///     "backup__new_wallet_created",
    ///     "backup__new_wallet_successfully_created",
    ///     "backup__recover_anytime",
    ///     "backup__title_backup_wallet",
    ///     "backup__title_skip",
    ///     "backup__want_to_skip",
    ///     "binance__buy",
    ///     "binance__confirm_cancel",
    ///     "binance__confirm_input",
    ///     "binance__confirm_order",
    ///     "binance__confirm_output",
    ///     "binance__order_id",
    ///     "binance__pair",
    ///     "binance__price",
    ///     "binance__quantity",
    ///     "binance__sell",
    ///     "binance__sender_address",
    ///     "binance__side",
    ///     "binance__unknown",
    ///     "bitcoin__commitment_data",
    ///     "bitcoin__confirm_locktime",
    ///     "bitcoin__create_proof_of_ownership",
    ///     "bitcoin__high_mining_fee_template",
    ///     "bitcoin__locktime_no_effect",
    ///     "bitcoin__locktime_set_to",
    ///     "bitcoin__locktime_set_to_blockheight",
    ///     "bitcoin__lot_of_change_outputs",
    ///     "bitcoin__multiple_accounts",
    ///     "bitcoin__new_fee_rate",
    ///     "bitcoin__simple_send_of",
    ///     "bitcoin__ticket_amount",
    ///     "bitcoin__title_confirm_details",
    ///     "bitcoin__title_finalize_transaction",
    ///     "bitcoin__title_high_mining_fee",
    ///     "bitcoin__title_meld_transaction",
    ///     "bitcoin__title_modify_amount",
    ///     "bitcoin__title_payjoin",
    ///     "bitcoin__title_proof_of_ownership",
    ///     "bitcoin__title_purchase_ticket",
    ///     "bitcoin__title_update_transaction",
    ///     "bitcoin__unknown_path",
    ///     "bitcoin__unknown_transaction",
    ///     "bitcoin__unusually_high_fee",
    ///     "bitcoin__unverified_external_inputs",
    ///     "bitcoin__valid_signature",
    ///     "bitcoin__voting_rights",
    ///     "buttons__abort",
    ///     "buttons__access",
    ///     "buttons__again",
    ///     "buttons__allow",
    ///     "buttons__back_up",
    ///     "buttons__cancel",
    ///     "buttons__change",
    ///     "buttons__check",
    ///     "buttons__check_again",
    ///     "buttons__close",
    ///     "buttons__confirm",
    ///     "buttons__continue",
    ///     "buttons__details",
    ///     "buttons__enable",
    ///     "buttons__enter",
    ///     "buttons__enter_share",
    ///     "buttons__export",
    ///     "buttons__format",
    ///     "buttons__go_back",
    ///     "buttons__hold_to_confirm",
    ///     "buttons__info",
    ///     "buttons__more_info",
    ///     "buttons__ok_i_understand",
    ///     "buttons__purchase",
    ///     "buttons__quit",
    ///     "buttons__restart",
    ///     "buttons__retry",
    ///     "buttons__select",
    ///     "buttons__set",
    ///     "buttons__show_all",
    ///     "buttons__show_words",
    ///     "buttons__skip",
    ///     "buttons__try_again",
    ///     "buttons__turn_off",
    ///     "buttons__turn_on",
    ///     "cardano__addr_base",
    ///     "cardano__addr_enterprise",
    ///     "cardano__addr_legacy",
    ///     "cardano__addr_pointer",
    ///     "cardano__addr_reward",
    ///     "cardano__address_no_staking",
    ///     "cardano__amount",
    ///     "cardano__amount_burned",
    ///     "cardano__amount_minted",
    ///     "cardano__amount_sent",
    ///     "cardano__anonymous_pool",
    ///     "cardano__asset_fingerprint",
    ///     "cardano__auxiliary_data_hash",
    ///     "cardano__block",
    ///     "cardano__catalyst",
    ///     "cardano__certificate",
    ///     "cardano__certificate_path",
    ///     "cardano__change_output",
    ///     "cardano__change_output_path",
    ///     "cardano__change_output_staking_path",
    ///     "cardano__check_all_items",
    ///     "cardano__choose_level_of_details",
    ///     "cardano__collateral_input_id",
    ///     "cardano__collateral_input_index",
    ///     "cardano__collateral_output_contains_tokens",
    ///     "cardano__collateral_return",
    ///     "cardano__confirm",
    ///     "cardano__confirm_signing_stake_pool",
    ///     "cardano__confirm_transaction",
    ///     "cardano__confirming_a_multisig_transaction",
    ///     "cardano__confirming_pool_registration",
    ///     "cardano__confirming_transction",
    ///     "cardano__cost",
    ///     "cardano__credential_mismatch",
    ///     "cardano__datum_hash",
    ///     "cardano__delegating_to",
    ///     "cardano__for_account",
    ///     "cardano__for_key_hash",
    ///     "cardano__for_script",
    ///     "cardano__inline_datum",
    ///     "cardano__input_id",
    ///     "cardano__input_index",
    ///     "cardano__intro_text_address",
    ///     "cardano__intro_text_change",
    ///     "cardano__intro_text_owned_by_device",
    ///     "cardano__intro_text_registration_payment",
    ///     "cardano__key_hash",
    ///     "cardano__margin",
    ///     "cardano__multisig_path",
    ///     "cardano__nested_scripts_template",
    ///     "cardano__network",
    ///     "cardano__no_output_tx",
    ///     "cardano__nonce",
    ///     "cardano__other",
    ///     "cardano__path",
    ///     "cardano__pledge",
    ///     "cardano__pointer",
    ///     "cardano__policy_id",
    ///     "cardano__pool_metadata_hash",
    ///     "cardano__pool_metadata_url",
    ///     "cardano__pool_owner",
    ///     "cardano__pool_owner_path",
    ///     "cardano__pool_reward_account",
    ///     "cardano__reference_input_id",
    ///     "cardano__reference_input_index",
    ///     "cardano__reference_script",
    ///     "cardano__required_signer",
    ///     "cardano__reward",
    ///     "cardano__reward_address",
    ///     "cardano__reward_eligibility_warning",
    ///     "cardano__rewards_go_to",
    ///     "cardano__script",
    ///     "cardano__script_all",
    ///     "cardano__script_any",
    ///     "cardano__script_data_hash",
    ///     "cardano__script_hash",
    ///     "cardano__script_invalid_before",
    ///     "cardano__script_invalid_hereafter",
    ///     "cardano__script_key",
    ///     "cardano__script_n_of_k",
    ///     "cardano__script_reward",
    ///     "cardano__sending",
    ///     "cardano__show_simple",
    ///     "cardano__sign_tx_path_template",
    ///     "cardano__stake_delegation",
    ///     "cardano__stake_deregistration",
    ///     "cardano__stake_pool_registration",
    ///     "cardano__stake_pool_registration_pool_id",
    ///     "cardano__stake_registration",
    ///     "cardano__staking_key_for_account",
    ///     "cardano__to_pool",
    ///     "cardano__token_minting_path",
    ///     "cardano__total_collateral",
    ///     "cardano__transaction",
    ///     "cardano__transaction_contains_minting_or_burning",
    ///     "cardano__transaction_contains_script_address_no_datum",
    ///     "cardano__transaction_fee",
    ///     "cardano__transaction_id",
    ///     "cardano__transaction_no_collateral_input",
    ///     "cardano__transaction_no_script_data_hash",
    ///     "cardano__transaction_output_contains_tokens",
    ///     "cardano__ttl",
    ///     "cardano__unknown",
    ///     "cardano__unknown_collateral_amount",
    ///     "cardano__unusual_path",
    ///     "cardano__valid_since",
    ///     "cardano__verify_script",
    ///     "cardano__vote_key_registration",
    ///     "cardano__vote_public_key",
    ///     "cardano__voting_purpose",
    ///     "cardano__warning",
    ///     "cardano__weight",
    ///     "cardano__withdrawal_for_address_template",
    ///     "cardano__witness_path",
    ///     "cardano__x_of_y_signatures_template",
    ///     "coinjoin__access_account",
    ///     "coinjoin__do_not_disconnect",
    ///     "coinjoin__max_mining_fee",
    ///     "coinjoin__max_rounds",
    ///     "coinjoin__title",
    ///     "coinjoin__title_do_not_disconnect",
    ///     "coinjoin__title_progress",
    ///     "coinjoin__waiting_for_others",
    ///     "confirm_total__account",
    ///     "confirm_total__fee_rate",
    ///     "confirm_total__sending_from_account",
    ///     "confirm_total__title_fee",
    ///     "confirm_total__title_information",
    ///     "confirm_total__title_sending_from",
    ///     "debug__loading_seed",
    ///     "debug__loading_seed_not_recommended",
    ///     "device_name__change_template",
    ///     "device_name__title",
    ///     "entropy__send",
    ///     "entropy__title",
    ///     "entropy__title_confirm",
    ///     "eos__about_to_sign_template",
    ///     "eos__account",
    ///     "eos__action_name",
    ///     "eos__amount",
    ///     "eos__arbitrary_data",
    ///     "eos__buy_ram",
    ///     "eos__bytes",
    ///     "eos__cancel_vote",
    ///     "eos__checksum",
    ///     "eos__code",
    ///     "eos__contract",
    ///     "eos__cpu",
    ///     "eos__creator",
    ///     "eos__delegate",
    ///     "eos__delete_auth",
    ///     "eos__from",
    ///     "eos__link_auth",
    ///     "eos__memo",
    ///     "eos__name",
    ///     "eos__net",
    ///     "eos__new_account",
    ///     "eos__no",
    ///     "eos__owner",
    ///     "eos__parent",
    ///     "eos__payer",
    ///     "eos__permission",
    ///     "eos__proxy",
    ///     "eos__receiver",
    ///     "eos__refund",
    ///     "eos__requirement",
    ///     "eos__sell_ram",
    ///     "eos__sender",
    ///     "eos__sign_transaction",
    ///     "eos__threshold",
    ///     "eos__to",
    ///     "eos__transfer",
    ///     "eos__type",
    ///     "eos__undelegate",
    ///     "eos__unlink_auth",
    ///     "eos__update_auth",
    ///     "eos__vote_for_producers",
    ///     "eos__vote_for_proxy",
    ///     "eos__voter",
    ///     "eos__yes",
    ///     "errors__cancelled",
    ///     "errors__invalid_session",
    ///     "errors__pin_entry_cancelled",
    ///     "errors__pin_invalid",
    ///     "ethereum__amount_sent",
    ///     "ethereum__confirm_fee",
    ///     "ethereum__contract",
    ///     "ethereum__data_size_template",
    ///     "ethereum__gas_limit",
    ///     "ethereum__gas_price",
    ///     "ethereum__max_gas_price",
    ///     "ethereum__name_and_version",
    ///     "ethereum__new_contract",
    ///     "ethereum__no_message_field",
    ///     "ethereum__priority_fee",
    ///     "ethereum__show_full_array",
    ///     "ethereum__show_full_domain",
    ///     "ethereum__show_full_message",
    ///     "ethereum__show_full_struct",
    ///     "ethereum__sign_eip712",
    ///     "ethereum__title_confirm_data",
    ///     "ethereum__title_confirm_domain",
    ///     "ethereum__title_confirm_message",
    ///     "ethereum__title_confirm_struct",
    ///     "ethereum__title_confirm_typed_data",
    ///     "ethereum__title_signing_address",
    ///     "ethereum__units_template",
    ///     "ethereum__unknown_token",
    ///     "ethereum__valid_signature",
    ///     "experimental_mode__enable",
    ///     "experimental_mode__only_for_dev",
    ///     "experimental_mode__title",
    ///     "fido__already_registered",
    ///     "fido__device_already_registered",
    ///     "fido__device_already_registered_with_template",
    ///     "fido__device_not_registered",
    ///     "fido__does_not_belong",
    ///     "fido__erase_credentials",
    ///     "fido__export_credentials",
    ///     "fido__not_registered",
    ///     "fido__not_registered_with_template",
    ///     "fido__please_enable_pin_protection",
    ///     "fido__title_authenticate",
    ///     "fido__title_import_credential",
    ///     "fido__title_list_credentials",
    ///     "fido__title_register",
    ///     "fido__title_remove_credential",
    ///     "fido__title_reset",
    ///     "fido__title_u2f_auth",
    ///     "fido__title_u2f_register",
    ///     "fido__title_verify_user",
    ///     "fido__unable_to_verify_user",
    ///     "fido__wanna_erase_credentials",
    ///     "homescreen__click_to_connect",
    ///     "homescreen__click_to_unlock",
    ///     "homescreen__title_backup_failed",
    ///     "homescreen__title_backup_needed",
    ///     "homescreen__title_coinjoin_authorized",
    ///     "homescreen__title_experimental_mode",
    ///     "homescreen__title_hold_to_lock",
    ///     "homescreen__title_no_usb_connection",
    ///     "homescreen__title_pin_not_set",
    ///     "homescreen__title_seedless",
    ///     "homescreen__title_set",
    ///     "inputs__back",
    ///     "inputs__cancel",
    ///     "inputs__delete",
    ///     "inputs__enter",
    ///     "inputs__return",
    ///     "inputs__show",
    ///     "inputs__space",
    ///     "joint__title",
    ///     "joint__to_the_total_amount",
    ///     "joint__you_are_contributing",
    ///     "language__change_template",
    ///     "language__set_default",
    ///     "language__title_change",
    ///     "lockscreen__tap_to_connect",
    ///     "lockscreen__tap_to_unlock",
    ///     "lockscreen__title_locked",
    ///     "lockscreen__title_not_connected",
    ///     "misc__decrypt_value",
    ///     "misc__encrypt_value",
    ///     "misc__title_suite_labeling",
    ///     "modify_amount__address",
    ///     "modify_amount__decrease_amount",
    ///     "modify_amount__increase_amount",
    ///     "modify_amount__new_amount",
    ///     "modify_amount__title",
    ///     "modify_fee__decrease_fee",
    ///     "modify_fee__fee_rate",
    ///     "modify_fee__increase_fee",
    ///     "modify_fee__new_transaction_fee",
    ///     "modify_fee__no_change",
    ///     "modify_fee__title",
    ///     "modify_fee__transaction_fee",
    ///     "monero__confirm_export",
    ///     "monero__confirm_fee",
    ///     "monero__confirm_ki_sync",
    ///     "monero__confirm_refresh",
    ///     "monero__confirm_unlock_time",
    ///     "monero__hashing_inputs",
    ///     "monero__payment_id",
    ///     "monero__postprocessing",
    ///     "monero__processing",
    ///     "monero__processing_inputs",
    ///     "monero__processing_outputs",
    ///     "monero__signing",
    ///     "monero__signing_inputs",
    ///     "monero__unlock_time_set_template",
    ///     "monero__wanna_export_tx_der",
    ///     "monero__wanna_export_tx_key",
    ///     "monero__wanna_export_watchkey",
    ///     "monero__wanna_start_refresh",
    ///     "monero__wanna_sync_key_images",
    ///     "nem__absolute",
    ///     "nem__activate",
    ///     "nem__add",
    ///     "nem__confirm_action",
    ///     "nem__confirm_address",
    ///     "nem__confirm_creation_fee",
    ///     "nem__confirm_fee",
    ///     "nem__confirm_mosaic",
    ///     "nem__confirm_multisig_fee",
    ///     "nem__confirm_namespace",
    ///     "nem__confirm_payload",
    ///     "nem__confirm_properties",
    ///     "nem__confirm_rental_fee",
    ///     "nem__confirm_transfer_of",
    ///     "nem__convert_account_to_multisig",
    ///     "nem__cosign_transaction_for",
    ///     "nem__cosignatory",
    ///     "nem__create_mosaic",
    ///     "nem__create_namespace",
    ///     "nem__deactivate",
    ///     "nem__decrease",
    ///     "nem__description",
    ///     "nem__divisibility_and_levy_cannot_be_shown",
    ///     "nem__encrypted",
    ///     "nem__final_confirm",
    ///     "nem__immutable",
    ///     "nem__increase",
    ///     "nem__initial_supply",
    ///     "nem__initiate_transaction_for",
    ///     "nem__levy_divisibility",
    ///     "nem__levy_fee",
    ///     "nem__levy_fee_of",
    ///     "nem__levy_mosaic",
    ///     "nem__levy_namespace",
    ///     "nem__levy_recipient",
    ///     "nem__levy_type",
    ///     "nem__modify_supply_for",
    ///     "nem__modify_the_number_of_cosignatories_by",
    ///     "nem__mutable",
    ///     "nem__no",
    ///     "nem__of",
    ///     "nem__percentile",
    ///     "nem__raw_units_template",
    ///     "nem__remote_harvesting",
    ///     "nem__remove",
    ///     "nem__set_minimum_cosignatories_to",
    ///     "nem__sign_tx_fee_template",
    ///     "nem__supply_change",
    ///     "nem__supply_units_template",
    ///     "nem__transferable",
    ///     "nem__under_namespace",
    ///     "nem__unencrypted",
    ///     "nem__unknown_mosaic",
    ///     "nem__yes",
    ///     "passphrase__access_hidden_wallet",
    ///     "passphrase__always_on_device",
    ///     "passphrase__from_host_not_shown",
    ///     "passphrase__hidden_wallet",
    ///     "passphrase__hide",
    ///     "passphrase__next_screen_will_show_passphrase",
    ///     "passphrase__please_enter",
    ///     "passphrase__revoke_on_device",
    ///     "passphrase__title_confirm",
    ///     "passphrase__title_enter",
    ///     "passphrase__title_hide",
    ///     "passphrase__title_settings",
    ///     "passphrase__title_source",
    ///     "passphrase__turn_off",
    ///     "passphrase__turn_on",
    ///     "pin__change",
    ///     "pin__changed",
    ///     "pin__cursor_will_change",
    ///     "pin__diff_from_wipe_code",
    ///     "pin__disabled",
    ///     "pin__enabled",
    ///     "pin__enter",
    ///     "pin__enter_new",
    ///     "pin__entered_not_valid",
    ///     "pin__info",
    ///     "pin__invalid_pin",
    ///     "pin__last_attempt",
    ///     "pin__mismatch",
    ///     "pin__please_check_again",
    ///     "pin__reenter_new",
    ///     "pin__reenter_to_confirm",
    ///     "pin__should_be_long",
    ///     "pin__title_check_pin",
    ///     "pin__title_mismatch",
    ///     "pin__title_settings",
    ///     "pin__title_wrong_pin",
    ///     "pin__tries_left",
    ///     "pin__turn_off",
    ///     "pin__turn_on",
    ///     "pin__wrong_pin",
    ///     "progress__authenticity_check",
    ///     "progress__done",
    ///     "progress__loading_transaction",
    ///     "progress__one_second_left",
    ///     "progress__please_wait",
    ///     "progress__processing",
    ///     "progress__refreshing",
    ///     "progress__signing_transaction",
    ///     "progress__syncing",
    ///     "progress__x_seconds_left_template",
    ///     "reboot_to_bootloader__restart",
    ///     "reboot_to_bootloader__title",
    ///     "recovery__cancel_dry_run",
    ///     "recovery__check_dry_run",
    ///     "recovery__cursor_will_change",
    ///     "recovery__dry_run_bip39_valid_match",
    ///     "recovery__dry_run_bip39_valid_mismatch",
    ///     "recovery__dry_run_slip39_valid_match",
    ///     "recovery__dry_run_slip39_valid_mismatch",
    ///     "recovery__enter_any_share",
    ///     "recovery__enter_backup",
    ///     "recovery__enter_different_share",
    ///     "recovery__enter_share_from_diff_group",
    ///     "recovery__group_num_template",
    ///     "recovery__group_threshold_reached",
    ///     "recovery__invalid_seed_entered",
    ///     "recovery__invalid_share_entered",
    ///     "recovery__more_shares_needed",
    ///     "recovery__num_of_words",
    ///     "recovery__only_2_to_three_words",
    ///     "recovery__only_first_letters",
    ///     "recovery__progress_will_be_lost",
    ///     "recovery__select_num_of_words",
    ///     "recovery__share_already_entered",
    ///     "recovery__share_from_another_shamir",
    ///     "recovery__share_num_template",
    ///     "recovery__title",
    ///     "recovery__title_cancel_dry_run",
    ///     "recovery__title_cancel_recovery",
    ///     "recovery__title_dry_run",
    ///     "recovery__title_recover",
    ///     "recovery__title_remaining_shares",
    ///     "recovery__type_word_x_of_y_template",
    ///     "recovery__wallet_recovered",
    ///     "recovery__wanna_cancel_dry_run",
    ///     "recovery__wanna_cancel_recovery",
    ///     "recovery__word_count_template",
    ///     "recovery__word_x_of_y_template",
    ///     "recovery__x_of_y_entered_template",
    ///     "recovery__you_have_entered",
    ///     "reset__advanced_group_threshold_info",
    ///     "reset__all_x_of_y_template",
    ///     "reset__any_x_of_y_template",
    ///     "reset__button_create",
    ///     "reset__button_recover",
    ///     "reset__by_continuing",
    ///     "reset__check_backup_title",
    ///     "reset__check_group_share_title_template",
    ///     "reset__check_seed_title",
    ///     "reset__check_share_title_template",
    ///     "reset__continue_with_next_share",
    ///     "reset__continue_with_share_template",
    ///     "reset__finished_verifying_group_template",
    ///     "reset__finished_verifying_seed",
    ///     "reset__finished_verifying_shares",
    ///     "reset__group_description",
    ///     "reset__group_info",
    ///     "reset__group_share_checked_successfully_template",
    ///     "reset__group_share_title_template",
    ///     "reset__more_info_at",
    ///     "reset__need_all_share_template",
    ///     "reset__need_any_share_template",
    ///     "reset__needed_to_form_a_group",
    ///     "reset__needed_to_recover_your_wallet",
    ///     "reset__never_make_digital_copy",
    ///     "reset__num_of_share_holders_template",
    ///     "reset__num_of_shares_advanced_info_template",
    ///     "reset__num_of_shares_basic_info",
    ///     "reset__num_shares_for_group_template",
    ///     "reset__number_of_shares_info",
    ///     "reset__one_share",
    ///     "reset__only_one_share_will_be_created",
    ///     "reset__recovery_seed_title",
    ///     "reset__recovery_share_title_template",
    ///     "reset__required_number_of_groups",
    ///     "reset__select_correct_word",
    ///     "reset__select_word_template",
    ///     "reset__select_word_x_of_y_template",
    ///     "reset__set_it_to_count_template",
    ///     "reset__share_checked_successfully_template",
    ///     "reset__share_words_title",
    ///     "reset__slip39_checklist_num_groups",
    ///     "reset__slip39_checklist_num_shares",
    ///     "reset__slip39_checklist_set_sizes",
    ///     "reset__slip39_checklist_set_threshold",
    ///     "reset__slip39_checklist_title",
    ///     "reset__slip39_checklist_write_down",
    ///     "reset__the_threshold_sets_the_number_of_shares",
    ///     "reset__threshold_info",
    ///     "reset__title_backup_is_done",
    ///     "reset__title_create_wallet",
    ///     "reset__title_create_wallet_shamir",
    ///     "reset__title_group_threshold",
    ///     "reset__title_number_of_groups",
    ///     "reset__title_number_of_shares",
    ///     "reset__title_set_group_threshold",
    ///     "reset__title_set_number_of_groups",
    ///     "reset__title_set_number_of_shares",
    ///     "reset__title_set_threshold",
    ///     "reset__to_form_group_template",
    ///     "reset__tos_link",
    ///     "reset__total_number_of_shares_in_group_template",
    ///     "reset__use_your_backup",
    ///     "reset__write_down_words_template",
    ///     "reset__wrong_word_selected",
    ///     "reset__you_need_one_share",
    ///     "reset__your_backup_is_done",
    ///     "ripple__confirm_tag",
    ///     "ripple__destination_tag_template",
    ///     "rotation__change_template",
    ///     "rotation__east",
    ///     "rotation__north",
    ///     "rotation__south",
    ///     "rotation__title_change",
    ///     "rotation__west",
    ///     "safety_checks__approve_unsafe_always",
    ///     "safety_checks__approve_unsafe_temporary",
    ///     "safety_checks__enforce_strict",
    ///     "safety_checks__title",
    ///     "safety_checks__title_safety_override",
    ///     "sd_card__all_data_will_be_lost",
    ///     "sd_card__card_required",
    ///     "sd_card__disable",
    ///     "sd_card__disabled",
    ///     "sd_card__enable",
    ///     "sd_card__enabled",
    ///     "sd_card__error",
    ///     "sd_card__format_card",
    ///     "sd_card__insert_correct_card",
    ///     "sd_card__please_insert",
    ///     "sd_card__please_unplug_and_insert",
    ///     "sd_card__problem_accessing",
    ///     "sd_card__refresh",
    ///     "sd_card__refreshed",
    ///     "sd_card__restart",
    ///     "sd_card__title",
    ///     "sd_card__title_problem",
    ///     "sd_card__unknown_filesystem",
    ///     "sd_card__unplug_and_insert_correct",
    ///     "sd_card__use_different_card",
    ///     "sd_card__wanna_format",
    ///     "sd_card__wrong_sd_card",
    ///     "send__address_path",
    ///     "send__amount",
    ///     "send__confirm_sending",
    ///     "send__from_multiple_accounts",
    ///     "send__including_fee",
    ///     "send__maximum_fee",
    ///     "send__receiving_to_multisig",
    ///     "send__title_amount",
    ///     "send__title_confirm_sending",
    ///     "send__title_joint_transaction",
    ///     "send__title_receiving_to",
    ///     "send__title_recipient",
    ///     "send__title_sending",
    ///     "send__title_sending_amount",
    ///     "send__title_sending_to",
    ///     "send__to_the_total_amount",
    ///     "send__total_amount",
    ///     "send__transaction_id",
    ///     "send__you_are_contributing",
    ///     "share_words__words_in_order",
    ///     "share_words__wrote_down_all",
    ///     "sign_message__confirm_address",
    ///     "sign_message__confirm_message",
    ///     "stellar__account",
    ///     "stellar__account_merge",
    ///     "stellar__account_thresholds",
    ///     "stellar__add_signer",
    ///     "stellar__add_trust",
    ///     "stellar__all_will_be_sent_to",
    ///     "stellar__allow_trust",
    ///     "stellar__asset",
    ///     "stellar__bump_sequence",
    ///     "stellar__buying",
    ///     "stellar__clear_data",
    ///     "stellar__clear_flags",
    ///     "stellar__confirm_issuer",
    ///     "stellar__confirm_memo",
    ///     "stellar__confirm_network",
    ///     "stellar__confirm_operation",
    ///     "stellar__confirm_stellar",
    ///     "stellar__confirm_timebounds",
    ///     "stellar__create_account",
    ///     "stellar__debited_amount",
    ///     "stellar__delete",
    ///     "stellar__delete_passive_offer",
    ///     "stellar__delete_trust",
    ///     "stellar__destination",
    ///     "stellar__exchanges_require_memo",
    ///     "stellar__final_confirm",
    ///     "stellar__hash",
    ///     "stellar__high",
    ///     "stellar__home_domain",
    ///     "stellar__inflation",
    ///     "stellar__initial_balance",
    ///     "stellar__initialize_signing_with",
    ///     "stellar__issuer_template",
    ///     "stellar__key",
    ///     "stellar__limit",
    ///     "stellar__low",
    ///     "stellar__master_weight",
    ///     "stellar__medium",
    ///     "stellar__new_offer",
    ///     "stellar__new_passive_offer",
    ///     "stellar__no_memo_set",
    ///     "stellar__no_restriction",
    ///     "stellar__on_network_template",
    ///     "stellar__path_pay",
    ///     "stellar__path_pay_at_least",
    ///     "stellar__pay",
    ///     "stellar__pay_at_most",
    ///     "stellar__preauth_transaction",
    ///     "stellar__price_per_template",
    ///     "stellar__private_network",
    ///     "stellar__remove_signer",
    ///     "stellar__revoke_trust",
    ///     "stellar__selling",
    ///     "stellar__set_data",
    ///     "stellar__set_flags",
    ///     "stellar__set_sequence_to_template",
    ///     "stellar__sign_tx_count_template",
    ///     "stellar__sign_tx_fee_template",
    ///     "stellar__source_account",
    ///     "stellar__testnet_network",
    ///     "stellar__trusted_account",
    ///     "stellar__update",
    ///     "stellar__valid_from",
    ///     "stellar__valid_to",
    ///     "stellar__value_sha256",
    ///     "stellar__wanna_clean_value_key_template",
    ///     "stellar__your_account",
    ///     "tezos__address",
    ///     "tezos__amount",
    ///     "tezos__baker_address",
    ///     "tezos__balance",
    ///     "tezos__ballot",
    ///     "tezos__confirm_delegation",
    ///     "tezos__confirm_origination",
    ///     "tezos__delegator",
    ///     "tezos__fee",
    ///     "tezos__proposal",
    ///     "tezos__register_delegate",
    ///     "tezos__remove_delegation",
    ///     "tezos__submit_ballot",
    ///     "tezos__submit_proposal",
    ///     "tezos__submit_proposals",
    ///     "tutorial__middle_click",
    ///     "tutorial__press_and_hold",
    ///     "tutorial__ready_to_use",
    ///     "tutorial__scroll_down",
    ///     "tutorial__sure_you_want_skip",
    ///     "tutorial__title_hello",
    ///     "tutorial__title_screen_scroll",
    ///     "tutorial__title_skip",
    ///     "tutorial__title_tutorial_complete",
    ///     "tutorial__use_trezor",
    ///     "tutorial__welcome_press_right",
    ///     "u2f__get",
    ///     "u2f__set_template",
    ///     "u2f__title_get",
    ///     "u2f__title_set",
    ///     "wipe__info",
    ///     "wipe__title",
    ///     "wipe__want_to_wipe",
    ///     "wipe_code__change",
    ///     "wipe_code__changed",
    ///     "wipe_code__diff_from_pin",
    ///     "wipe_code__disabled",
    ///     "wipe_code__enabled",
    ///     "wipe_code__enter_new",
    ///     "wipe_code__info",
    ///     "wipe_code__invalid",
    ///     "wipe_code__mismatch",
    ///     "wipe_code__reenter",
    ///     "wipe_code__reenter_to_confirm",
    ///     "wipe_code__title_check",
    ///     "wipe_code__title_invalid",
    ///     "wipe_code__title_mismatch",
    ///     "wipe_code__title_settings",
    ///     "wipe_code__turn_off",
    ///     "wipe_code__turn_on",
    ///     "word_count__title",
    ///     "words__are_you_sure",
    ///     "words__buying",
    ///     "words__continue_anyway",
    ///     "words__continue_with",
    ///     "words__error",
    ///     "words__from",
    ///     "words__keep_it_safe",
    ///     "words__know_what_your_doing",
    ///     "words__my_trezor",
    ///     "words__outputs",
    ///     "words__please_check_again",
    ///     "words__please_try_again",
    ///     "words__really_wanna",
    ///     "words__sign",
    ///     "words__title_check",
    ///     "words__title_group",
    ///     "words__title_remember",
    ///     "words__title_share",
    ///     "words__title_shares",
    ///     "words__title_success",
    ///     "words__title_summary",
    ///     "words__title_threshold",
    ///     "words__unknown",
    ///     "words__warning",
    /// ]) -> str:
    ///     """Get translation in the current language."""
    Qstr::MP_QSTR_tr => obj_fn_1!(translate_obj).as_obj(),
};
