use std::{path::PathBuf, os::unix::ffi::OsStrExt};

use sha2::{Sha256, Digest};

// pub fn hash_sha_256(input: &[u8]) -> String {
// 	let mut hasher = Sha256::new();
// 	hasher.update(input);
// 	let digest = hasher.finalize();
// 	return format!("{:x}", digest);
// }

// pub fn hash_string_sha_256(input: &str) -> String {
// 	return hash_sha_256(input.as_bytes());
// }

pub fn hash_path_buff_sha_256(input: &PathBuf) -> String {
	let mut hasher = Sha256::new();
	let v = input.as_os_str();
	hasher.update(v.as_bytes());
	let digest = hasher.finalize();
	return format!("{:x}", digest);
}