laa-patch-rs
===

Easily apply LAA patches to EXE files.  
If the LAA flag for Windows 32-bit applications is not enabled, Windows 32-bit applications will not be able to handle more than 2GB of memory on 64-bit Windows.  
This can cause bugs and crashes in older 32-bit applications.  
To fix this, patch the binaries directly and enable the LAA flag.  
This flag is determined at run time, so I think there is no problem with the compiled binary.  

## Usage

run `laa-patch-rs.exe`, then choose the EXE file that you want to LAA patch.  
You can optionally choose to output the patch application as a othet EXE file or overwrite itself.  

## Build

required `Rust 1.6x nightly`  
clone this repository then `cargo build --release`  

## For Japanese

FFXのリマスター版にて、メモリの問題で進行不能バグが見つかっています。  
これはメモリの扱いを決めるフラグに問題があり、直接パッチを当ててフラグを有効にすることで、扱えるメモリの上限を増やして問題を回避することが出来ます。  
Steamのレビューページなどでは、パッチをするためのEXEファイルへのリンクが貼られたりしていますが、中身が不明なバイナリをなんでもかんでも実行するのはよくありません。  
そのため、ソースを公開し、何をしているのかを明確にしたパッチツールとして作られました。  
Releaseにはビルド済みバイナリを添付していますが、上記よりこちらも自己責任となります。  
自分でビルドしたい方は、Rust言語のツールチェーンをインストール後、こちらのリポジトリをcloneし、 `cargo run --release` したり等各自で実行してください。  

## Credit

~~**EXEのパッチ気持ちよすぎだろ！**~~
