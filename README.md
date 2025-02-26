# overlap-chunk

テキストを指定されたサイズでチャンク分割するRustライブラリ。オーバーラップの割合を調整できます。

## 特徴

### 現在の機能
* テキストを指定されたサイズでチャンク分割する基本機能
* オーバーラップの割合を調整できるオプション

### 将来的な機能
* 単語境界やセンテンス境界を尊重したチャンキング
* 多言語テキストのサポート
* ストリーミング入力への対応

## 使用例

```rust
use overlap_chunk::ChunkOptions;
use overlap_chunk::chunk_text;

fn main() {
    let text = \"これはテストテキストです。長いテキストを小さなチャンクに分割します。\";
    
    // デフォルトオプションでチャンク分割（オーバーラップなし）
    let chunks = chunk_text(text, 10, None);
    println!(\"{:?}\", chunks);
    
    // オーバーラップありでチャンク分割（50%オーバーラップ）
    let options = ChunkOptions {
        overlap_percentage: 50,
        ..Default::default()
    };
    let chunks_with_overlap = chunk_text(text, 10, Some(options));
    println!(\"{:?}\", chunks_with_overlap);
}
```

## ライセンス

MIT License