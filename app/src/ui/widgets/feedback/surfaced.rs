//! A failure as it reaches the user: the [`AppError`], the use case that
//! failed and when. It is what the error widgets show and what the app log
//! gets, as `src/shared/api/errors.ts`, `invoke.ts` and `log.ts` did:
//! making one writes the log line, so a failure can't be shown unlogged.

use homeport_core::error::{AppError, ErrorCode, Hint};
use jiff::Timestamp;

/// A failure, tagged with where and when it happened.
#[derive(Debug, Clone, PartialEq)]
pub struct Surfaced {
    pub error: AppError,
    /// The use case that failed, by its function's name (`containers_act`);
    /// `None` for a failure that started in the window itself.
    pub command: Option<&'static str>,
    pub at: Timestamp,
}

/// The app, for the Details dialog and bug reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct About {
    pub name: String,
    pub version: String,
    pub os: String,
    pub arch: String,
}

impl About {
    /// `Homeport 1.2.0 on linux x86_64`, the first line of a report.
    fn line(&self) -> String {
        format!(
            "{} {} on {} {}",
            self.name, self.version, self.os, self.arch
        )
    }
}

/// `permission_denied`: the code as the log and bug reports write it.
pub fn code_name(code: ErrorCode) -> String {
    serde_json::to_value(code)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| format!("{code:?}"))
}

/// The hint's commands as shell prompt lines (`  $ …`), one per line of
/// each command.
pub fn command_lines(hint: &Hint) -> Vec<String> {
    hint.commands
        .iter()
        .flat_map(|command| command.split('\n'))
        .map(|line| format!("  $ {line}"))
        .collect()
}

impl Surfaced {
    /// The failure of the use case `command`, now. Writes it to the app log.
    pub fn new(error: AppError, command: &'static str) -> Self {
        Self::at(error, Some(command), Timestamp::now()).logged()
    }

    /// A failure that started in the window (no use case to name). Logged.
    pub fn in_window(error: AppError) -> Self {
        Self::at(error, None, Timestamp::now()).logged()
    }

    /// The same, at a given time and without logging: for tests and for a
    /// failure that was logged where it happened.
    pub fn at(error: AppError, command: Option<&'static str>, at: Timestamp) -> Self {
        Self { error, command, at }
    }

    fn logged(self) -> Self {
        log::warn!("{}", self.log_line());
        self
    }

    /// One line for the app log: where, the code, the message in English
    /// and the raw detail.
    pub fn log_line(&self) -> String {
        let detail = self.error.detail.as_deref().map_or_else(String::new, |d| {
            let flat: Vec<&str> = d.split_whitespace().collect();
            format!(" ({})", flat.join(" "))
        });
        format!(
            "{}: [{}] {}{detail}",
            self.command.unwrap_or("window"),
            code_name(self.error.code),
            self.error.message,
        )
    }

    /// Everything known about the failure as plain text, ready to paste
    /// into a bug report: in English, whatever the app's language.
    pub fn report(&self, about: Option<&About>) -> String {
        let mut lines = vec![
            about.map_or_else(|| "Homeport".to_owned(), About::line),
            // As JavaScript's `toISOString()`: UTC, always with milliseconds.
            format!("When: {}", self.at.strftime("%Y-%m-%dT%H:%M:%S%.3fZ")),
        ];
        lines.extend(self.command.map(|command| format!("Command: {command}")));
        lines.push(format!("Code: {}", code_name(self.error.code)));
        lines.push(String::new());
        lines.push(self.error.message.clone());
        if let Some(hint) = &self.error.hint {
            lines.push(String::new());
            lines.push(format!("What to do: {}", hint.text));
            lines.extend(command_lines(hint));
        }
        // An empty detail says nothing: no block for it.
        if let Some(detail) = self.error.detail.as_ref().filter(|d| !d.is_empty()) {
            lines.push(String::new());
            lines.push("Detail:".to_owned());
            lines.push(detail.clone());
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::messages as msg;

    use super::*;

    fn refused() -> Surfaced {
        let error = AppError::new(ErrorCode::Io, msg::IO_FAILED)
            .with_hint(msg::UNREADABLE_DATA)
            .with_command("ls -la\nwhoami")
            .with_detail("Permission denied\n  (os error 13)");
        Surfaced::at(
            error,
            Some("containers_act"),
            "2026-03-04T05:06:07.0891Z".parse().unwrap(),
        )
    }

    #[test]
    fn the_log_line_names_the_command_the_code_and_the_flattened_detail() {
        let surfaced = refused();
        assert_eq!(
            surfaced.log_line(),
            format!(
                "containers_act: [io] {} (Permission denied (os error 13))",
                surfaced.error.message
            )
        );
        let bare = Surfaced::at(
            AppError::new(ErrorCode::Json, msg::IO_FAILED),
            None,
            surfaced.at,
        );
        assert_eq!(
            bare.log_line(),
            format!("window: [json] {}", bare.error.message)
        );
    }

    #[test]
    fn the_report_is_in_english_with_everything_known() {
        let surfaced = refused();
        let about = About {
            name: "Homeport".to_owned(),
            version: "1.2.0".to_owned(),
            os: "linux".to_owned(),
            arch: "x86_64".to_owned(),
        };
        let hint = surfaced.error.hint.as_ref().unwrap().text.clone();
        assert_eq!(
            surfaced.report(Some(&about)),
            format!(
                "Homeport 1.2.0 on linux x86_64\nWhen: 2026-03-04T05:06:07.089Z\n\
                 Command: containers_act\nCode: io\n\n{}\n\nWhat to do: {hint}\n  \
                 $ ls -la\n  $ whoami\n\nDetail:\nPermission denied\n  (os error 13)",
                surfaced.error.message
            )
        );
        // A whole second still shows its milliseconds, and an empty detail
        // is no detail (`if (error.detail)`).
        let plain = Surfaced::at(
            AppError::new(ErrorCode::NotFound, msg::IO_FAILED).with_detail(""),
            None,
            "2026-03-04T05:06:07Z".parse().unwrap(),
        );
        assert_eq!(
            plain.report(None),
            format!(
                "Homeport\nWhen: 2026-03-04T05:06:07.000Z\nCode: not_found\n\n{}",
                plain.error.message
            )
        );
    }
}
