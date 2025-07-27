## シャトルのクリスマスコードハント2024

---

[< 全てのチャレンジに戻る](https://console.shuttle.dev/shuttlings/cch24)

## Day -1: 帰ってきたぞ！

シャトルのクリスマスコードハント2024へようこそ！

サンタさんは、今年も皆さんが手伝いに来てくれることを、信じられないほど楽しみにしています。

このチャレンジは、Shuttlingsのコードチャレンジの解き方と、CCH24プロジェクトをShuttleにデプロイする方法に慣れるためのウォームアップチャレンジです。

![cover](https://console.shuttle.dev/api/shuttlings/assets/cch24/-1.png)

---

## ⭐ はじめに

まず、RustとShuttleで作業するためのセットアップをしましょう！

以下の手順で助けが必要な場合は、[Discord](https://discord.gg/shuttle)のCCHチャンネルで気軽に質問するか、右下のサポートチャットをご利用ください。

Rustと`cargo`コマンドをインストールするには、[rustup](https://rustup.rs/)にアクセスして指示に従ってください。インストールされたバージョンは`cargo -V`で確認できます（1.78以上が必要です）。

Shuttle CLIをインストールするには、[インストールに関するドキュメント](https://docs.shuttle.dev/getting-started/installation)のいずれかのオプションを使用してください。インストールされたバージョンは`shuttle -V`で確認できます（0.48.0以上が必要です）。

CCH24のソリューションを作成したいディレクトリに移動し、以下のコマンドでプロジェクトを初期化します。テンプレートを選択する際は、いずれかのWebフレームワークのHello Worldテンプレートを選んでください。どれを選べばいいかわからない場合は、Axum、Actix Web、Rocketをお勧めします。

```
shuttle init --name shuttlings-cch24
```

その後、生成されたディレクトリに`cd`で移動し、お好みのコードエディタで開いてください。

これでタスク1を開始する準備が整いました！

---

## チャレンジの構成

各チャレンジは、1つ以上の*コアタスク*（⭐マーク）と1つ以上の*ボーナスタスク*（ ）に分かれています。チャレンジを**完了**するには、*コアタスク*⭐のテストに合格するだけで十分です。*ボーナスタスク* は完了するのがより難しいですが、さらに多くのポイントがもらえます。これらのテストには、より多くのエッジケースや意表を突く問題が含まれています。

---

## ⭐ タスク1: Hello, bird!

*サンタさんは「Hello, world!」というフレーズに飽きてしまいました。誰もがいつも使っています！**気分を変えて…「Hello, bird!」と言ってみましょう。***なぜかって？そのうちわかるでしょう。*

先ほど実行したinitコマンドのスターターテンプレートには、GETリクエストに対して「Hello, world!」という文字列と`200 OK`ステータスコードで応答するルートエンドポイント`/`があるはずです。応答文字列を「Hello, bird!」に変更してください。

### ヒント

プロジェクトディレクトリで`shuttle run`を使うと、Shuttleアプリをローカルでテストできます（下記参照）。

**参考資料:**

- [Shuttle docs: Quick start](https://docs.shuttle.dev/getting-started/quick-start)
- [Shuttle docs: Local run](https://docs.shuttle.dev/getting-started/local-run)
- [Shuttle docs: Axum](https://docs.shuttle.dev/examples/axum)
- [Shuttle docs: Actix Web](https://docs.shuttle.dev/examples/actix)
- [Shuttle docs: Rocket](https://docs.shuttle.dev/examples/rocket)
- [Shuttle docs: Tower, Warp, Salvo, Poem, Thruster, Tide](https://docs.shuttle.dev/examples/other)
- [MDN web docs: 200 OK](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200)

### 例

CCH24全体の多くのタスクは、`curl`コマンドで簡単にテストできます。ローカルでテストする際に使用し、その後デプロイされたアプリが動作することを確認できます。

```
# `shuttle run`でアプリを実行中
curl -I -X GET http://localhost:8000/

HTTP/1.1 200 OK
...
```
```
curl http://localhost:8000/

Hello, bird!
```

もしすべてが順調に見え、早速 ✨*素敵な架空のポイント*✨ を獲得したい場合は、このページの下部までスクロールして、ローカルバリデーターとShuttleで検証を試みてください！

---

## タスク2: 求めよ、さらば与えられん（ボーナスポイント0）

*サンタさんは、去年から北極がひどく静かになったと考えています。**「陽気な曲をいくつか紹介する時が来た！✨今日のVibe✨と名付けよう。**さて、最初の曲へのリンクはどこに置いたかな？」*

このボーナスタスクでは、`/-1/seek`にエンドポイントを追加し、GETリクエストに対して`302 Found`リダイレクト応答で応答するようにしてください。

`302`応答が有効であるためには、リクエストされたリソースが見つかる場所を示す`Location`ヘッダーが設定されている必要があります。今日サンタさんがリクエストしたLocationヘッダーの値は次のとおりです: `https://www.youtube.com/watch?v=9Gc4QTqslN4`

### ヒント

- [Axum Responses](https://docs.rs/axum/latest/axum/response/index.html)
- [Actix Web Responses](https://actix.rs/docs/response)
- [Rocket Responses](https://rocket.rs/v0.5/guide/responses/)
- [MDN web docs: 302 Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/302)

### 例

```
curl -I -X GET http://localhost:8000/-1/seek

HTTP/1.1 302 Found
location: https://www.youtube.com/watch?v=9Gc4QTqslN4
...
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

✅ このチャレンジを完了しました！[Xで投稿する!](https://x.com/intent/tweet?text=I+just+completed+challenge+-1+on+Shuttle%27s+Christmas+Code+Hunt+2024%0A%0Ahttps%3A%2F%2Fwww.shuttle.dev%2Fcch%0A%0A%40shuttle_dev+%23shuttlings+%23cch24)

## 提出履歴

| 時間 | ステータス | コアタスクポイント | ボーナスポイント | ログ |
| --- | --- | --- | --- | --- |
| 2024/12/7 23:42:24 | 完了 | 0 | 0 | `Day -1: 検証中... タスク1: 完了 Day -1: コアタスク完了 ✅ タスク2: 完了 ` |
| 2024/12/7 23:40:52 | 完了 | 0 | 0 | `Day -1: 検証中... タスク1: テスト #1 失敗 ` |

[< 全てのチャレンジに戻る](https://console.shuttle.dev/shuttlings/cch24)
