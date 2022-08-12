# sample cli

逆ポーランド記法での計算を行うCLIツール

## インストール

```sh
cargo install --path .
```

## 使い方

### 標準入力

```sh
rpncalc 
> 1 1 +
2
```

### ファイル入力

```sh
rpncalc input.txt
2 # 1 1 +
21 # 1 2 + 3 4 + *
invalid syntax at 2 # 1000 * 1000
```

## テスト

```sh
cargo test
```
