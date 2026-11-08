//! Turning one kubeconfig context into a Cluster's connection and secrets.

use base64::Engine as _;
use secrecy::ExposeSecret;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::{read_referenced, Source};
use crate::{
    messages as msg,
    model::{AuthMethod, ClusterConnection, ClusterSecrets, ExecConfig, KubeconfigSource},
};

/// The connection and credentials of the context `name`: what reaches the
/// API server in the first, every credential in the second, the files the
/// kubeconfig refers to read and inlined.
pub fn resolve(source: &Source, name: &str) -> AppResult<(ClusterConnection, ClusterSecrets)> {
    let config = &source.config;
    let context = config
        .contexts
        .iter()
        .find(|c| c.name == name)
        .and_then(|c| c.context.as_ref())
        .ok_or_else(|| {
            AppError::new(ErrorCode::NotFound, msg::CONTEXT_GONE.with("context", name))
                .with_hint(msg::CONTEXT_GONE_HINT)
        })?;
    let cluster = config
        .clusters
        .iter()
        .find(|c| c.name == context.cluster)
        .and_then(|c| c.cluster.as_ref())
        .ok_or_else(|| {
            AppError::validation(msg::CONTEXT_NO_CLUSTER.with("cluster", &context.cluster))
        })?;
    let server = cluster
        .server
        .clone()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::validation(msg::CONTEXT_NO_SERVER))?;

    let mut secrets = ClusterSecrets {
        certificate_authority: match (
            &cluster.certificate_authority_data,
            &cluster.certificate_authority,
        ) {
            (Some(data), _) => Some(decode_pem(data)?),
            (None, Some(path)) => Some(read_referenced(source, path)?),
            (None, None) => None,
        },
        ..Default::default()
    };

    let user = context.user.as_ref().and_then(|u| {
        config
            .auth_infos
            .iter()
            .find(|a| &a.name == u)
            .and_then(|a| a.auth_info.as_ref())
    });
    if let (Some(u), None) = (&context.user, user) {
        if !u.is_empty() {
            return Err(AppError::validation(msg::CONTEXT_NO_USER.with("user", u)));
        }
    }
    let mut auth = AuthMethod::None;
    if let Some(user) = user {
        if let Some(provider) = &user.auth_provider {
            if provider.name != "oidc" {
                return Err(auth_provider_unsupported(&provider.name));
            }
            secrets.oidc = Some(provider.config.clone().into_iter().collect());
            auth = AuthMethod::Oidc;
        }
        if let Some(exec) = &user.exec {
            let command = exec.command.clone().unwrap_or_default();
            secrets.exec = Some(ExecConfig {
                api_version: exec
                    .api_version
                    .clone()
                    .unwrap_or_else(|| "client.authentication.k8s.io/v1".into()),
                command: command.clone(),
                args: exec.args.clone().unwrap_or_default(),
                env: exec
                    .env
                    .iter()
                    .flatten()
                    .filter_map(|e| Some((e.get("name")?.clone(), e.get("value")?.clone())))
                    .collect(),
                install_hint: exec.install_hint.clone(),
                interactive_mode: exec.interactive_mode.as_ref().map(|m| format!("{m:?}")),
                provide_cluster_info: exec.provide_cluster_info,
            });
            auth = AuthMethod::Exec { command };
        }
        secrets.client_certificate = match (&user.client_certificate_data, &user.client_certificate)
        {
            (Some(data), _) => Some(decode_pem(data)?),
            (None, Some(path)) => Some(read_referenced(source, path)?),
            (None, None) => None,
        };
        secrets.client_key = match (&user.client_key_data, &user.client_key) {
            (Some(data), _) => Some(decode_pem(data.expose_secret())?),
            (None, Some(path)) => Some(read_referenced(source, path)?),
            (None, None) => None,
        };
        if secrets.client_certificate.is_some() && secrets.client_key.is_some() {
            auth = pick(auth, AuthMethod::ClientCertificate);
        }
        secrets.token = match (&user.token, &user.token_file) {
            (Some(token), _) => Some(token.expose_secret().to_owned()),
            (None, Some(path)) => Some(read_referenced(source, path)?.trim().to_owned()),
            (None, None) => None,
        };
        if secrets.token.is_some() {
            auth = pick(auth, AuthMethod::Token);
        }
        if let (Some(username), Some(password)) = (&user.username, &user.password) {
            secrets.username = Some(username.clone());
            secrets.password = Some(password.expose_secret().to_owned());
            auth = pick(
                auth,
                AuthMethod::Basic {
                    username: username.clone(),
                },
            );
        }
    }

    let connection = ClusterConnection {
        server,
        tls_server_name: cluster.tls_server_name.clone(),
        insecure_skip_tls_verify: cluster.insecure_skip_tls_verify.unwrap_or(false),
        proxy_url: cluster.proxy_url.clone(),
        namespace: context.namespace.clone(),
        auth,
        source: Some(KubeconfigSource {
            context: name.to_owned(),
            pasted: source.pasted,
        }),
    };
    Ok((connection, secrets))
}

/// The method shown for a user with several: an exec plugin or an OIDC
/// provider first, as it's what really authenticates.
fn pick(current: AuthMethod, next: AuthMethod) -> AuthMethod {
    match current {
        AuthMethod::None => next,
        current => current,
    }
}

/// The PEM text of a kubeconfig's `*-data` field (base64 of the PEM).
fn decode_pem(data: &str) -> AppResult<String> {
    let compact: String = data.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(compact)
        .map_err(|e| AppError::validation(msg::KUBECONFIG_DATA_INVALID).with_detail(e))?;
    String::from_utf8(bytes)
        .map_err(|e| AppError::validation(msg::KUBECONFIG_DATA_INVALID).with_detail(e))
}

/// The gcp and azure auth providers were removed from Kubernetes.
fn auth_provider_unsupported(provider: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::KubeconfigAuthProviderUnsupported,
        msg::AUTH_PROVIDER_UNSUPPORTED.with("provider", provider),
    );
    match provider {
        "gcp" => error
            .with_hint(msg::AUTH_PROVIDER_GCP_HINT)
            .with_command("gcloud components install gke-gcloud-auth-plugin")
            .with_command("gcloud container clusters get-credentials <cluster>"),
        "azure" => error
            .with_hint(msg::AUTH_PROVIDER_AZURE_HINT)
            .with_command("kubelogin convert-kubeconfig"),
        _ => error.with_hint(msg::AUTH_PROVIDER_OTHER_HINT),
    }
}
