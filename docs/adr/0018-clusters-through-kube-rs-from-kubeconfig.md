# Clusters through kube-rs, from kubeconfig contexts

## Context

v0.15.0 adds Kubernetes Clusters: Homeport must reach every cluster a `kubectl` user already can (local ones, EKS, GKE and AKS through their exec plugins, OIDC users), keep their credentials as safely as SSH keys (ADR 0002), and install nothing on a cluster.

## Decision

- **kube-rs in its own crate.** `homeport-clusters` talks to the Kubernetes API with `kube` and `k8s-openapi` (the `earliest` schema), on rustls and ring like the rest of the app, so no other crate carries them (ADR 0005). No `kubectl`, no agent on the cluster.
- **A Cluster is a kubeconfig context, imported.** Its server, TLS server name, proxy and namespace go in clear JSON; every credential (CA, client certificate and key, token or token file, basic auth, an OIDC auth-provider, an exec plugin with its env) is sealed. Files a kubeconfig refers to are read once and inlined; a pasted kubeconfig is parsed in memory and never written to disk. The kubeconfig is never changed; **Refresh from kubeconfig** reads the context again. The removed `gcp` and `azure` auth providers are refused with how to switch.
- **The client is built in memory** from the stored connection and the unsealed credentials, never from a kubeconfig file. Each Cluster gets a supervisor like a Host's: `/version` on connect and every 15 s, the Hosts' backoff for transient failures, and a stop on refused credentials, unusable certificates, a missing or interactive exec plugin, lost credentials or a missing keychain.
- **Exec plugins are run by Homeport**, not by kube-rs, following the ExecCredential protocol: never interactive, `PATH` widened with the usual cloud CLI folders, a timeout, stderr kept for Details. What they print stays in memory until shortly before it expires, and is asked again once on a 401. OIDC ID tokens are refreshed by kube-rs in memory.
- **Read-only browsing, live.** Lists come from paged `list` calls mapped to small views; metadata-only watches tell an open list it changed. A 403 becomes `Forbidden` naming the verb and resource, so a page says what isn't allowed and the others work. Secret values never leave the backend except one key at a time, on Reveal, and are kept in no cache.

## Consequences

- Cloud clusters work as they do for `kubectl`, with the same plugins, and their tokens never touch the disk; a plugin that needs a browser sign-in asks the user to run `kubectl` once.
- A cluster recreated with new certificates (kind, k3d, minikube) needs a refresh, which the error says.
- Operating clusters (v0.16.0) and Helm (v0.17.0) build on the same client and errors.
