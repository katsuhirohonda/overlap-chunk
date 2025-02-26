// overlap-chunk
// テキストをチャンクに分割するライブラリ

/// チャンク分割のオプション設定
#[derive(Debug, Clone)]
pub struct ChunkOptions {
    /// オーバーラップの割合（0〜100%）
    pub overlap_percentage: u8,
    // 将来的に追加予定のオプション：
    // pub respect_word_boundaries: bool,
    // pub respect_sentence_boundaries: bool,
}

impl Default for ChunkOptions {
    fn default() -> Self {
        ChunkOptions {
            overlap_percentage: 0,
            // respect_word_boundaries: false,
            // respect_sentence_boundaries: false,
        }
    }
}

/// テキストを指定されたサイズでチャンクに分割する
///
/// # 引数
///
/// * `text` - 分割する元のテキスト
/// * `chunk_size` - 各チャンクの最大サイズ（文字数）
/// * `options` - チャンク分割のオプション設定（省略可能）
///
/// # 戻り値
///
/// 分割されたテキストチャンクのベクター
///
/// # 例
///
/// ```
/// use overlap_chunk::{chunk_text, ChunkOptions};
///
/// let text = "これはテストテキストです。長いテキストを小さなチャンクに分割します。";
/// let chunks = chunk_text(text, 10, None);
/// assert_eq!(chunks.len(), 5);
///
/// let options = ChunkOptions {
///     overlap_percentage: 50,
///     ..Default::default()
/// };
/// let chunks_with_overlap = chunk_text(text, 10, Some(options));
/// assert_eq!(chunks_with_overlap.len(), 9);
/// ```
pub fn chunk_text(text: &str, chunk_size: usize, options: Option<ChunkOptions>) -> Vec<String> {
    if text.is_empty() || chunk_size == 0 {
        return vec![];
    }

    let options = options.unwrap_or_default();
    
    // 文字単位での処理のためにcharのベクターに変換
    let chars: Vec<char> = text.chars().collect();
    let total_chars = chars.len();
    
    if total_chars <= chunk_size {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();

    // オーバーラップサイズを計算
    let overlap_size = (chunk_size as f64 * options.overlap_percentage as f64 / 100.0).round() as usize;
    
    // オーバーラップを考慮したステップサイズを計算
    let step_size = if overlap_size >= chunk_size {
        1 // 最小ステップサイズ
    } else {
        chunk_size - overlap_size
    };

    let mut start = 0;
    
    while start < total_chars {
        let end = (start + chunk_size).min(total_chars);
        
        // 現在のチャンクを文字から文字列に変換して追加
        let chunk: String = chars[start..end].iter().collect();
        chunks.push(chunk);

        // 次のチャンクの開始位置を計算
        start += step_size;
        
        // 最後のチャンクを作成した場合は終了
        if start >= total_chars {
            break;
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text() {
        let chunks = chunk_text("", 10, None);
        assert_eq!(chunks.len(), 0);
    }

    #[test]
    fn test_text_smaller_than_chunk() {
        let text = "小さなテキスト";
        let chunks = chunk_text(text, 20, None);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_no_overlap() {
        let text = "これはテストテキストです。長いテキストを小さなチャンクに分割します。";
        let chunks = chunk_text(text, 10, None);
        
        // 正確なチャンク数を確認
        assert_eq!(chunks.len(), 5);
        
        // 各チャンクの内容を確認
        assert_eq!(chunks[0], "これはテストテキスト");
        assert_eq!(chunks[1], "です。長いテキス");
        assert_eq!(chunks[2], "トを小さなチャン");
        assert_eq!(chunks[3], "クに分割しま");
        assert_eq!(chunks[4], "す。");
    }

    #[test]
    fn test_with_overlap() {
        let text = "これはテストテキストです。長いテキストを小さなチャンクに分割します。";
        let options = ChunkOptions {
            overlap_percentage: 50,
            ..Default::default()
        };
        let chunks = chunk_text(text, 10, Some(options));
        
        // オーバーラップがあるので、チャンク数が増える
        assert!(chunks.len() > 5);
        
        // 最初のチャンクを確認
        assert_eq!(chunks[0], "これはテストテキスト");
        
        // 2番目のチャンクが最初のチャンクとオーバーラップしていることを確認
        assert!(chunks[1].starts_with("テキスト"));
    }

    #[test]
    fn test_full_overlap() {
        let text = "これはテストテキストです。";
        let options = ChunkOptions {
            overlap_percentage: 100,
            ..Default::default()
        };
        let chunks = chunk_text(text, 5, Some(options));
        
        // 完全オーバーラップ（ステップサイズ1）の場合、チャンク数は (文字数 - チャンクサイズ + 1)
        assert_eq!(chunks.len(), text.chars().count() - 5 + 1);
        
        // 各チャンクが1文字ずつずれていることを確認
        assert_eq!(chunks[0], "これはテス");
        assert_eq!(chunks[1], "はテスト");
    }
}
