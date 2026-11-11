---
title: RBAC
description: クラスターの Role、バインディング、サブジェクト、各サブジェクトがどこで何をできるか、そして自分の認証情報で何ができるかを確認します。
---

Kubernetes クラスターのサイドバーにある **RBAC** は、クラスター上で誰が何をできるかを示します。読み取り専用で、Homeport が RBAC オブジェクトを作成、変更、削除することはありません（YAML として編集するには、引き続き [API リソース](/docs/clusters/api-resources/) を使えます）。

ほかのページと同じく Namespace の選択に従います。Namespace を選ぶと、その Namespace の Role と RoleBinding に加えて、どこにでも適用される ClusterRole と ClusterRoleBinding を表示します。

## Role

**Role** は、Role（それぞれの Namespace 内）と ClusterRole を、ルールの数とともに一覧表示します。開くとそのページが表示されます：

- Kubernetes が書くとおりの**ルール**：リソース、API グループ（core グループは `core`）、リソース名と動詞、または `/healthz` のような非リソース URL
- **このロールを使うバインディング**。それぞれのサブジェクトへのリンク付き

**集約**と表示された ClusterRole には `aggregationRule` があります。そのページには、ラベルセレクター、それに一致する ClusterRole が表示され、ルールにはそれらのルールが含まれます。Kubernetes 標準の `admin`、`edit`、`view` はこの仕組みで作られているため、`rbac.authorization.k8s.io/aggregate-to-edit: "true"` ラベルの付いた CRD の ClusterRole は `edit` に現れます。

## Binding

**Binding** は RoleBinding と ClusterRoleBinding を一覧表示し、それぞれが付与するロール（リンク付き）と、指定しているサブジェクト（Users、Groups、ServiceAccounts）を示します。参照するロールが存在しないバインディングには**見つかりません**と表示され、何も付与しません。

## サブジェクト

**サブジェクト**は、いずれかのバインディングが指定しているすべての対象を一覧表示します。User、Group、ServiceAccount を開くと、何ができるかをリソース × 動詞のマトリクスで、適用範囲ごとに表示します。最初に**クラスター全体**、次に各 Namespace です。その下には、それを付与しているバインディングと、その経路（直接指定か、グループ経由か）が表示されます。

Homeport は、適用されるすべてのバインディングのルールをまとめます：

- ClusterRoleBinding は、その ClusterRole をどこでも付与します。すべての Namespace、Node のようなクラスタースコープのリソース、非リソース URL が対象です。
- RoleBinding は、ClusterRole をバインドする場合でも、自分の Namespace の中だけでロールを付与します。`prod` の RoleBinding で `edit` にバインドされた ServiceAccount は、`prod` では編集でき、ほかではできません。
- Kubernetes がすべてのサブジェクトを自動的に入れるグループへのバインディングも含まれます。ServiceAccount は `system:serviceaccounts`、`system:serviceaccounts:<その Namespace>`、`system:authenticated` に属し、ユーザー `system:serviceaccount:<Namespace>:<名前>` でもあります。User は `system:authenticated` に属します。
- `*` はすべての動詞、API グループ、リソースを表し、`*/scale` はすべてのリソースのそのサブリソースを表します。リソース名のあるルールはそのオブジェクトだけに適用されます（マトリクスでは**対象のみ**に表示）。より広い行に含まれる行は省略されます。

**これはベストエフォートの表示です。** サブジェクトの権限は、API サーバーに問い合わせるのではなく、あなたの認証情報で読み取れるバインディングから計算するため、誰かになりすます権限は必要ありません。そのため、Webhook（マネージドクラスターでよく使われます）や ABAC など、ほかのオーソライザーが許可する内容や、認証機構がユーザーに与えるグループ（OIDC のクレームやクライアント証明書の組織から）は反映されないことがあります。これらは API サーバーだけが知っています。自分の認証情報について確かめるには、**ここで何ができる？**を使ってください。

Namespace を選んでいるときは、その Namespace の RoleBinding だけを読み取ります。サブジェクトの権限をすべての場所で見るには、**すべての Namespace** を選んでください。

## ここで何ができる？

**ここで何ができる？**は、Homeport が使っている認証情報について、選んだ Namespace（**すべての Namespace** のときはクラスターのデフォルト Namespace）で API サーバー自身が返す答えで、`kubectl auth can-i --list` の表示と同じです。それを教えてくれるクラスター（Kubernetes 1.28 以降）では、どのユーザーとしてログインし、どのグループに属しているかを示し、そのサブジェクトのページへのリンクも表示します。

API サーバーが一覧は不完全な可能性があると示した場合（Webhook のようなオーソライザーは、一覧表示できる以上のことを許可する場合があります）、ページにその旨が表示されます。

**…できる？**は、`kubectl auth can-i` と同じように 1 つの質問をします。動詞、リソース、Namespace（空欄ですべての Namespace）を指定し、必要に応じて API グループ、サブリソース、オブジェクト名も指定できます。リソースは複数形（`secrets`）、短縮名（`deploy`）、グループ付きの複数形（`deployments.apps`）で指定でき、Homeport は kubectl と同じように解決します。答えには、同じ質問をコピーできるコマンドが付きます。たとえば `kubectl auth can-i delete secrets --namespace prod` で、同じ答えが返ります。

あなたの認証情報で RBAC オブジェクトをまったく一覧表示できない場合、ページには拒否された内容を示すメッセージの下に**ここで何ができる？**だけが表示されます。一部だけ一覧表示できる場合（たとえば RoleBinding はできても ClusterRole はできない場合）、ページはどれかを示し、表示内容にはそれらが含まれません。
