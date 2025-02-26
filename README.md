# overlap-chunk
A Rust library for splitting text into chunks of specified size with adjustable overlap percentage.

## Features
### Current Features
* Basic functionality to split text into chunks of specified size
* Option to adjust the overlap percentage between chunks
* Command-line interface for easy text processing
  
### Future Features
* Chunking that respects word boundaries and sentence boundaries
* Support for multilingual text
* Support for streaming input

## Library Usage
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

## Command Line Usage
The library includes a command-line interface for processing text files:

```
Usage: overlap-chunk [OPTIONS] [FILE]
  If no file is specified, read from standard input

Options:
  -h, --help              Display this help message
  -s, --size SIZE         Specify chunk size (default: 100)
  -o, --overlap PERCENT   Specify overlap percentage between 0 and 90 (default: 0)
```

### Examples

Process a file with default settings:
```bash
overlap-chunk myfile.txt
```

Process a file with custom chunk size and overlap:
```bash
overlap-chunk -s 50 -o 30 myfile.txt
```

Process standard input:
```bash
cat myfile.txt | overlap-chunk -s 50
```

## License
MIT License