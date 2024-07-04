# minigrep
- Rust圣经入门练习项目

## Usage
- 大小写敏感查找
  ```shell
    cargo run -- to poem.txt
  ```

- 忽略大小写敏感查找
  ```shell
    IGNORE_CASE=1 cargo run -- to poem.txt 
  ```

- 命令行忽略大小写敏感查找
  ```shell
  cargo run -- to poem.txt ignore_case
  cargo run -- to poem.txt IGNORE_CASE
  ```