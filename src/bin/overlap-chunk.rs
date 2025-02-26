use overlap_chunk::{chunk_text, ChunkOptions};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 引数の解析
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    // コマンドラインオプションの解析
    let mut chunk_size = 100; // デフォルトのチャンクサイズ
    let mut overlap_percentage = 0; // デフォルトはオーバーラップなし
    let mut input_file = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-s" | "--size" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(size) => chunk_size = size,
                        Err(_) => {
                            eprintln!("エラー: チャンクサイズは数値である必要があります");
                            process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("エラー: --size オプションには値が必要です");
                    process::exit(1);
                }
            }
            "-o" | "--overlap" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u8>() {
                        Ok(percentage) if percentage <= 100 => overlap_percentage = percentage,
                        _ => {
                            eprintln!("エラー: オーバーラップは0〜100の数値である必要があります");
                            process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("エラー: --overlap オプションには値が必要です");
                    process::exit(1);
                }
            }
            arg if !arg.starts_with('-') => {
                input_file = Some(arg.to_string());
                i += 1;
            }
            _ => {
                eprintln!("エラー: 不明なオプション: {}", args[i]);
                print_usage();
                process::exit(1);
            }
        }
    }

    // テキストの読み込み
    let text = match input_file {
        Some(filename) => match fs::read_to_string(&filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("ファイル '{}' の読み込みエラー: {}", filename, e);
                process::exit(1);
            }
        },
        None => {
            // 標準入力から読み込み
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => buffer,
                Err(e) => {
                    eprintln!("標準入力の読み込みエラー: {}", e);
                    process::exit(1);
                }
            }
        }
    };

    // オプションの設定
    let options = ChunkOptions {
        overlap_percentage,
    };

    // テキストのチャンク分割
    let chunks = chunk_text(&text, chunk_size, Some(options));

    // 結果の出力
    for (i, chunk) in chunks.iter().enumerate() {
        println!("チャンク {}: {}", i + 1, chunk);
    }
}

fn print_usage() {
    println!("使用方法: overlap-chunk [オプション] [ファイル]");
    println!("  ファイルを指定しない場合は標準入力から読み込みます");
    println!();
    println!("オプション:");
    println!("  -h, --help              このヘルプメッセージを表示します");
    println!("  -s, --size SIZE         チャンクサイズを指定します (デフォルト: 100)");
    println!("  -o, --overlap PERCENT   オーバーラップの割合を0〜100で指定します (デフォルト: 0)");
}
