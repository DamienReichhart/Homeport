//! Running a job from its specification, HCL2 or JSON, the way `nomad job
//! run` does but with Nomad's own endpoints: Nomad parses the HCL, plans
//! the job to show what would change, then runs it only if nothing changed
//! the job in between. Homeport keeps no copy of the specification: Nomad
//! stores it with the job.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    client::{segment, Nomad},
    error::mentions,
    jobs::namespace_query,
    messages as msg,
};

/// How a job specification is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum SpecFormat {
    Hcl2,
    Json,
}

impl SpecFormat {
    /// JSON when the text starts with `{`, HCL otherwise.
    pub fn detect(spec: &str) -> Self {
        if spec.trim_start().starts_with('{') {
            Self::Json
        } else {
            Self::Hcl2
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Hcl2 => "hcl2",
            Self::Json => "json",
        }
    }
}

/// A field a plan changes, in a task group or one of its tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadFieldChange {
    /// Such as `Task "server" › Config › image`.
    pub path: String,
    /// `Added`, `Deleted` or `Edited`.
    pub change: String,
    pub old: Option<String>,
    pub new: Option<String>,
}

/// Why some allocations of a task group can't be placed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadPlacementFailure {
    pub nodes_evaluated: u32,
    pub nodes_filtered: u32,
    pub nodes_exhausted: u32,
    /// The resources no node had enough of, such as `memory`.
    pub exhausted: Vec<String>,
    /// The constraints that filtered nodes out, with how many each did.
    pub constraints: Vec<String>,
    /// More allocations failing the same way.
    pub coalesced: u32,
}

/// What a plan does to one task group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadPlanGroup {
    pub name: String,
    /// `Added`, `Deleted`, `Edited` or `None`.
    pub change: String,
    pub place: u32,
    pub stop: u32,
    pub in_place: u32,
    pub destructive: u32,
    pub migrate: u32,
    pub canary: u32,
    pub ignore: u32,
    pub fields: Vec<NomadFieldChange>,
    pub failure: Option<NomadPlacementFailure>,
}

/// Nomad's plan of a job, and what running it needs.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadPlan {
    pub job_id: String,
    pub namespace: String,
    pub format: SpecFormat,
    /// The job as Nomad's JSON, run as it was planned.
    pub job: Value,
    /// The job's index when planned: running fails if it moved since.
    pub job_modify_index: u32,
    /// A job no Nomad has yet.
    pub new_job: bool,
    /// `Added`, `Deleted`, `Edited` or `None`, for the job as a whole.
    pub change: String,
    pub groups: Vec<NomadPlanGroup>,
    pub warnings: Option<String>,
    /// Allocations that can't be placed, explained; running still queues them.
    pub placement: Option<AppError>,
}

/// What running a job started.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadRunResult {
    pub job_id: String,
    pub namespace: String,
    pub eval_id: Option<String>,
    pub warnings: Option<String>,
}

/// A job's specification for editing: as submitted when Nomad kept it,
/// else its JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadJobSource {
    pub source: String,
    pub format: SpecFormat,
    /// Nomad kept the specification as it was submitted.
    pub submitted: bool,
}

/// The line and column Nomad's HCL parser names, as in
/// `input.hcl:3,5-9: Unsupported argument; …`.
pub fn parse_position(text: &str) -> Option<(u32, u32)> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while let Some(colon) = text[i..].find(':').map(|p| p + i) {
        let rest = &text[colon + 1..];
        let line_len = rest.bytes().take_while(u8::is_ascii_digit).count();
        if line_len > 0 && rest.as_bytes().get(line_len) == Some(&b',') {
            let after = &rest[line_len + 1..];
            let column_len = after.bytes().take_while(u8::is_ascii_digit).count();
            if column_len > 0 {
                let line = rest[..line_len].parse().ok()?;
                let column = after[..column_len].parse().ok()?;
                return Some((line, column));
            }
        }
        i = colon + 1;
        if i >= bytes.len() {
            break;
        }
    }
    None
}

fn parse_failed(detail: &str, at: Option<(u32, u32)>) -> AppError {
    let message = match at {
        Some((line, column)) => msg::PARSE_FAILED_AT
            .with("line", line)
            .with("column", column),
        None => msg::PARSE_FAILED.into(),
    };
    AppError::new(ErrorCode::NomadParseFailed, message)
        .with_hint(msg::PARSE_FAILED_HINT)
        .with_detail(detail)
}

fn not_a_job() -> AppError {
    AppError::new(ErrorCode::NomadParseFailed, msg::JSON_NOT_A_JOB)
        .with_hint(msg::JSON_NOT_A_JOB_HINT)
        .with_command("nomad job inspect <job>")
}

/// The job in JSON `value`: the `Job` of an API request or answer, or a
/// bare job object.
pub fn job_of_json(value: Value) -> AppResult<Value> {
    match value {
        Value::Object(mut map) => match map.remove("Job") {
            Some(job @ Value::Object(_)) => Ok(job),
            Some(_) => Err(not_a_job()),
            None if map.contains_key("ID") || map.contains_key("TaskGroups") => {
                Ok(Value::Object(map))
            }
            None => Err(not_a_job()),
        },
        _ => Err(not_a_job()),
    }
}

/// A count in a plan's JSON.
fn count(value: &Value, key: &str) -> u32 {
    value
        .get(key)
        .and_then(Value::as_u64)
        .map_or(0, |n| u32::try_from(n).unwrap_or(u32::MAX))
}

fn text(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

/// The fields of a plan's diff object, its nested objects' under their path.
fn fields(diff: &Value, path: &str, out: &mut Vec<NomadFieldChange>) {
    for field in diff
        .get("Fields")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let change = text(field, "Type").unwrap_or_default();
        if change == "None" || change.is_empty() {
            continue;
        }
        let name = text(field, "Name").unwrap_or_default();
        out.push(NomadFieldChange {
            path: if path.is_empty() {
                name
            } else {
                format!("{path} › {name}")
            },
            change,
            old: text(field, "Old"),
            new: text(field, "New"),
        });
    }
    for object in diff
        .get("Objects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if text(object, "Type").as_deref() == Some("None") {
            continue;
        }
        let name = text(object, "Name").unwrap_or_default();
        let nested = if path.is_empty() {
            name
        } else {
            format!("{path} › {name}")
        };
        fields(object, &nested, out);
    }
}

fn placement_failure(metric: &Value) -> NomadPlacementFailure {
    let keyed = |key: &str| -> Vec<String> {
        metric
            .get(key)
            .and_then(Value::as_object)
            .map(|m| {
                m.iter()
                    .map(|(k, v)| format!("{k} ({})", v.as_u64().unwrap_or(0)))
                    .collect()
            })
            .unwrap_or_default()
    };
    NomadPlacementFailure {
        nodes_evaluated: count(metric, "NodesEvaluated"),
        nodes_filtered: count(metric, "NodesFiltered"),
        nodes_exhausted: count(metric, "NodesExhausted"),
        exhausted: keyed("DimensionExhausted"),
        constraints: keyed("ConstraintFiltered"),
        coalesced: count(metric, "CoalescedFailures"),
    }
}

/// The task groups of Nomad's answer to a plan, with what changes in each.
pub fn plan_groups(answer: &Value) -> Vec<NomadPlanGroup> {
    let diffs: BTreeMap<String, &Value> = answer
        .pointer("/Diff/TaskGroups")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|g| Some((text(g, "Name")?, g)))
        .collect();
    let updates = answer
        .pointer("/Annotations/DesiredTGUpdates")
        .and_then(Value::as_object);
    let failed = answer.get("FailedTGAllocs").and_then(Value::as_object);
    let mut names: Vec<String> = diffs.keys().cloned().collect();
    for name in updates.into_iter().flat_map(|u| u.keys()) {
        if !names.contains(name) {
            names.push(name.clone());
        }
    }
    names
        .into_iter()
        .map(|name| {
            let diff = diffs.get(&name).copied().unwrap_or(&Value::Null);
            let update = updates.and_then(|u| u.get(&name)).unwrap_or(&Value::Null);
            let mut changes = vec![];
            fields(diff, "", &mut changes);
            for task in diff
                .get("Tasks")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if text(task, "Type").as_deref() == Some("None") {
                    continue;
                }
                let task_name = text(task, "Name").unwrap_or_default();
                fields(task, &format!("Task \"{task_name}\""), &mut changes);
            }
            NomadPlanGroup {
                change: text(diff, "Type").unwrap_or_else(|| "None".into()),
                place: count(update, "Place"),
                stop: count(update, "Stop"),
                in_place: count(update, "InPlaceUpdate"),
                destructive: count(update, "DestructiveUpdate"),
                migrate: count(update, "Migrate"),
                canary: count(update, "Canary"),
                ignore: count(update, "Ignore"),
                fields: changes,
                failure: failed.and_then(|f| f.get(&name)).map(placement_failure),
                name,
            }
        })
        .collect()
}

impl Nomad {
    /// The job in `spec`, as Nomad's JSON: HCL2 parsed by Nomad itself,
    /// JSON read here. A parse error points at the line Nomad named.
    pub async fn parse_job(&self, spec: &str) -> AppResult<(Value, SpecFormat)> {
        if spec.trim().is_empty() {
            return Err(AppError::validation(msg::SPEC_EMPTY));
        }
        let format = SpecFormat::detect(spec);
        let job = match format {
            SpecFormat::Json => {
                let value: Value = serde_json::from_str(spec).map_err(|e| {
                    parse_failed(
                        &e.to_string(),
                        u32::try_from(e.line())
                            .ok()
                            .zip(u32::try_from(e.column()).ok()),
                    )
                })?;
                job_of_json(value)?
            }
            // Not canonicalized, so a job without a namespace stays without
            // one and goes to the one selected; Nomad canonicalizes on plan.
            SpecFormat::Hcl2 => self
                .post(
                    "/v1/jobs/parse",
                    &[],
                    &json!({ "JobHCL": spec, "Canonicalize": false }),
                )
                .await
                .map_err(|e| {
                    if e.is(ErrorCode::BadRequest) || e.is(ErrorCode::ClusterApiError) {
                        let detail = e.detail.clone().unwrap_or_default();
                        parse_failed(&detail, parse_position(&detail))
                    } else {
                        e
                    }
                })?,
        };
        Ok((job, format))
    }

    /// Asks Nomad what running `job` would change, in `namespace` when the
    /// job names none (`default` without either).
    pub async fn plan_job(
        &self,
        mut job: Value,
        format: SpecFormat,
        namespace: Option<&str>,
    ) -> AppResult<NomadPlan> {
        let job_id = text(&job, "ID")
            .or_else(|| text(&job, "Name"))
            .ok_or_else(not_a_job)?;
        if text(&job, "ID").is_none() {
            job["ID"] = Value::String(job_id.clone());
        }
        let namespace = match text(&job, "Namespace") {
            Some(ns) => ns,
            None => {
                let ns = namespace
                    .filter(|ns| !ns.is_empty() && *ns != "*")
                    .unwrap_or("default")
                    .to_owned();
                job["Namespace"] = Value::String(ns.clone());
                ns
            }
        };
        let answer: Value = self
            .post(
                &format!("/v1/job/{}/plan", segment(&job_id)),
                &namespace_query(Some(&namespace)),
                &json!({ "Job": &job, "Diff": true }),
            )
            .await
            .map_err(|e| self.submit_error(e, &namespace))?;
        let groups = plan_groups(&answer);
        let placement = placement_problem(&job_id, &groups);
        let change = answer
            .pointer("/Diff/Type")
            .and_then(Value::as_str)
            .unwrap_or("None")
            .to_owned();
        Ok(NomadPlan {
            job_modify_index: count(&answer, "JobModifyIndex"),
            new_job: change == "Added",
            change,
            warnings: text(&answer, "Warnings"),
            placement,
            groups,
            job_id,
            namespace,
            format,
            job,
        })
    }

    /// Runs a planned job, unless it changed since the plan, keeping its
    /// specification `source` with it when Nomad can store one.
    pub async fn run_job(
        &self,
        plan: &NomadPlanToRun,
        source: Option<&str>,
    ) -> AppResult<NomadRunResult> {
        let mut body = json!({
            "Job": &plan.job,
            "EnforceIndex": true,
            "JobModifyIndex": plan.job_modify_index,
        });
        if let Some(source) = source.filter(|s| !s.trim().is_empty()) {
            body["Submission"] = json!({ "Source": source, "Format": plan.format.as_str() });
        }
        let answer: Value = self
            .post("/v1/jobs", &namespace_query(Some(&plan.namespace)), &body)
            .await
            .map_err(|e| {
                let text = e.detail.clone().unwrap_or_default();
                if mentions(
                    &text,
                    &["enforcing job modify index", "conflicting job modify index"],
                ) {
                    let message = if plan.job_modify_index == 0 {
                        msg::JOB_EXISTS.with("job", &plan.job_id)
                    } else {
                        msg::JOB_MODIFIED.with("job", &plan.job_id)
                    };
                    AppError::new(ErrorCode::NomadJobModified, message)
                        .with_hint(msg::JOB_MODIFIED_HINT)
                        .with_detail(text)
                } else {
                    self.submit_error(e, &plan.namespace)
                }
            })?;
        Ok(NomadRunResult {
            job_id: plan.job_id.clone(),
            namespace: plan.namespace.clone(),
            eval_id: text(&answer, "EvalID"),
            warnings: text(&answer, "Warnings"),
        })
    }

    /// The specification of job `id` to edit: the one submitted when
    /// Nomad kept it, else the job's JSON as Nomad's API takes it.
    pub async fn job_source(&self, namespace: Option<&str>, id: &str) -> AppResult<NomadJobSource> {
        let query = namespace_query(namespace);
        let path = format!("/v1/job/{}", segment(id));
        let job: Value = self
            .get(&path, &query)
            .await
            .map_err(|e| self.job_not_found(e, id))?;
        let version = job.get("Version").and_then(Value::as_u64).unwrap_or(0);
        let mut submission_query = query.clone();
        submission_query.push(("version", version.to_string()));
        // Nomad before 1.6, or a job submitted without its source, has none.
        if let Ok(submission) = self
            .get::<Value>(&format!("{path}/submission"), &submission_query)
            .await
        {
            if let Some(source) = text(&submission, "Source") {
                let format = match text(&submission, "Format").as_deref() {
                    Some("json") => SpecFormat::Json,
                    _ => SpecFormat::Hcl2,
                };
                return Ok(NomadJobSource {
                    source,
                    format,
                    submitted: true,
                });
            }
        }
        Ok(NomadJobSource {
            source: serde_json::to_string_pretty(&json!({ "Job": job }))?,
            format: SpecFormat::Json,
            submitted: false,
        })
    }

    /// A refusal to plan or run, said for the namespace.
    fn submit_error(&self, error: AppError, namespace: &str) -> AppError {
        if error.is(ErrorCode::NomadPermissionDenied) {
            AppError {
                detail: error.detail,
                ..AppError::new(
                    ErrorCode::NomadPermissionDenied,
                    msg::SUBMIT_DENIED
                        .with("name", self.name())
                        .with("namespace", namespace),
                )
                .with_hint(msg::SUBMIT_DENIED_HINT)
            }
        } else {
            error
        }
    }
}

/// What running a planned job needs from its plan, as the frontend sends it back.
#[derive(Debug, Clone, PartialEq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadPlanToRun {
    pub job_id: String,
    pub namespace: String,
    pub format: SpecFormat,
    pub job: Value,
    pub job_modify_index: u32,
}

/// The placement failures of a plan as an error to show, when it has some.
fn placement_problem(job: &str, groups: &[NomadPlanGroup]) -> Option<AppError> {
    let failed: Vec<&str> = groups
        .iter()
        .filter(|g| g.failure.is_some())
        .map(|g| g.name.as_str())
        .collect();
    (!failed.is_empty()).then(|| {
        AppError::new(
            ErrorCode::NomadPlacementFailed,
            msg::PLACEMENT_FAILED
                .with("job", job)
                .with("groups", failed.join(", ")),
        )
        .with_hint(msg::PLACEMENT_FAILED_HINT)
    })
}

#[cfg(test)]
mod tests;
