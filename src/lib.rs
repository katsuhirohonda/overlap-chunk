#[derive(Debug, Clone)]
pub struct ChunkOptions {
    /// Overlap percentage (0-100%)
    pub overlap_percentage: u8,
}

impl Default for ChunkOptions {
    fn default() -> Self {
        ChunkOptions {
            overlap_percentage: 0,
        }
    }
}

/// Split text into chunks of specified size
///
/// # Arguments
///
/// * `text` - The text to be split
/// * `chunk_size` - Maximum size of each chunk (in characters)
/// * `options` - Optional chunking options
///
/// # Returns
///
/// A vector of text chunks
///
/// # Example
///
/// ```
/// use overlap_chunk::{chunk_text, ChunkOptions};
///
/// let text = "This is a test text. We will split this long text into smaller chunks.";
/// let chunks = chunk_text(text, 10, None);
/// assert_eq!(chunks.len(), 6);
///
/// let options = ChunkOptions {
///     overlap_percentage: 50,
///     ..Default::default()
/// };
/// let chunks_with_overlap = chunk_text(text, 10, Some(options));
/// assert_eq!(chunks_with_overlap.len(), 11);
/// ```
pub fn chunk_text(text: &str, chunk_size: usize, options: Option<ChunkOptions>) -> Vec<String> {
    if text.is_empty() || chunk_size == 0 {
        return vec![];
    }

    let options = options.unwrap_or_default();

    // Convert to character vector for proper handling
    let chars: Vec<char> = text.chars().collect();
    let total_chars = chars.len();

    if total_chars <= chunk_size {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();

    // Calculate overlap size
    let overlap_size =
        ((chunk_size as f64 * options.overlap_percentage as f64 / 100.0) + 0.5) as usize;

    // Calculate step size considering overlap
    let step_size = if overlap_size >= chunk_size {
        1 // Minimum step size
    } else {
        chunk_size - overlap_size
    };

    let mut start = 0;

    while start < total_chars {
        let end = std::cmp::min(start + chunk_size, total_chars);
        let chunk: String = chars[start..end].iter().collect();
        chunks.push(chunk);

        // Calculate start position for next chunk
        start += step_size;
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
        let text = "Small text";
        let chunks = chunk_text(text, 20, None);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_no_overlap() {
        let text = "This is a test text. We will split this long text into smaller chunks.";
        let chunks = chunk_text(text, 10, None);
        // Check the exact number of chunks
        assert_eq!(chunks.len(), 6);
        // Check the content of each chunk
        assert_eq!(chunks[0], "This is a ");
        assert_eq!(chunks[1], "test text.");
        assert_eq!(chunks[2], " We will s");
        assert_eq!(chunks[3], "plit this ");
        assert_eq!(chunks[4], "long text ");
        assert_eq!(chunks[5], "into small");
    }

    #[test]
    fn test_with_overlap() {
        let text = "This is a test text. We will split this long text into smaller chunks.";
        let options = ChunkOptions {
            overlap_percentage: 50,
            ..Default::default()
        };
        let chunks = chunk_text(text, 10, Some(options));
        assert_eq!(chunks.len(), 11);
        // Check overlap - second chunk should start with "test"
        assert!(chunks[1].starts_with("test"));
    }

    #[test]
    fn test_full_overlap() {
        let text = "This is a test text. We will split this long text into smaller chunks.";
        let options = ChunkOptions {
            overlap_percentage: 100,
            ..Default::default()
        };
        let chunks = chunk_text(text, 5, Some(options));
        // With 100% overlap (step size 1), number of chunks should be: total_chars - chunk_size + 1
        assert_eq!(chunks.len(), text.chars().count() - 5 + 1);
    }
}
