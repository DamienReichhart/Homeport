//! A container's row, held to the React model's tests.

use homeport_engine::containers::PortBinding;

use super::*;
use crate::test_support::fixtures::container;

fn c(name: &str, stack: Option<&str>) -> Container {
    let mut c = container(name, stack, "running", 0);
    c.id = format!("{name}-id");
    c.image = format!("{name}:latest");
    c
}

#[test]
fn the_tone_follows_the_state_and_an_unhealthy_check_wins() {
    assert_eq!(tone("running", None), Tone::Success);
    assert_eq!(tone("running", Some("healthy")), Tone::Success);
    assert_eq!(tone("running", Some("starting")), Tone::Warning);
    assert_eq!(tone("running", Some("unhealthy")), Tone::Danger);
    assert_eq!(tone("paused", None), Tone::Warning);
    assert_eq!(tone("restarting", None), Tone::Warning);
    assert_eq!(tone("dead", None), Tone::Danger);
    assert_eq!(tone("exited", None), Tone::Muted);
    assert_eq!(tone("created", Some("starting")), Tone::Muted);
}

#[test]
fn it_matches_by_image_service_and_id_prefix() {
    let web = c("shop-web-1", Some("shop"));
    assert!(matches(&web, "web-1:lat"));
    assert!(matches(&web, "shop-web-1-i"));
    assert!(matches(&web, " SHOP "));
    assert!(matches(&web, ""));
    assert!(!matches(&web, "nope"));
    // The id from its start only, the service anywhere.
    assert!(!matches(&c("x", None), "-id"));
    assert!(matches(&web, "eb"));
}

#[test]
fn published_ports_are_labelled_before_internal_ones() {
    let port = |private, public| PortBinding {
        ip: None,
        private_port: private,
        public_port: public,
        protocol: "tcp".to_owned(),
    };
    let mut x = c("x", None);
    x.ports = vec![
        port(6379, None),
        port(80, Some(8080)),
        port(443, Some(8443)),
    ];
    let labels = port_labels(&x);
    let shown: Vec<&str> = labels.iter().map(|p| p.label.as_str()).collect();
    assert_eq!(shown, ["8080:80/tcp", "8443:443/tcp", "6379/tcp"]);
    assert_eq!(labels[0].public_port, Some(8080));
    assert_eq!(labels[2].public_port, None);
}

#[test]
fn a_swarm_stack_s_task_is_told_from_a_compose_project_s_container() {
    let mut task = c("shop_web.1.abc", None);
    task.labels
        .insert(STACK_NAMESPACE.to_owned(), "shop".to_owned());
    assert_eq!(stack_of(&task), Some(("shop", true)));
    assert_eq!(
        stack_of(&c("shop-web-1", Some("shop"))),
        Some(("shop", false))
    );
    assert_eq!(stack_of(&c("lonely", None)), None);
}

#[test]
fn the_health_is_not_said_twice() {
    let mut web = c("web", None);
    for (status, shown) in [
        ("Up 3 hours (healthy)", "Up 3 hours"),
        ("Up 2 minutes (unhealthy)", "Up 2 minutes"),
        ("Up 4 seconds (health: starting)", "Up 4 seconds"),
        ("Exited (0) 2 hours ago", "Exited (0) 2 hours ago"),
        ("", "running"),
    ] {
        web.status = status.to_owned();
        assert_eq!(status_without_health(&web), shown);
    }
}

#[test]
fn the_last_report_of_a_batch_is_the_usage() {
    let usage = |id: &str, cpu| ContainerUsage {
        id: id.to_owned(),
        cpu_percent: cpu,
        memory_usage: 1024,
        memory_limit: 4096,
    };
    let batch = Update::Batch {
        items: vec![
            vec![usage("a", 1.0), usage("b", 2.0)],
            vec![usage("a", 3.0)],
        ],
        dropped: 0,
    };
    let latest = latest_usage(batch).unwrap();
    assert_eq!(
        latest.len(),
        1,
        "a container that stopped is no longer in it"
    );
    assert!((latest["a"].cpu_percent - 3.0).abs() < f64::EPSILON);
    assert_eq!(latest_usage(Update::Started(1)), None);
    assert_eq!(latest_usage(Update::Ended), None);
}
