use std::collections::BTreeMap;

use k8s_openapi::{
    api::core::v1::{
        Container, ContainerPort, PodCondition, PodSpec, PodStatus, ServicePort, ServiceSpec,
    },
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};

use super::*;

fn service(target_port: Option<IntOrString>) -> Service {
    Service {
        spec: Some(ServiceSpec {
            selector: Some(BTreeMap::from([("app".into(), "api".into())])),
            ports: Some(vec![ServicePort {
                port: 80,
                target_port,
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn pod(name: &str, phase: &str, ready: bool) -> Pod {
    Pod {
        metadata: ObjectMeta {
            name: Some(name.into()),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: "api".into(),
                ports: Some(vec![ContainerPort {
                    name: Some("http".into()),
                    container_port: 8080,
                    ..Default::default()
                }]),
                ..Default::default()
            }],
            ..Default::default()
        }),
        status: Some(PodStatus {
            phase: Some(phase.into()),
            conditions: Some(vec![PodCondition {
                type_: "Ready".into(),
                status: if ready { "True" } else { "False" }.into(),
                ..Default::default()
            }]),
            ..Default::default()
        }),
    }
}

#[test]
fn a_service_port_goes_to_its_numeric_target_port() {
    let (selector, target) = service_target(&service(Some(IntOrString::Int(8080))), 80).unwrap();
    assert_eq!(selector, "app=api");
    assert_eq!(target, TargetPort::Number(8080));
    // Without a target port, the Service's own port.
    assert_eq!(
        service_target(&service(None), 80).unwrap().1,
        TargetPort::Number(80)
    );
}

#[test]
fn a_named_target_port_is_found_in_the_pod() {
    let (_, target) =
        service_target(&service(Some(IntOrString::String("http".into()))), 80).unwrap();
    assert_eq!(target, TargetPort::Name("http".into()));
    assert_eq!(pod_port(&pod("a", "Running", true), &target), Some(8080));
    assert_eq!(
        pod_port(
            &pod("a", "Running", true),
            &TargetPort::Name("metrics".into())
        ),
        None
    );
}

#[test]
fn a_service_without_that_port_or_a_selector_is_refused() {
    assert_eq!(
        service_target(&service(None), 443),
        Err(ServiceProblem::NoSuchPort(443))
    );
    let mut headless = service(None);
    headless.spec.as_mut().unwrap().selector = None;
    assert_eq!(
        service_target(&headless, 80),
        Err(ServiceProblem::NoSelector)
    );
}

#[test]
fn only_a_ready_running_pod_is_picked() {
    let pods = [
        pod("c", "Running", true),
        pod("a", "Pending", false),
        pod("b", "Running", false),
        pod("d", "Running", true),
    ];
    assert_eq!(
        ready_pod(&pods).and_then(|p| p.metadata.name.as_deref()),
        Some("c")
    );
    assert!(ready_pod(&pods[1..3]).is_none());
}

#[tokio::test]
async fn a_free_port_is_picked_and_a_taken_one_refused() {
    let (listeners, port) = bind(None).await.unwrap();
    assert_ne!(port, 0);
    assert!(listeners
        .iter()
        .all(|l| l.local_addr().unwrap().ip().is_loopback()));
    assert!(!is_free(port).await);
    let e = bind(Some(port)).await.unwrap_err();
    assert_eq!(e.code, ErrorCode::PortInUse);
    drop(listeners);
    assert!(is_free(port).await);
}
