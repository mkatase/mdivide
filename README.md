# Mdivide (Markdown Divider)

![GitHub release (latest by date)](https://img.shields.io/github/v/release/mkatase/mdivide)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/github/license/mkatase/mdivide)

## Overview
A line-based multi-language divider for Markdown using @tag syntax by Rust.

## About
Mdivide ensures the integrity of multi-language documents by strictly enforcing "line-beginning @tag" rules. It is designed for developers who manage multiple translations within a single source file and require reliable extraction.

## Environment
- cargo 1.95 on Fedora 44 (6.19.14-300)

## Build and Install

```bash
$ cargo install --git https://github.com/mkatase/mdivide.git
```

## Markdown Flavor Text Format
### Example 
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

## Tag Block

| Start   | End  | Note                      |
|:--------|:-----|:--------------------------|
| @note   | @end | No Output                 |
| @common | @end | Always Output             |
| @xxx    | @end | Target Output (-l/--lang) |

- Output is enclosed area by @common+@end or @xxx+@end 
- Out of block is no output

### Filelist Format (-f / --file) 

- Write one input file path per line. 
- Lines starting with \# are treated as comments and ignored. 
- Leading and trailing whitespace on each line is automatically trimmed. 

## Usage

1. [Single In, Single Out](#usage1)
2. [Mutiple In, Mutiple Out](#usage2)
3. [Single File Check](#usage3)
4. [Filelist Check](#usage4)

### <a id="usage1"></a>Single In, Single Out
```bash
$ mdivide -l en -i InputFile -o OutputFile
or
$ mdivide --lang en --input InputFile --output OutputFile
```
### <a id="usage2"></a>Multiple In, Multiple Out
```bash
$ mdivide -l en -f FileList -d OutputDir
or
$ mdivide --lang en --file FileList --dir OutputDir
```
### <a id="usage3"></a>Single File Check
```bash
$ mdivide -i InputFile -c
or
$ mdivide --input InputFile --check
```
### <a id="usage4"></a>Filelist Check
```bash
$ mdivide  -f  FileList  -c
or
$ mdivide  --file  FileList  --check
```
## Test Case

1. [Placement of Tag](#case1)
2. [Collect Tag Pattern](#case2)
3. [Out of Block](#case3)
4. [Lack of Tag](#case4)
5. [User Defined Tag](#case5)

### <a id="case1"></a>Placement of Tag
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
tag must placed at the beginning of a line.
### <a id="case2"></a>Collect Tag Pattern
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
### <a id="case3"></a>Out of Block
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
### <a id="case4"></a>Lack of Tag
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
### <a id="case5"></a>User Defined Tag
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
## Appendix
- This file is generated by mdivide. Original is [Here](./docs/README.txt).
## ChangeLog
- ChageLog is [Here](./CHANGELOG.md)
## License
- License is [MIT](./LICENSE)
## 🎧 B.G.M.
- [VISIONS/NAQT VANE](https://www.youtube.com/watch?v=RvXi1_DddHc)
