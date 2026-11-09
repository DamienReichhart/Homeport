---
title: API リソース
description: カスタムリソースを含め、クラスターが提供するあらゆる種類のオブジェクトを、kubectl get と同じ列で閲覧します。
---

Homeport には、ワークロード、Pod、Service、Ingress、ConfigMap、Secret、Node、イベントのための専用ページがあります。クラスターのサイドバーにある **API リソース** は、それ以外のすべてを扱います: PersistentVolume とその claim、NetworkPolicy、ServiceAccount と RBAC、そしてオペレーターが CRD で追加するリソース（cert-manager の Certificate、Argo CD の Application、Prometheus の ServiceMonitor）です。

## 種類を探す

左の列には、クラスターが提供するすべての種類が、`kubectl api-resources` と同じく、API グループごとに（`core` が先頭）、各グループの優先バージョンで一覧表示されます。kind、複数形、グループで検索するか、短縮名を入力します: `pvc`、`netpol`、`sa`、`cert`。

鍵のマークは、一覧表示できない種類を示します。選択した Namespace で認証情報が一覧表示を許可されていない種類（クラスターが `SelfSubjectRulesReview` を通じて Homeport に伝えるとおり）と、API がそもそも一覧表示に対応していない少数の種類です。開くと理由が表示されます。オーソライザーが webhook であるクラスターなど、一部のクラスターはこれを伝えられません。その場合は何もマークされず、拒否された一覧にその旨が表示されます。

CRD をインストールまたは削除すると、一覧は自動で更新されます。**更新** で読み込み直すこともできます。

## オブジェクトの一覧表示

種類を選ぶと、そのオブジェクトが `kubectl get` と同じ列で一覧表示されます。どの列にするかは API サーバーが決め、CRD の `additionalPrinterColumns` も含まれます。そのため、cert-manager の Certificate には、ターミナルと同じく **Ready**、**Secret**、**Age** が表示されます。一覧は watch を通じて変更をリアルタイムに追従します。

Namespace に属する種類には、Homeport のほかのページと同じく Namespace の選択欄があります。**すべての Namespace** を選ぶと、**Namespace** 列が追加されます。

## オブジェクトのページ

オブジェクトをクリックすると、そのページが開きます:

- **概要**: Namespace、そのオブジェクトを所有するオブジェクト（それぞれのページへのリンク付き）、経過時間、ラベルとアノテーション。
- **状態**: `status` に含まれている場合。Homeport 独自のページと同じです。
- そのオブジェクトについての **イベント** と、`managedFields` を除いた **YAML**。

種類が許す場合は、**編集** で YAML を開き、プレビューとサーバーサイド適用を経て変更できます（[YAML の適用と編集](/docs/clusters/yaml/)を参照）。**削除** は、オブジェクトとクラスターの名前を示した確認の後に削除します（[削除](/docs/clusters/actions/#deleting)を参照）。一覧で複数のオブジェクトにチェックを入れると、まとめて削除できます。
