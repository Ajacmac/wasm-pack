//! Functionality related to publishing to npm.

use child;
use command::publish::access::Access;
use failure::{self, ResultExt};
use log::info;

/// The default npm registry used when we aren't working with a custom registry.
pub const DEFAULT_NPM_REGISTRY: &str = "https://registry.npmjs.org/";

/// Run the `npm pack` command.
pub fn npm_pack(path: &str) -> Result<(), failure::Error> {
    let mut cmd = child::new_command("npm");
    cmd.current_dir(path).arg("pack");
    child::run(cmd, "npm pack").context("Packaging up your code failed")?;
    Ok(())
}

/// Run the `npm publish` command.
pub fn npm_publish(path: &str, access: Option<Access>) -> Result<(), failure::Error> {
    let mut cmd = child::new_command("npm");
    match access {
        Some(a) => cmd
            .current_dir(path)
            .arg("publish")
            .arg(&format!("{}", a.to_string())),
        None => cmd.current_dir(path).arg("publish"),
    };

    child::run(cmd, "npm publish").context("Publishing to npm failed")?;
    Ok(())
}

/// Run the `npm login` command.
pub fn npm_login(
    registry: &str,
    scope: &Option<String>,
    always_auth: bool,
    auth_type: &Option<String>,
) -> Result<(), failure::Error> {
    let mut args = vec![format!("login"), format!("--registry={}", registry)];

    if let Some(scope) = scope {
        args.push(format!("--scope={}", scope));
    }

    if always_auth {
        args.push(format!("--always_auth"));
    }

    if let Some(auth_type) = auth_type {
        args.push(format!("--auth_type={}", auth_type));
    }

    // Interactively ask user for npm login info.
    //  (child::run does not support interactive input)
    let mut cmd = child::new_command("npm");
    cmd.args(args);

    info!("Running {:?}", cmd);
    match cmd.status()?.success() {
        true => Ok(()),
        false => bail!("Login to registry {} failed", registry),
    }
}
