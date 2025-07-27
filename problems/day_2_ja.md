## シャトルのクリスマスコードハント2024

---

[< 全てのチャレンジに戻る](https://console.shuttle.dev/shuttlings/cch24)

## Day 2: とんでもないルーティング

*インターネットの黎明期、サンタはプレゼントを贈る魔法を効率化するため、北極に魔法のデータセンターイグルーを陽気に建てました！断熱材として輝く氷のレンガと魅惑的な雪の結晶を混ぜて作られたそのイグルーは、キャンディケイン柄のサーバーと氷の結晶の光ファイバーで満たされていました。*

✨ [*今日のVibe*](https://www.youtube.com/watch?v=2G8LO44Ax8w) ✨

![cover](https://console.shuttle.dev/api/shuttlings/assets/cch24/2.png)

---

## ⭐ タスク1: とんでもない暗号化

サンタはデータセンターイグルーのネットワークルーターを次世代のものにアップグレードしています。理由は不明ですが、彼は内部ネットワークのトラフィックを難読化するために特別なIPルーティングアルゴリズムを使用しています。（あるエルフによれば、それは単に対称暗号化のひどい実装のように見えるとのことです。）彼は今、ルーティングアルゴリズムの計算を検証するための簡単なWeb APIを実装するのにあなたの助けを必要としています。

IPv4アドレスのアルゴリズムは次のとおりです：

パケットの宛先IPを計算するには、送信元IPを取得し、*キー*アドレスを適用します。`from + key == dest`という式（ここで「`+`」は**オーバーフロー加算**）が、4つの*オクテット*それぞれに個別に適用されます。

クエリパラメータ`from`と`key`を受け取り、`dest`アドレスをテキストで応答するGETエンドポイント`/2/dest`を作成してください。

### ヒント

`u8`型のオーバーフロー加算とは、255に1を加えると0になることを意味します（値は0-255の範囲でラップアラウンドします）。

- [Axumのクエリパラメータ](https://docs.rs/axum/latest/axum/extract/struct.Query.html)
- [Actix Webのクエリパラメータ](https://actix.rs/docs/extractors/#query)
- [Rocketのクエリ文字列](https://rocket.rs/v0.5/guide/requests/#query-strings)
- [Rust docs: Ipv4Addr](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html)
- [Rust docs: u8](https://doc.rust-lang.org/std/primitive.u8.html)
- [整数オーバーフロー](https://en.wikipedia.org/wiki/Integer_overflow)

### 例

```
curl "http://localhost:8000/2/dest?from=10.0.0.0&key=1.2.3.255"

11.2.3.255
```
```
curl "http://localhost:8000/2/dest?from=128.128.33.0&key=255.0.255.33"

127.128.32.33
```

## ⭐ タスク2: 逆の道を行く

サンタは時々、ルーティングの計算が正しいかどうかも再確認したいと考えています。

代わりに`from`と`to`のパラメータを受け取り、宛先の計算に使用された`key`を返すGETエンドポイント`/2/key`を実装してください（タスク1の計算を逆にします）。

### 例

```
curl "http://localhost:8000/2/key?from=10.0.0.0&to=11.2.3.255"

1.2.3.255
```
```
curl "http://localhost:8000/2/key?from=128.128.33.0&to=127.128.32.33"

255.0.255.33
```

---

## タスク3: バージョン5に何が起こったの？（ボーナスポイント50）

時代に追いつくため、サンタはこのタイプのルーティングをIPv6パケットにも使用したいと考えています。彼は小学校レベルの算数に少し飽きてしまい、IPv6パケットでは、アルゴリズムが**オーバーフロー加算の代わりにXORを使用する**べきだと決めました。

タスク1と2と同様に、IPv6アドレスを使用して`/2/v6/dest`と`/2/v6/key`を実装してください。

### 例

```
curl "http://localhost:8000/2/v6/dest?from=fe80::1&key=5:6:7::3333"

fe85:6:7::3332
```
```
curl "http://localhost:8000/2/v6/key?from=aaaa::aaaa&to=5555:ffff:c:0:0:c:1234:5555"

ffff:ffff:c::c:1234:ffff
```

---

作者: [jonaro00](https://github.com/jonaro00)

---

## チャレンジの検証

️ ローカルバリデーターを使用

[cch24-validator](https://crates.io/crates/cch24-validator)を使用して、ローカルで開発中にプロジェクトに対してテストを実行できます！

 オンラインバリデーターを使用

最新の変更が[shuttlings-cch24](https://console.shuttle.dev/projects/proj_01JEGNV8TP4KZH15S6R5WFNBT6)にデプロイされ、目的のエンドポイントが実装されていることを確認してください。（`shuttle deploy --name shuttlings-cch24`でソリューションをデプロイできます）

*Submit*をクリックすると、このチャレンジのテストがあなたの[shuttlings-cch24](https://console.shuttle.dev/projects/proj_01JEGNV8TP4KZH15S6R5WFNBT6)プロジェクトに対して実行されます。

注意: 新しいデプロイが有効になる前に、以前のデプロイが完全に停止するのを待つ必要がある場合があります。

## 提出履歴

| 時間 | ステータス | コアタスクポイント | ボーナスポイント | ログ |
| --- | --- | --- | --- | --- |

[< 全てのチャレンジに戻る](https://console.shuttle.dev/shuttlings/cch24)
