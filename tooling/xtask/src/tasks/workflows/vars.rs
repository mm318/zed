use std::cell::RefCell;

use gh_workflow::{Env, Expression, WorkflowDispatchInput};

use crate::tasks::workflows::{runners::Platform, steps::NamedJob};

macro_rules! secret {
    ($secret_name:ident) => {
        pub const $secret_name: &str = concat!("${{ secrets.", stringify!($secret_name), " }}");
    };
}

macro_rules! var {
    ($var_name:ident) => {
        pub const $var_name: &str = concat!("${{ vars.", stringify!($var_name), " }}");
    };
}

secret!(APPLE_NOTARIZATION_ISSUER_ID);
secret!(APPLE_NOTARIZATION_KEY);
secret!(APPLE_NOTARIZATION_KEY_ID);
secret!(AZURE_SIGNING_CLIENT_ID);
secret!(AZURE_SIGNING_CLIENT_SECRET);
secret!(AZURE_SIGNING_TENANT_ID);
secret!(CACHIX_AUTH_TOKEN);
secret!(GITHUB_TOKEN);
secret!(MACOS_CERTIFICATE);
secret!(MACOS_CERTIFICATE_PASSWORD);
secret!(SENTRY_AUTH_TOKEN);
secret!(ZED_CLIENT_CHECKSUM_SEED);
secret!(ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON);
secret!(ZED_SENTRY_MINIDUMP_ENDPOINT);
secret!(R2_ACCOUNT_ID);
secret!(R2_ACCESS_KEY_ID);
secret!(R2_SECRET_ACCESS_KEY);

// todo(ci) make these secrets too...
var!(AZURE_SIGNING_ACCOUNT_NAME);
var!(AZURE_SIGNING_CERT_PROFILE_NAME);
var!(AZURE_SIGNING_ENDPOINT);

pub fn bundle_envs(platform: Platform) -> Env {
    let env = Env::default()
        .add("CARGO_INCREMENTAL", 0)
        .add("ZED_CLIENT_CHECKSUM_SEED", ZED_CLIENT_CHECKSUM_SEED)
        .add("ZED_MINIDUMP_ENDPOINT", ZED_SENTRY_MINIDUMP_ENDPOINT);

    match platform {
        Platform::Linux => env,
        Platform::Mac => env
            .add("MACOS_CERTIFICATE", MACOS_CERTIFICATE)
            .add("MACOS_CERTIFICATE_PASSWORD", MACOS_CERTIFICATE_PASSWORD)
            .add("APPLE_NOTARIZATION_KEY", APPLE_NOTARIZATION_KEY)
            .add("APPLE_NOTARIZATION_KEY_ID", APPLE_NOTARIZATION_KEY_ID)
            .add("APPLE_NOTARIZATION_ISSUER_ID", APPLE_NOTARIZATION_ISSUER_ID),
        Platform::Windows => env
            .add("AZURE_TENANT_ID", AZURE_SIGNING_TENANT_ID)
            .add("AZURE_CLIENT_ID", AZURE_SIGNING_CLIENT_ID)
            .add("AZURE_CLIENT_SECRET", AZURE_SIGNING_CLIENT_SECRET)
            .add("ACCOUNT_NAME", AZURE_SIGNING_ACCOUNT_NAME)
            .add("CERT_PROFILE_NAME", AZURE_SIGNING_CERT_PROFILE_NAME)
            .add("ENDPOINT", AZURE_SIGNING_ENDPOINT)
            .add("FILE_DIGEST", "SHA256")
            .add("TIMESTAMP_DIGEST", "SHA256")
            .add("TIMESTAMP_SERVER", "http://timestamp.acs.microsoft.com"),
    }
}

// Represents a pattern to check for changed files and corresponding output variable
pub struct PathCondition {
    pub name: &'static str,
    pub pattern: &'static str,
    pub invert: bool,
    pub set_by_step: RefCell<Option<String>>,
}
impl PathCondition {
    pub fn new(name: &'static str, pattern: &'static str) -> Self {
        Self {
            name,
            pattern,
            invert: false,
            set_by_step: Default::default(),
        }
    }
    pub fn inverted(name: &'static str, pattern: &'static str) -> Self {
        Self {
            name,
            pattern,
            invert: true,
            set_by_step: Default::default(),
        }
    }
    pub fn guard(&self, job: NamedJob) -> NamedJob {
        let set_by_step = self
            .set_by_step
            .borrow()
            .clone()
            .unwrap_or_else(|| panic!("condition {},is never set", self.name));
        NamedJob {
            name: job.name,
            job: job
                .job
                .add_need(set_by_step.clone())
                .cond(Expression::new(format!(
                    "needs.{}.outputs.{} == 'true'",
                    &set_by_step, self.name
                ))),
        }
    }
}

pub struct WorkflowInput {
    pub input_type: &'static str,
    pub name: &'static str,
    pub default: Option<String>,
    pub description: Option<String>,
}

impl WorkflowInput {
    pub fn string(name: &'static str, default: Option<String>) -> Self {
        Self {
            input_type: "string",
            name,
            default,
            description: None,
        }
    }

    pub fn input(&self) -> WorkflowDispatchInput {
        WorkflowDispatchInput {
            description: self
                .description
                .clone()
                .unwrap_or_else(|| self.name.to_owned()),
            required: self.default.is_none(),
            input_type: self.input_type.to_owned(),
            default: self.default.clone(),
        }
    }

    pub(crate) fn expr(&self) -> String {
        format!("inputs.{}", self.name)
    }
}

impl std::fmt::Display for WorkflowInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{{ {} }}}}", self.expr())
    }
}

impl serde::Serialize for WorkflowInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub mod assets {
    pub const MAC_AARCH64: &str = "Zed-aarch64.dmg";
    pub const MAC_X86_64: &str = "Zed-x86_64.dmg";
    pub const LINUX_AARCH64: &str = "zed-linux-aarch64.tar.gz";
    pub const LINUX_X86_64: &str = "zed-linux-x86_64.tar.gz";
    pub const WINDOWS_X86_64: &str = "Zed-x86_64.exe";
    pub const WINDOWS_AARCH64: &str = "Zed-aarch64.exe";

    pub const REMOTE_SERVER_MAC_AARCH64: &str = "zed-remote-server-macos-aarch64.gz";
    pub const REMOTE_SERVER_MAC_X86_64: &str = "zed-remote-server-macos-x86_64.gz";
    pub const REMOTE_SERVER_LINUX_AARCH64: &str = "zed-remote-server-linux-aarch64.gz";
    pub const REMOTE_SERVER_LINUX_X86_64: &str = "zed-remote-server-linux-x86_64.gz";
    pub const REMOTE_SERVER_WINDOWS_AARCH64: &str = "zed-remote-server-windows-aarch64.zip";
    pub const REMOTE_SERVER_WINDOWS_X86_64: &str = "zed-remote-server-windows-x86_64.zip";
}
