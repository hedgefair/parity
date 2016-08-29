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

use std::fs;
use std::path::{PathBuf, Path};
use util::{H64, H256};
use util::journaldb::Algorithm;
use helpers::replace_home;

// this const is irrelevent cause we do have migrations now,
// but we still use it for backwards compatibility
const LEGACY_CLIENT_DB_VER_STR: &'static str = "5.3";

#[derive(Debug, PartialEq)]
pub struct Directories {
	pub db: String,
	pub keys: String,
	pub signer: String,
	pub dapps: String,
}

impl Default for Directories {
	fn default() -> Self {
		Directories {
			db: replace_home("$HOME/.parity"),
			keys: replace_home("$HOME/.parity/keys"),
			signer: replace_home("$HOME/.parity/signer"),
			dapps: replace_home("$HOME/.parity/dapps"),
		}
	}
}

impl Directories {
	pub fn create_dirs(&self) -> Result<(), String> {
		try!(fs::create_dir_all(&self.db).map_err(|e| e.to_string()));
		try!(fs::create_dir_all(&self.keys).map_err(|e| e.to_string()));
		try!(fs::create_dir_all(&self.signer).map_err(|e| e.to_string()));
		try!(fs::create_dir_all(&self.dapps).map_err(|e| e.to_string()));
		Ok(())
	}

	/// Get the root path for database
	pub fn db_version_path(&self, genesis_hash: H256, fork_name: Option<&String>, pruning: Algorithm) -> PathBuf {
		let mut dir = Path::new(&self.db).to_path_buf();
		dir.push(format!("{:?}{}", H64::from(genesis_hash), fork_name.map(|f| format!("-{}", f)).unwrap_or_default()));
		dir.push(format!("v{}-sec-{}", LEGACY_CLIENT_DB_VER_STR, pruning.as_internal_name_str()));
		dir
	}

	/// Get the path for the databases given the genesis_hash and information on the databases.
	pub fn client_path(&self, genesis_hash: H256, fork_name: Option<&String>, pruning: Algorithm) -> PathBuf {
		let mut dir = self.db_version_path(genesis_hash, fork_name, pruning);
		dir.push("db");
		dir
	}

	/// Get the ipc sockets path
	pub fn ipc_path(&self) -> PathBuf {
		let mut dir = Path::new(&self.db).to_path_buf();
		dir.push("ipc");
		dir
	}

	/// Get user defaults path
	pub fn user_defaults_path(&self, fork_name: &Option<String>) -> PathBuf {
		let mut dir = Path::new(&self.db).to_path_buf();
		match *fork_name {
			Some(ref name) => dir.push(format!("user_defaults_{}", name)),
			None => dir.push("user_defaults"),
		}
		dir
	}
}

#[cfg(test)]
mod tests {
	use super::Directories;
	use helpers::replace_home;

	#[test]
	fn test_default_directories() {
		let expected = Directories {
			db: replace_home("$HOME/.parity"),
			keys: replace_home("$HOME/.parity/keys"),
			signer: replace_home("$HOME/.parity/signer"),
			dapps: replace_home("$HOME/.parity/dapps"),
		};
		assert_eq!(expected, Directories::default());
	}
}
