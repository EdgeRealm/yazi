use std::{path::Path, process::Stdio};

use anyhow::{bail, Result};
use tokio::process::Command;
use yazi_shared::Url;

pub struct FzfOpt {
	pub cwd: Url,
}

pub async fn fzf(opt: FzfOpt) -> Result<Url> {
	let child =
		Command::new("fzf").current_dir(&opt.cwd).kill_on_drop(true).stdout(Stdio::piped()).spawn()?;

	let output = child.wait_with_output().await?;
	let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

	if selected.is_empty() {
		bail!("No match")
	}
	return Ok(Url::from(Path::new(&opt.cwd).join(selected)));
}

pub async fn fzf_marks() -> Result<Url> {
    let mut child_marks =
		Command::new("cat").arg("/Users/edge/.fzf-marks").stdout(Stdio::piped()).spawn().expect("");
    let child_fzf_stdin: Stdio = child_marks
        .stdout
        .take()
        .unwrap()
        .try_into()
        .expect("");
    let child_fzf = Command::new("fzf").arg("--query").arg("").stdin(child_fzf_stdin).stdout(Stdio::piped()).spawn().unwrap();

    let output = child_fzf.wait_with_output().await?;
    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected.is_empty() {
        bail!("No match")
    }
    let mut selected_path: Vec<String> = selected.split(" : ").map(|s| s.to_string()).collect();
    let selected_url = Url::from(selected_path.remove(1));
    return Ok(selected_url);
}
