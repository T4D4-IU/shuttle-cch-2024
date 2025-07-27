# Day 2: とんでもないルーティング - 計画

この計画は、既存のActix WebプロジェクトでDay 2の課題を解決するための手順を概説します。

## 1. 理解 (Understand)

- [x] **要件分析**: `problems/day_2_ja.md` を確認済み。
    - **タスク1**: IPv4のオーバーフロー加算のためのGET `/2/dest`。
    - **タスク2**: IPv4のオーバーフロー減算のためのGET `/2/key`。
    - **タスク3 (ボーナス)**: IPv6のXOR演算のためのGET `/2/v6/dest` と `/2/v6/key`。
- [x] **既存コードの調査**:
    - `src/main.rs`: プロジェクトはActix Webフレームワークを使用している。
    - `Cargo.toml`: `shuttle-actix-web` が存在する。クエリパラメータのデシリアライズのために `serde` が必要になる。

## 2. 計画 (Plan)

### 修正するファイル: `src/main.rs`

すべての実装は `src/main.rs` 内で行います。

### ステップ1: 依存関係の追加

`Cargo.toml` に `serde` を追加します:
```toml
[dependencies]
# ... 既存の依存関係
serde = { version = "1.0", features = ["derive"] }
```

### ステップ2: `use` 文の追加

`src/main.rs` の先頭に必要なインポートを追加します:
```rust
use std::net::{Ipv4Addr, Ipv6Addr};
use actix_web::{get, web, Responder, ServiceConfig};
use serde::Deserialize;
```

### ステップ3: タスク1の実装 (`/2/dest`)

- **クエリパラメータ用の構造体を作成:**
```rust
#[derive(Deserialize)]
pub struct Ipv4DestParams {
    from: Ipv4Addr,
    key: Ipv4Addr,
}
```
- **ハンドラ関数を作成:**
```rust
#[get("/2/dest")]
async fn calculate_dest(params: web::Query<Ipv4DestParams>) -> impl Responder {
    let from_octets = params.from.octets();
    let key_octets = params.key.octets();

    let mut dest_octets = [0u8; 4];
    for i in 0..4 {
        dest_octets[i] = from_octets[i].wrapping_add(key_octets[i]);
    }

    Ipv4Addr::from(dest_octets).to_string()
}
```

### ステップ4: タスク2の実装 (`/2/key`)

- **クエリパラメータ用の構造体を作成:**
```rust
#[derive(Deserialize)]
pub struct Ipv4KeyParams {
    from: Ipv4Addr,
    to: Ipv4Addr,
}
```
- **ハンドラ関数を作成:**
```rust
#[get("/2/key")]
async fn calculate_key(params: web::Query<Ipv4KeyParams>) -> impl Responder {
    let from_octets = params.from.octets();
    let to_octets = params.to.octets();

    let mut key_octets = [0u8; 4];
    for i in 0..4 {
        key_octets[i] = to_octets[i].wrapping_sub(from_octets[i]);
    }

    Ipv4Addr::from(key_octets).to_string()
}
```

### ステップ5: タスク3の実装 (ボーナス IPv6)

- **クエリパラメータ用の構造体を作成:**
```rust
#[derive(Deserialize)]
pub struct Ipv6DestParams {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[derive(Deserialize)]
pub struct Ipv6KeyParams {
    from: Ipv6Addr,
    to: Ipv6Addr,
}
```
- **ハンドラ関数を作成:**
```rust
#[get("/2/v6/dest")]
async fn calculate_v6_dest(params: web::Query<Ipv6DestParams>) -> impl Responder {
    let from_segments = params.from.segments();
    let key_segments = params.key.segments();

    let mut dest_segments = [0u16; 8];
    for i in 0..8 {
        dest_segments[i] = from_segments[i] ^ key_segments[i];
    }

    Ipv6Addr::from(dest_segments).to_string()
}

#[get("/2/v6/key")]
async fn calculate_v6_key(params: web::Query<Ipv6KeyParams>) -> impl Responder {
    let from_segments = params.from.segments();
    let to_segments = params.to.segments();

    let mut key_segments = [0u16; 8];
    for i in 0..8 {
        key_segments[i] = from_segments[i] ^ to_segments[i];
    }

    Ipv6Addr::from(key_segments).to_string()
}
```

### ステップ6: サービス設定の更新

`main` 関数を修正して、すべての新しいサービスを登録します。

```rust
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird)
           .service(seek)
           .service(calculate_dest)
           .service(calculate_key)
           .service(calculate_v6_dest)
           .service(calculate_v6_key);
    };

    Ok(config.into())
}
```

## 3. 実装 (Implement)

- [ ] 計画されたコード変更を `src/main.rs` と `Cargo.toml` に適用する。

## 4. 検証 (Verify)

- [ ] `shuttle run` を使用してローカルでサーバーを起動する。
- [ ] 問題の説明にある例を使って、`curl` で各エンドポイントをテストする。
- [ ] `cch24-validator` を実行して、解決策が正しいことを確認する。
- [ ] `shuttle deploy --name shuttlings-cch24` でShuttleにデプロイし、オンラインバリデーターを使用する。