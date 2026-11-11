//! The crates' failures in the app's language: an [`AppError`] carries the
//! catalogue key of its message and of its hint (`errors.*`) with their
//! parameters, and its English text for when no catalogue has the key.

use homeport_core::error::{AppError, Hint, Param, Params};

use super::{current, Arg, Args};

fn args(params: &Params) -> Args {
    let mut args = Args::new();
    for (name, value) in params {
        let value = match value {
            Param::Text(text) => Arg::Text(text.clone()),
            // Parameters count things; none comes near 2^53.
            #[allow(clippy::cast_precision_loss)]
            Param::Number(number) => Arg::Number(*number as f64),
        };
        args.set_owned(name.clone(), value);
    }
    args
}

fn translated(key: &str, params: &Params, english: &str) -> String {
    let language = current();
    if key.is_empty() || !language.has(key) {
        return english.to_owned();
    }
    language.text(key, Some(&args(params).0))
}

/// What failed, in the app's language.
pub fn message(error: &AppError) -> String {
    translated(&error.key, &error.params, &error.message)
}

/// What the user can do about it, in the app's language.
pub fn hint(hint: &Hint) -> String {
    translated(&hint.key, &hint.params, &hint.text)
}
