// Copyright 2015, 2016 Ethcore (UK) Ltd.
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

//! Block RLP compression.

use block::Block;
use header::Header;

use views::BlockView;
use rlp::{DecoderError, RlpStream, Stream, UntrustedRlp, View};
use rlp::{Compressible, RlpType};
use util::{Bytes, Hashable, H256};

const HEADER_FIELDS: usize = 10;
const BLOCK_FIELDS: usize = 2;

pub struct AbridgedBlock {
	rlp: Bytes,
}

impl AbridgedBlock {
	/// Create from rlp-compressed bytes. Does no verification.
	pub fn from_raw(compressed: Bytes) -> Self {
		AbridgedBlock {
			rlp: compressed,
		}
	}

	/// Return the inner bytes.
	pub fn into_inner(self) -> Bytes {
		self.rlp
	}

	/// Given a full block view, trim out the parent hash and block number,
	/// producing new rlp.
	pub fn from_block_view(block_view: &BlockView) -> Self {
		let header = block_view.header_view();
		let seal_fields = header.seal();

		// 10 header fields, unknown number of seal fields, and 2 block fields.
		let mut stream = RlpStream::new_list(
			HEADER_FIELDS +
			seal_fields.len() +
			BLOCK_FIELDS
		);

		// write header values.
		stream
			.append(&header.author())
			.append(&header.state_root())
			.append(&header.transactions_root())
			.append(&header.receipts_root())
			.append(&header.log_bloom())
			.append(&header.difficulty())
			.append(&header.gas_limit())
			.append(&header.gas_used())
			.append(&header.timestamp())
			.append(&header.extra_data());

		// write block values.
		stream.append(&block_view.transactions()).append(&block_view.uncles());

		// write seal fields.
		for field in seal_fields {
			stream.append_raw(&field, 1);
		}

		AbridgedBlock {
			rlp: UntrustedRlp::new(stream.as_raw()).compress(RlpType::Blocks).to_vec(),
		}
	}

	/// Flesh out an abridged block view with the provided parent hash and block number.
	///
	/// Will fail if contains invalid rlp.
	pub fn to_block(&self, parent_hash: H256, number: u64) -> Result<Block, DecoderError> {
		let rlp = UntrustedRlp::new(&self.rlp).decompress(RlpType::Blocks);
		let rlp = UntrustedRlp::new(&rlp);

		let mut header: Header = Default::default();
		header.set_parent_hash(parent_hash);
		header.set_author(try!(rlp.val_at(0)));
		header.set_state_root(try!(rlp.val_at(1)));
		header.set_transactions_root(try!(rlp.val_at(2)));
		header.set_receipts_root(try!(rlp.val_at(3)));
		header.set_log_bloom(try!(rlp.val_at(4)));
		header.set_difficulty(try!(rlp.val_at(5)));
		header.set_number(number);
		header.set_gas_limit(try!(rlp.val_at(6)));
		header.set_gas_used(try!(rlp.val_at(7)));
		header.set_timestamp(try!(rlp.val_at(8)));
		header.set_extra_data(try!(rlp.val_at(9)));

		let transactions = try!(rlp.val_at(10));
		let uncles: Vec<Header> = try!(rlp.val_at(11));

		let mut uncles_rlp = RlpStream::new();
		uncles_rlp.append(&uncles);
		header.set_uncles_hash(uncles_rlp.as_raw().sha3());

		let mut seal_fields = Vec::new();
		for i in (HEADER_FIELDS + BLOCK_FIELDS)..rlp.item_count() {
			let seal_rlp = try!(rlp.at(i));
			seal_fields.push(seal_rlp.as_raw().to_owned());
		}

		header.set_seal(seal_fields);

		Ok(Block {
			header: header,
			transactions: transactions,
			uncles: uncles,
		})
	}
}

#[cfg(test)]
mod tests {
	use views::BlockView;
	use block::Block;
	use super::AbridgedBlock;
	use types::transaction::{Action, Transaction};

	use util::{Address, H256, FixedHash, U256, Bytes};

	fn encode_block(b: &Block) -> Bytes {
		b.rlp_bytes(::basic_types::Seal::With)
	}

	#[test]
	fn empty_block_abridging() {
		let b = Block::default();
		let encoded = encode_block(&b);

		let abridged = AbridgedBlock::from_block_view(&BlockView::new(&encoded));
		assert_eq!(abridged.to_block(H256::new(), 0).unwrap(), b);
	}

	#[test]
	#[should_panic]
	fn wrong_number() {
		let b = Block::default();
		let encoded = encode_block(&b);

		let abridged = AbridgedBlock::from_block_view(&BlockView::new(&encoded));
		assert_eq!(abridged.to_block(H256::new(), 2).unwrap(), b);
	}

	#[test]
	fn with_transactions() {
		let mut b = Block::default();

		let t1 = Transaction {
			action: Action::Create,
			nonce: U256::from(42),
			gas_price: U256::from(3000),
			gas: U256::from(50_000),
			value: U256::from(1),
			data: b"Hello!".to_vec()
		}.fake_sign(Address::from(0x69));

		let t2 = Transaction {
			action: Action::Create,
			nonce: U256::from(88),
			gas_price: U256::from(12345),
			gas: U256::from(300000),
			value: U256::from(1000000000),
			data: "Eep!".into(),
		}.fake_sign(Address::from(0x55));

		b.transactions.push(t1);
		b.transactions.push(t2);

		let encoded = encode_block(&b);

		let abridged = AbridgedBlock::from_block_view(&BlockView::new(&encoded[..]));
		assert_eq!(abridged.to_block(H256::new(), 0).unwrap(), b);
	}
}
