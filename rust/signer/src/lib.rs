// Copyright 2015-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use pixelate::{Color, Image, BLACK};
use qrcodegen::{QrCode, QrCodeEcc};

use plot_icon;
use db_handling;
use transaction_parsing;
use transaction_signing;
use qr_reader_phone;

mod export;
mod result;

fn base64png(png: &[u8]) -> String {
	static HEADER: &str = "data:image/png;base64,";
	let mut out = String::with_capacity(png.len() + png.len() / 2 + HEADER.len());
	out.push_str(HEADER);
	base64::encode_config_buf(png, base64::STANDARD, &mut out);
	out
}

fn qrcode_bytes(data: &[u8]) -> anyhow::Result<String, anyhow::Error> {
	let qr = QrCode::encode_binary(data, QrCodeEcc::Medium)?;
	let palette = &[Color::Rgba(255, 255, 255, 0), BLACK];
	let mut pixels = Vec::with_capacity((qr.size() * qr.size()) as usize);
	for y in 0..qr.size() {
		for x in 0..qr.size() {
			pixels.push(qr.get_module(x, y) as u8);
		}
	}
	let mut result = Vec::new();
	let image = Image {
		palette,
		pixels: &pixels,
		width: qr.size() as usize,
		scale: 16,
	};
	match image.render(&mut result) {
    	Ok(_) => Ok(base64png(&result)),
        Err(_) => return Err(anyhow::anyhow!("Pixelation failed")),
    }
}

export! {
	@Java_io_parity_signer_SubstrateSignModule_ethkeyQrCode
	fn qrcode(
		data: &str
	) -> anyhow::Result<String, anyhow::Error> {
		qrcode_bytes(data.as_bytes())
	}

	@Java_io_parity_signer_SubstrateSignModule_qrparserGetPacketsTotal
	fn get_packets_total(
		data: &str
	) -> anyhow::Result<u32, anyhow::Error> {
        qr_reader_phone::get_length(data)
	}

	@Java_io_parity_signer_SubstrateSignModule_qrparserTryDecodeQrSequence
	fn try_decode_qr_sequence(
		data: &str
	) -> anyhow::Result<String, anyhow::Error> {
        qr_reader_phone::decode_sequence(data)
	}

    @Java_io_parity_signer_SubstrateSignModule_substrateParseTransaction
	fn parse_transaction(
		transaction: &str,
        dbname: &str
	) -> String {
        if transaction == "test all" {return transaction_parsing::test_all_cards::make_all_cards()}
        else {return transaction_parsing::produce_output(transaction, dbname)}
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateHandleAction
	fn handle_action(
		action: &str,
        seed_phrase: &str,
        password: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        transaction_signing::handle_action(action, seed_phrase, password, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateDevelopmentTest
	fn development_test(
		_input: &str
	) -> anyhow::Result<String, anyhow::Error> {
        //let output = Ok(std::env::consts::OS.to_string());
        let picture = plot_icon::png_data_from_hex("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", 64)?;
        Ok(hex::encode(picture))
    }

    @Java_io_parity_signer_SubstrateSignModule_dbGetNetwork
	fn get_network(
		genesis_hash: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        let spec = db_handling::chainspecs::get_network(dbname, genesis_hash)?;
        Ok(String::from(format!("{{\"color\":\"{}\",\"logo\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}}",
            spec.color, 
            spec.logo,
            spec.secondary_color,
            spec.title)))
    }

    @Java_io_parity_signer_SubstrateSignModule_dbGetAllNetworksForNetworkSelector
	fn get_all_networks_for_network_selector(
        dbname: &str
    ) -> anyhow::Result<String, anyhow::Error> {
        let specs = db_handling::chainspecs::get_all_networks(dbname)?;
        //TODO: gentler formatting, or serde-json?
        let mut output = "[".to_owned();
        for spec in specs {
            output.push_str(&format!("{{\"key\":\"{}\",\"color\":\"{}\",\"logo\":\"{}\",\"order\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}},",
                hex::encode(spec.genesis_hash),
                spec.color, 
                spec.logo, 
                spec.order,
                spec.secondary_color,
                spec.title))
        }
        result::return_json_array(output)
    }

    @Java_io_parity_signer_SubstrateSignModule_dbGetRelevantIdentities
	fn get_relevant_identities(
		seed_name: &str,
        genesis_hash: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::print_relevant_identities(seed_name, genesis_hash, dbname)
    }
    
    @Java_io_parity_signer_SubstrateSignModule_dbGetAllIdentities
	fn get_all_identities(
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::print_all_identities(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateTryCreateSeed
	fn try_create_seed(
        seed_name: &str,
        crypto: &str,
        seed_phrase: &str,
        seed_length: u32,
		dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::try_create_seed(seed_name, crypto, seed_phrase, seed_length, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateSuggestNPlusOne
	fn suggest_n_plus_one(
        path: &str,
        seed_name: &str,
        network_id_string: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::suggest_n_plus_one(path, seed_name, network_id_string, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateCheckPath
	fn check_path(
        path: &str
	) -> anyhow::Result<bool, anyhow::Error> {
        db_handling::identities::check_derivation_format(path)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateTryCreateIdentity
	fn try_create_identity(
        id_name: &str,
        seed_name: &str,
        seed_phrase: &str,
        crypto: &str,
        path: &str,
        network: &str,
        has_password: bool,
		dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::try_create_address(id_name, seed_name, seed_phrase, crypto, path, network, has_password, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateSuggestName
	fn suggest_name(
        path: &str
	) -> String {
        db_handling::identities::suggest_path_name(path)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateDeleteIdentity
	fn delete_identity(
        pub_key: &str,
        network: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::delete_address(pub_key, network, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateGetNetworkSpecs
	fn get_network_specs(
        network: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::network_details::get_network_details_by_hex(network, dbname)
    }
    
    @Java_io_parity_signer_SubstrateSignModule_substrateRemoveNetwork
	fn remove_network(
        network: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::remove_network::remove_network_by_hex(network, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateRemoveMetadata
	fn remove_metadata(
        network_name: &str,
        network_version: u32,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::remove_network::remove_metadata(network_name, network_version, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateRemoveSeed
	fn remove_seed(
        seed_name: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::remove_identities_for_seed(seed_name, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historyPrintHistory
	fn print_history(
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::manage_history::print_history(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historyClearHistory
	fn clear_history(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::clear_history(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historyInitHistory
	fn init_history(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::init_history(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historyDeviceWasOnline
	fn device_was_online(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::device_was_online(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historySeedsWereAccessed
	fn seeds_were_accessed(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::seeds_were_accessed(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historySeedsWereShown
	fn seeds_were_shown(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::seeds_were_shown(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_historyHistoryEntryUser
	fn history_entry_user(
        entry: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::history_entry_user(dbname, entry.to_string())
    }

    @Java_io_parity_signer_SubstrateSignModule_historyHistoryEntrySystem
	fn history_entry_system(
        entry: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::history_entry_system(dbname, entry.to_string())
    }
}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
	use super::*;
}
