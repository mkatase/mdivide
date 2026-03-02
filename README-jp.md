# Mdivide (Markdown Divider)

![GitHub release (latest by date)](https://img.shields.io/github/v/release/mkatase/mdivide)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/github/license/mkatase/mdivide)

## 概要
マークダウン形式を拡張し、多言語化したファイルを、ターゲットとする言語向けのマークダウンファイルを出力するプログラム

## About
Mdivideは、「行頭の@タグ」という厳格なルールを設けることで、多言語ドキュメントの整合性を担保します。単一のソースファイルで複数の翻訳を管理し、確実な切り出しを必要とする開発者のために設計されています 。

## 環境
- cargo 1.93 on Fedora 43 (6.18.10-200)

## 構築とインストール

```bash
$ cargo install --git https://github.com/mkatase/mdivide.git
```

## マークダウンフレーバー形式
### 例題
```bash
 @en
 one two three four
 @end
 @jp
 一　二　三　四
 @end
 @common
 1 2 3 4
 @end
```

## オプション

| Option        | Note                 |
|:--------------|:---------------------|
| -l / --lang   | 対象タグ指定         |
| -i / --input  | 入力ファイル指定     |
| -o / --output | 出力ファイル指定     |
| -f / --file   | 入力リスト指定       |
| -d / --dir    | 出力ディレクトリ指定 |
| -c / --check  | フォーマットチェック |
| -s / --skip   | 上書きスキップ       |
| -h / --help   | ヘルプ               |

- **-i**オプションは、**-o**オプションとのみ使用可能
- **-f**オプションは、**-d**オプションとのみ使用可能
- **-c**オプションは、**-i**オプション、もしくは、**-f**オプションと使用可能
- mdivideは上書き禁止のため、対象ファイルが存在する場合は、そこでストップするが、**-s**オプションを使用すれば、継続的に動作が可能

## タグ説明

| 名称    | 備考                 |
|:--------|:---------------------|
| @note   | コメント用タグ       |
| @common | 共通部分用タグ       |
| @end    | ブロックエンド用タグ |
| @xxx    | カスタム可能タグ     |

- @noteタグは、コメント用であり、出力されません
- @commonタグは、ターゲットタグに関係なく出力されます
- @endタグは、それ以外のタグと対をなします
- @xxxタグの**xxx**部分は、英文字、かつ、可変長
- xxxには、任意の文字列を指定できます（例: @en, @jp）

## タグブロック

| 始点    | 終点 | 説明                 |
|:--------|:-----|:---------------------|
| @note   | @end | 出力しない           |
| @common | @end | 常時出力             |
| @xxx    | @end | 対象出力 (-l/--lang) |

- 出力は、@common+@end、もしくは、@xxx+@endで囲まれた部分のみ
- ブロック外は、出力されません

### ファイルリスト形式 (-f / --file) 

- 1行につき1つの入力ファイルパスを記述します。
- \# で始まる行はコメントとして扱われ、無視されます 。
- 行前後の空白は自動的に取り除かれます（トリム処理）。

## 使用方法

1. [シングル入力、シングル出力](#usage1)
2. [マルチ入力、マルチ出力](#usage2)
3. [シングルファイル・チェック](#usage3)
4. [ファイルリスト・チェック](#usage4)

### <a id="usage1"></a>シングル入力、シングル出力
```bash
$ mdivide -l en -i InputFile -o OutputFile
or
$ mdivide --lang en --input InputFile --output OutputFile
```
### <a id="usage2"></a>マルチ入力、マルチ出力
```bash
$ mdivide -l en -f FileList -d OutputDir
or
$ mdivide --lang en --file FileList --dir OutputDir
```
### <a id="usage3"></a>シングルファイル・チェック
```bash
$ mdivide -i InputFile -c
or
$ mdivide --input InputFile --check
```
### <a id="usage4"></a>ファイルリスト・チェック
```bash
$ mdivide  -f  FileList  -c
or
$ mdivide  --file  FileList  --check
```
## テストケース

1. [タグ位置](#case1)
2. [正しいタグのパターン](#case2)
3. [タグブロック外の扱い](#case3)
4. [タグ不足のパターン](#case4)
5. [ユーザ定義のタグ](#case5)

### <a id="case1"></a>タグ位置
```bash
$ more data/test1.md
\@jp
境界テスト：あいうえお@en
\@end
$ mdivide -l data/test1.md -c
[CHECK REPORT] Source: ./data/test1.md
--------------------------------------------------
  \@jp      :   1 blocks (   1 lines total)
--------------------------------------------------
```
タグは、行頭になければ認識をしません
### <a id="case2"></a>正しいタグのパターン
```bash
$ more data/test2.md
 @jp
 @end
 @en
 Content
 @end
$ mdivide -i data/test2.md -c
[CHECK REPORT] Source: data/test2.md
--------------------------------------------------
 @en      :   1 blocks (   1 lines total)
 @jp      :   1 blocks (   0 lines total)
--------------------------------------------------
$ mdivide -l jp -i data/test2.md -o jp.out
...
$ mdivide -l en -i data/test2.md -o en.out
...
$ cat jp.out 
$ cat en.out 
Content
$
```
### <a id="case3"></a>タグブロック外の扱い
```bash
$ more data/test3.md
 This is out of block (not output)
 @en
 This is content (output)
 @end
 This is also out of block (not output)
$ mdivide -i data/test3.md -c
[CHECK REPORT] Source: ./data/test3.md
--------------------------------------------------
  @en      :   1 blocks (   1 lines total)
--------------------------------------------------
$ mdivide -l en -i data/test3.md -o en.out
...
$ cat en.out 
This is content (output)
$
```
### <a id="case4"></a>タグ不足のパターン
```bash
$ more data/test4.md 
 @note
 Error Test
 @end
 @any
 @jp
 This is Test
$ mdivide -i ./data/test4.md -c`
 Processing: ./data/test4.md
 Tag error: Tag @any (line 4) in "./data/test4.md" is never closed
```
### <a id="case5"></a>ユーザ定義のタグ
```bash
$ more data/test5.md 
 @note
 Some Tag Test
 @end
 @some
 This is some tag
 @end
 @jp
 This is Test
 @end
 $ mdvide -i ./data/test5.md -c
Processing: ./data/test5.md

[CHECK REPORT] Source: ./data/test5.md
--------------------------------------------------
  @jp      :   1 blocks (   1 lines total)
  @note    :   1 blocks (   1 lines total)
  @some    :   1 blocks (   1 lines total)
--------------------------------------------------
$ mdivide -i ./data/test5.md -o some.md -l some
Processing: ./data/test5.md
  -> Targeting: "some.md"
  [DONE] Generated: "some.md"
$ more ./some.md 
This is some tag
$ 
```
## 備考
- 本ファイルは、mdivideにて生成。元ファイルは、[こちら](./docs/README.txt)。
## ChangeLog
- ChageLog is [Here](./CHANGELOG.md)
## License
- License is [MIT](./LICENSE)
## 🎧 B.G.M.
- [VISIONS/NAQT VANE](https://www.youtube.com/watch?v=RvXi1_DddHc)
