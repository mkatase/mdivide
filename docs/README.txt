@note
- Edit Time: 2026-04-30 Modified tools version
- Edit Time: 2026-03-16 Modified tools version
- Edit Time: 2026-02-24 First Edit
- mdivide -l en -i ./README.txt -o README-en.md
@end
@common
# Mdivide (Markdown Divider)

![GitHub release (latest by date)](https://img.shields.io/github/v/release/mkatase/mdivide)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/github/license/mkatase/mdivide)

@end
#-----------------------------------------------------------------------
@en
## Overview
A line-based multi-language divider for Markdown using @tag syntax by Rust.

## About
Mdivide ensures the integrity of multi-language documents by strictly enforcing "line-beginning @tag" rules. It is designed for developers who manage multiple translations within a single source file and require reliable extraction.

## Environment
@end
@jp
## 概要
マークダウン形式を拡張し、多言語化したファイルを、ターゲットとする言語向けのマークダウンファイルを出力するプログラム

## About
Mdivideは、「行頭の@タグ」という厳格なルールを設けることで、多言語ドキュメントの整合性を担保します。単一のソースファイルで複数の翻訳を管理し、確実な切り出しを必要とする開発者のために設計されています 。

## 環境
@end
@common
- cargo 1.95 on Fedora 44 (6.19.14-300)

@end
#-----------------------------------------------------------------------
@en
## Build and Install

@end
@jp
## 構築とインストール

@end
@common
```bash
$ cargo install --git https://github.com/mkatase/mdivide.git
```

@end
#-----------------------------------------------------------------------
@en
## Markdown Flavor Text Format
### Example 
@end
@jp
## マークダウンフレーバー形式
### 例題
@end
@common
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

@end
#-----------------------------------------------------------------------
@en
## Options

| Option        | Note             |
|:--------------|:-----------------|
| -l / --lang   | Target Language  |
| -i / --input  | Input File       |
| -o / --output | Output File      |
| -f / --file   | Input Filelist   |
| -d / --dir    | Output Directory |
| -c / --check  | Format Check     |
| -s / --skip   | Skip Overwite    |
| -h / --help   | Help             |

- **-i** option can only be used with the **-o** option
- **-f** option can only be used with the **-d** option
- **-c** option is available only with **-i** or **-f** option
- By default, mdivide stops if the output file already extists. Use the **-s** option to skip existing files and continue operation.

@end
@jp
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

@end
#-----------------------------------------------------------------------
@en
## Tags

| Name   | Note              |
|:-------|:------------------|
| @note  | Tag for comment   |
| @common| Tag for common    |
| @end   | Tag for Block end |
| @xxx   | User-defined Tag  |

- @note tag is comment tag and no output.
- @common tag is normally output regardless of @xxx tag.
- All tag except @end tag forms a pair with @end tag.
- **xxx** part of @xxx tag is alphabet and variable-length.
- **xxx** is a user-defined string (e.g., en, jp, etc.).

@end
@jp
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

@end
#-------------------------------------------------------------------
@en
## Tag Block

| Start   | End  | Note                      |
|:--------|:-----|:--------------------------|
| @note   | @end | No Output                 |
| @common | @end | Always Output             |
| @xxx    | @end | Target Output (-l/--lang) |

- Output is enclosed area by @common+@end or @xxx+@end 
- Out of block is no output

@end
@jp
## タグブロック

| 始点    | 終点 | 説明                 |
|:--------|:-----|:---------------------|
| @note   | @end | 出力しない           |
| @common | @end | 常時出力             |
| @xxx    | @end | 対象出力 (-l/--lang) |

- 出力は、@common+@end、もしくは、@xxx+@endで囲まれた部分のみ
- ブロック外は、出力されません

@end
#-------------------------------------------------------------------
@en
### Filelist Format (-f / --file) 

- Write one input file path per line. 
- Lines starting with \# are treated as comments and ignored. 
- Leading and trailing whitespace on each line is automatically trimmed. 

@end
@jp
### ファイルリスト形式 (-f / --file) 

- 1行につき1つの入力ファイルパスを記述します。
- \# で始まる行はコメントとして扱われ、無視されます 。
- 行前後の空白は自動的に取り除かれます（トリム処理）。

@end

#-------------------------------------------------------------------
@en
## Usage

1. [Single In, Single Out](#usage1)
2. [Mutiple In, Mutiple Out](#usage2)
3. [Single File Check](#usage3)
4. [Filelist Check](#usage4)

@end
@jp
## 使用方法

1. [シングル入力、シングル出力](#usage1)
2. [マルチ入力、マルチ出力](#usage2)
3. [シングルファイル・チェック](#usage3)
4. [ファイルリスト・チェック](#usage4)

@end
#-------------------------------------------------------------------
@en
### <a id="usage1"></a>Single In, Single Out
@end
@jp
### <a id="usage1"></a>シングル入力、シングル出力
@end
@common
```bash
$ mdivide -l en -i InputFile -o OutputFile
or
$ mdivide --lang en --input InputFile --output OutputFile
```
@end
#-------------------------------------------------------------------
@en
### <a id="usage2"></a>Multiple In, Multiple Out
@end
@jp
### <a id="usage2"></a>マルチ入力、マルチ出力
@end
@common
```bash
$ mdivide -l en -f FileList -d OutputDir
or
$ mdivide --lang en --file FileList --dir OutputDir
```
@end
#-------------------------------------------------------------------
@en
### <a id="usage3"></a>Single File Check
@end
@jp
### <a id="usage3"></a>シングルファイル・チェック
@end
@common
```bash
$ mdivide -i InputFile -c
or
$ mdivide --input InputFile --check
```
@end
#-------------------------------------------------------------------
@en
### <a id="usage4"></a>Filelist Check
@end
@jp
### <a id="usage4"></a>ファイルリスト・チェック
@end
@common
```bash
$ mdivide  -f  FileList  -c
or
$ mdivide  --file  FileList  --check
```
@end
#-------------------------------------------------------------------
@en
## Test Case

1. [Placement of Tag](#case1)
2. [Collect Tag Pattern](#case2)
3. [Out of Block](#case3)
4. [Lack of Tag](#case4)
5. [User Defined Tag](#case5)

@end
@jp
## テストケース

1. [タグ位置](#case1)
2. [正しいタグのパターン](#case2)
3. [タグブロック外の扱い](#case3)
4. [タグ不足のパターン](#case4)
5. [ユーザ定義のタグ](#case5)

@end
#-------------------------------------------------------------------
@en
### <a id="case1"></a>Placement of Tag
@end
@jp
### <a id="case1"></a>タグ位置
@end
@common
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
@end
@en
tag must placed at the beginning of a line.
@end
@jp
タグは、行頭になければ認識をしません
@end
#-------------------------------------------------------------------
@en
### <a id="case2"></a>Collect Tag Pattern
@end
@jp
### <a id="case2"></a>正しいタグのパターン
@end
@common
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
@end
#-------------------------------------------------------------------
@en
### <a id="case3"></a>Out of Block
@end
@jp
### <a id="case3"></a>タグブロック外の扱い
@end
@common
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
@end
#-------------------------------------------------------------------
@en
### <a id="case4"></a>Lack of Tag
@end
@jp
### <a id="case4"></a>タグ不足のパターン
@end
@common
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
@end
#-------------------------------------------------------------------
@en
### <a id="case5"></a>User Defined Tag
@end
@jp
### <a id="case5"></a>ユーザ定義のタグ
@end
@common
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
@end
#-------------------------------------------------------------------
@en
## Appendix
- This file is generated by mdivide. Original is [Here](./docs/README.txt).
@end
@jp
## 備考
- 本ファイルは、mdivideにて生成。元ファイルは、[こちら](./docs/README.txt)。
@end
#-------------------------------------------------------------------
@common
## ChangeLog
- ChageLog is [Here](./CHANGELOG.md)
## License
- License is [MIT](./LICENSE)
## 🎧 B.G.M.
- [VISIONS/NAQT VANE](https://www.youtube.com/watch?v=RvXi1_DddHc)
@end
