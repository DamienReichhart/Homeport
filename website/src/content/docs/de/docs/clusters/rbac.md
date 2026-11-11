---
title: RBAC
description: Sehen Sie die Roles, Bindings und Subjekte eines Clusters, was jedes Subjekt wo darf, und was Ihre eigenen Zugangsdaten dürfen.
---

**RBAC** in der Seitenleiste eines Kubernetes-Clusters zeigt, wer auf dem Cluster was darf. Die Seite ist schreibgeschützt: Homeport erstellt, ändert und löscht keine RBAC-Objekte (als YAML bearbeiten können Sie sie weiterhin unter [API-Ressourcen](/docs/clusters/api-resources/)).

Wie die anderen Seiten folgt sie der Namespace-Auswahl: Mit einem gewählten Namespace zeigt sie dessen Roles und RoleBindings sowie die ClusterRoles und ClusterRoleBindings, die überall gelten.

## Roles

**Roles** listet die Roles (jede in ihrem Namespace) und die ClusterRoles mit der Anzahl ihrer Regeln. Öffnen Sie eine für ihre Seite:

- ihre **Regeln**, wie Kubernetes sie schreibt: Ressourcen, API-Gruppen (`core` für die Core-Gruppe), Ressourcennamen und Verben, oder Nicht-Ressourcen-URLs wie `/healthz`;
- die **Bindings, die sie verwenden**, jeweils mit Links zu ihren Subjekten.

Eine als **aggregiert** markierte ClusterRole hat eine `aggregationRule`: Ihre Seite zeigt deren Label-Selektoren, die ClusterRoles, auf die sie passen, und ihre Regeln enthalten deren Regeln. Die Kubernetes-eigenen `admin`, `edit` und `view` sind so gebaut; die ClusterRole einer CRD mit dem Label `rbac.authorization.k8s.io/aggregate-to-edit: "true"` erscheint daher in `edit`.

## Bindings

**Bindings** listet die RoleBindings und ClusterRoleBindings, jeweils mit der Role, die sie gewähren (verlinkt), und den Subjekten, die sie nennen: Users, Groups und ServiceAccounts. Ein Binding, dessen Role nicht existiert, ist als **nicht gefunden** markiert; es gewährt nichts.

## Subjekte

**Subjekte** listet alle, die ein Binding nennt. Öffnen Sie einen User, eine Group oder einen ServiceAccount, um zu sehen, was er darf: als Matrix aus Ressourcen und Verben, gruppiert danach, wo es gilt, zuerst **Clusterweit**, dann jeder Namespace. Darunter stehen die Bindings, die es gewähren, und wie: indem sie es direkt nennen oder über eine Gruppe.

Homeport führt die Regeln aller zutreffenden Bindings zusammen:

- Ein ClusterRoleBinding gewährt seine ClusterRole überall: in jedem Namespace, auf clusterweiten Ressourcen wie Nodes und auf Nicht-Ressourcen-URLs.
- Ein RoleBinding gewährt seine Role nur in seinem eigenen Namespace, auch wenn es eine ClusterRole bindet. Ein ServiceAccount, der per RoleBinding in `prod` an `edit` gebunden ist, darf in `prod` bearbeiten und nirgends sonst.
- Bindings an die Gruppen, in die Kubernetes jedes Subjekt selbst steckt, zählen auch: Ein ServiceAccount ist in `system:serviceaccounts`, `system:serviceaccounts:<sein Namespace>` und `system:authenticated` und ist zugleich der Benutzer `system:serviceaccount:<Namespace>:<Name>`; ein User ist in `system:authenticated`.
- `*` steht für jedes Verb, jede API-Gruppe oder Ressource, `*/scale` für diese Unterressource jeder Ressource, und eine Regel mit Ressourcennamen gilt nur für diese Objekte (die Matrix zeigt sie unter **Nur**). Eine Zeile, die eine breitere schon abdeckt, entfällt.

**Dies ist eine Näherung nach bestem Wissen.** Die Berechtigungen eines Subjekts werden aus den Bindings berechnet, die Ihre Zugangsdaten lesen können, nicht beim API-Server erfragt; sie erfordern also kein Recht, sich als jemand anderes auszugeben. Was andere Autorisierer erlauben, etwa ein Webhook (häufig bei verwalteten Clustern) oder ABAC, kann fehlen, ebenso die Gruppen, die ein Authentifizierer einem Benutzer gibt (aus OIDC-Claims oder der Organisation eines Client-Zertifikats) und die nur der API-Server kennt. Um bei Ihren eigenen Zugangsdaten sicherzugehen, verwenden Sie **Was darf ich hier?**.

Mit einem gewählten Namespace werden nur dessen RoleBindings gelesen; wählen Sie **Alle Namespaces**, um die Berechtigungen eines Subjekts überall zu sehen.

## Was darf ich hier?

**Was darf ich hier?** ist die Antwort des API-Servers selbst für die Zugangsdaten, die Homeport verwendet, im gewählten Namespace (dem Standard-Namespace des Clusters bei **Alle Namespaces**), wie `kubectl auth can-i --list` sie zeigt. Die Seite nennt, als wer sie sich anmelden und in welchen Gruppen, auf Clustern, die das sagen (Kubernetes 1.28 und neuer), mit einem Link zur Seite dieses Subjekts.

Wenn der API-Server meldet, dass seine Liste unvollständig sein kann — ein Autorisierer wie ein Webhook kann mehr erlauben, als er auflisten kann —, sagt die Seite das.

**Darf ich …?** stellt eine Frage wie `kubectl auth can-i`: ein Verb, eine Ressource und einen Namespace (leer für alle Namespaces), optional eine API-Gruppe, eine Unterressource und den Namen eines Objekts. Die Ressource kann ihr Plural sein (`secrets`), ein Kurzname (`deploy`) oder ihr Plural mit ihrer Gruppe (`deployments.apps`); Homeport löst sie auf wie kubectl. Zur Antwort gehört dieselbe Frage als Befehl zum Kopieren, etwa `kubectl auth can-i delete secrets --namespace prod`, der dieselbe Antwort gibt.

Wenn Ihre Zugangsdaten gar keine RBAC-Objekte auflisten dürfen, zeigt die Seite nur **Was darf ich hier?**, unter einer Meldung, die nennt, was verweigert wurde. Wenn nur einige aufgelistet werden können, etwa RoleBindings, aber keine ClusterRoles, sagt die Seite welche, und was sie zeigt, lässt sie aus.
