# RUST学習

## 特徴

- 高速
    - 機械語に直接コンパイルされる（仮想マシンを持たない）
    - ガベージコレクションを持たない（所有権、借用、ライフタイムにより手動でメモリ管理する必要はない）
    - ゼロコスト抽象化（言語が持つ抽象化機能を実行速度低下やメモリ使用量の増加などのコストを払わず使用できる）
- モダンな言語機能
    - 変数は標準で不変
    - filter, mapなどのコレクション操作（アダプター）
    - 代数的データ型とパターンマッチング
    - 型推論
    - トレイト（ゼロコスト抽象化によりオブジェクトへの紐付けは実行時ではなくコンパイル時に行われる）
- OSからWebアプリケーションまで実装可能
- 充実したツール群
    - cargo
- 安全性
    - メモリ安全
    - スレッド安全
- 互換性（エディション）

## 環境構築

```sh
cargo new hello
cd hello
cargo run
```

## 型

### 数値型

- 整数型: i8, i16, i32, i64, i128
- 符号なし整数型: u8, u16, u32, u64, u128
- 浮動小数点型: f32, f64

### 文字列型

- str: 文字列スライス。固定サイズで文字列そのものの変更は不可
- String: データや長さの変更が可能。こちらを使うことが多い

### タプル

異なる型の値をまとめることができる。内部の型を後から変えることはできない。

```rs
let mut t = (1, "2");
t.0 = 2;
t.1 = "3"
```

### 配列

特定の型の値の集合。サイズは固定。  
配列を参照するとスライスとして扱え、[start..end]のような範囲指定が可能になる。

```rs
let mut a = [0, 1, 2];
let b = [0; 3];
a[1] = b[1];
a[2] = b[2];
println!("{:?}", &a[1..3]);
```

### 構造体

```rs
struct Person {
    name: String,
    age: u32,
}

let p = Person {
    name: String::from("John"),
    age: 8,
}
```

### 列挙型

それぞれの列挙子にさらにデータを付与することができる。

```rs
enum Event {
    Quit,
    KeyDown(u8),
    MouseDown {x: i32, y: i32},
}

let e1 = Event::Quit;
let e2 = Event::MouseDown { x: 10, y: 10 };
```

### 標準ライブラリの型
#### Option

データが存在する場合としない場合を表現できる列挙型。

#### Result

処理の結果が成功かエラーかを表現できる列挙型。

```rs
let result: Result<i32, String> = Ok(200);

match {
    Ok(code) => println!("code: {}", code),
    Err(err) => println!("err: {}", err),
}
```

#### Vector

配列とは違い内部の要素の数を増減させることができる。

```rs
let v1 = vec![1, 2, 3, 4, 5];
let v2 = vec![0; 5];

println!("{}", v1[0]);
for e in &v1 {
    println!("{}", e);
}
```

#### Box

格納する値のサイズがコンパイル時にわかっている必要のあるスタック領域ではなくヒープ領域に格納できる。  
[T]型などなどコンパイル時にサイズのわからない型に対して使う。

```rs
fn main() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
}

// [u8]のみでは配列のサイズがわからずコンパイルエラーになる
fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
```

## 制御構文

### if 

Rustにおけるifは式。

```rs
let number = 1;
if 0 < number {
    println!("0 < number");
} else if number < 0 {
    println!("number < 0");
} else {
    println!("0 == number");
}
```

### ループ

#### loop

単純なループ。式であり、breakで戻り値を返すこともできる。

```rs
let mut count = 0;

let result = loop {
    println!("count: {}", count);
    count += 1;
    if count == 10 {
        break count;
    }
};

```

#### while

```rs
let mut count = 0;

while count < 10 {
    println!("count: {}", count);
    count += 1;
}
```

#### for

- start..end はRange型
- 配列などはIteratorトレイトが適用されている

```rs
for count in 0..10 {
    println!("count: {}", count);
}

let array = [0,1,2,3,4,5,6,7,8,9];
for element in &array {
    println!("element: {}", element);
}
```


#### match

- 値だけでなく型、値の範囲、ワイルドカードなどで広範囲に比較できる
- 列挙型をmatchで分岐する際は、網羅性の確認をしてすべてのパターンがない場合はエラーになる
- 式なので分岐処理後の結果を変数に格納できる

```rs
let i = 1;
match i {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("misc"),
}
```

## 関数

### fn

返り値はセミコロンなしの最終行。returnで返すことも可能。

```rs
fn main() {
    println!("{}", add(1, 2));
}

fn add(a: i32, b: i32) -> i32{
    a + b
}
```

### impl

- 構造体にメソッドを生やしてクラスのような扱いをすることが可能
- メソッドの戻り値に自分自身を指定することでメソッドチェーンを使うことが可能
- メソッドの第1引数が&selfでない場合は関連関数という型から直接呼ぶ関数になる

```rs
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {}.", self.age);
        self
    }

    fn new(name: &str, age: u32) -> Person {
        Person { name: String::from(name), age: age }
    }
}

let p = Person::new("Taro", 20);
p.say_name().say_age();
```

## マクロ

よく使う実装について簡易的な方法を提供している。!を末尾にして関数のように呼び出せるものが多い。

```rs
// 文字列操作
let s = concat!("A", "b2", 3);
let s = format!("{}-{:?}", s, ("D", 5));
let s = format!("{}{}", "abc", "def");

// データ出力
println!("hello {}", "world");
eprintln!("hello {}", "error");
use std::io::Write;
let mut w = Vec::new();
writeln!(&mut w, "is 123");
dbg!(w);

// 異常終了
panic!("it will panic");

// ベクタ初期化
let v = vec![1, 2, 3];

// プログラム外のリソースへのアクセス
println!("defined in file: {}", file!());
println!("defined in line: {}", line!());
println!("is test: {}", cfg!(unix));
println!("CARGO_HOME: {}", env!("CARGO_HOME"));

// アサーション
assert!(true);
assert_eq!(1, 1);
assert_ne!(1, 1);

// 実装補助
unimplemented!();
todo!();
unreachable!();
```

## トレイト

### 標準ライブラリのトレイト

自作の型に標準的な実装を自動的にderiveすることができる。

```rs
#[derive(Eq, PartialEq)]
struct A(i32);

#[derive(PartialEq, PartialOrd)]
struct B(f32);

#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;

fn main() {
    // ==, > を使用可能
    println!("{:?}", A(0) == A(1));
    println!("{:?}", B(1.0) > B(0.0));

    // Cはムーブでなくコピーされる
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;

    // Dはclone可能
    let d0 = D;
    let _d1 = d0.clone();

    // Eはデバッグプリント可能
    println!("{:?}", E);

    // Fはdefault可能
    let _f = F::default();
}
```

### traitとdyn

- traitにより様々な型に共通のメソッドを実装できる
- コンパイル時にはtraitを実装したどの型で解決するかわからないときはdynにより動的ディスパッチできるようにする

```rs
struct Dove;
struct Duck;

trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uooooooohhh!!!!!");
    }
}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }
}
```

## ジェネリクス

任意の型で動作するような処理を作ることができる。

```rs
fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

let t1 = make_tuple(1, 2);
let t2 = make_tuple("Hello", "world");
let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
let t4 = make_tuple(3, "years old");
```

## 所有権と借用

- 所有権
    - 値には所有権という概念があり、変数がそれを持っている
    - 所有権を持つ変数がスコープから外れたらその値のメモリが解放される（GCに頼らず高速化するため）
    - 1つの値に対して所有権を持つことができるのは1つの変数に限られる
    - そのため、`let b = a` のようなコードは値のコピーではなく移動になる。これをムーブセマンティクスと呼ぶ
- 借用
    - 常に値の移動になると関数への引数渡しなどで不便なので、`&a`のようにして値の参照渡しをすることができる。これを借用と呼ぶ
    - 不変な変数はいくつでも参照を渡せるが、可変な変数はその変数の生存期間（ライフタイム）と重複していい参照を渡された変数は1つだけにする必要がある

```rs
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn main() {
    let a = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let b = a; // 所有権が譲渡されるのでaを呼び出そうとすればエラーになる
    println!("{} {} {}", b.r, b.g, b.b);
}
```

## マルチスレッド

### 共有メモリ方式

- thread::spawnは引数のクロージャを新しいスレッドで実行する
    - クロージャ内に渡された変数の所有権を移行するためmoveキーワードを使用する
    - handle.join()によりスレッドの終了を待つ
- スレッド間で所有権を共有するにはArcを使用する
    - 参照カウンタが行われており0になったらメモリが解放される
- マルチスレッドで同じデータに対して書き換えを行うことはできないので、Mutexにより排他制御を行う
    - あるスレッドがlockしている間は他のスレッドはlockが完了せず待ちになる
    - dataの参照を得ているのは常に1つのスレッドだけであることが保証される

```rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
```

### メッセージパッシング

- mpsc::channel()でチャネルの作成を行う
    - 各チャネルはSender(tx)とReciver(rx)を持つ
    - Sender.sendによりチャネルへデータを送信することができる
    - Reciver.recvによりチャネルはデータを受信することができる

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // mainから各スレッドへのチャネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        // mainから各スレッドへのチャネルで受け取ったdataの要素を+1してmainスレッドに送信する
        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    // 各スレッドにdataの値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    // 各スレッドからの結果をdataに格納
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
```
