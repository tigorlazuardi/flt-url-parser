use std::time::Duration;

use anyhow::Result;

/// HTTP Request to given url
///
/// * `url`: url request
pub fn get(url: &str) -> Result<String> {
	Ok(ureq::get(url)
		.timeout(Duration::from_secs(10))
		.call()?
		.into_string()?)
}
