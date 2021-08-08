# cargo

- cargo buildかcargo checkでプロジェクトをビルド
- プロジェクトのビルドと実行はcargo run
- ビルドの結果をコードと同じディレクトリに保存するのではなく、Cargoはtarget/debugディレクトリに格納

# リリースビルド
- `cargo build --release`


# 数当てゲームをプログラムする
- `/guessing_game`
- `println!`は、文字列を画面に表示するマクロ
- `std::io`ライブラリを使用することで、ユーザ入力を受け付ける能力などの実用的な機能の多くを使用することができる
- `&`という記号は、この引数が参照であることを表す
- `io::Result`オブジェクト
  - expectメソッド
    - Err値の場合、expectメソッドはプログラムをクラッシュさせ、 引数として渡されたメッセージを表示
```
let foo = 5; // immutable
let mut bar = 5; // mutable
```

## クレートを使用して機能を追加
```Cargo.toml
[dependencies]
rand = "0.4.0"
```
- match式
- cmpメソッドは2値を比較
- .trim().parse()

## ループで複数回の予想を可能にする
- roop
## 正しい予想をした後に終了する
- break
## 不正な入力を処理する
- Result型
  - OK
  - Err
- continue