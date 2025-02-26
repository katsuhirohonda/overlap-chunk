# overlap-chunk
A Rust library for splitting text into chunks of specified size with adjustable overlap percentage.

## Features

### Current Features
* Basic functionality to split text into chunks of specified size
* Option to adjust the overlap percentage between chunks
  
### Future Features
* Chunking that respects word boundaries and sentence boundaries
* Support for multilingual text
* Support for streaming input

## Usage Examples

```rust
use overlap_chunk::ChunkOptions;
use overlap_chunk::chunk_text;

fn main() {
    let text = "This is a test text. We will split this long text into smaller chunks.";
    
    // Chunk splitting with default options (no overlap)
    let chunks = chunk_text(text, 10, None);
    println!("{:?}", chunks);
    
    // Chunk splitting with overlap (50% overlap)
    let options = ChunkOptions {
        overlap_percentage: 50,
        ..Default::default()
    };
    let chunks_with_overlap = chunk_text(text, 10, Some(options));
    println!("{:?}", chunks_with_overlap);
}
```

## License
MIT License