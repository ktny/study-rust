// 逆ポーランド記法での計算を行うCLIツール

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

/**
 * CLI定義
 */
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "ktny",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        // ファイル読み込み
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock(); // ロックするとバッファリングして読み出せるようになり高速
        run(reader, opts.verbose);
    }
}

/**
 * バッファを1行ずつ読み込んで数式を計算する
 *
 * @param reader: トレイトBufReadを実装している任意の型
 * @param verbose: 処理の詳細を出力するか
 */
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

/**
 * 数式計算処理を行う構造体
 *
 * bool: 処理詳細を出力するか
 */
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    /**
     * 数式を計算する。内部処理はeval_innerで行う
     *
     * @param formula: 数式文字列
     * @return 計算結果
     */
    pub fn eval(&self, formula: &str) -> i32 {
        // スペースで区切ってトークンリストにする
        let mut tokens = formula.split_whitespace().rev().collect();
        self.eval_inner(&mut tokens)
    }

    /**
     * 数式を計算する
     *
     * @param toekns: 数式の要素となるトークンリスト
     * @return 計算結果
     */
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        // トークンから読み出した要素をスタックするリスト
        let mut stack = Vec::new();

        // トークンの末尾から1つずつ読み出す
        while let Some(token) = tokens.pop() {
            // 数値であればスタックに積む
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            // 演算子であればスタックから2つ要素を取り出し計算して結果をスタックに積む
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            // -vオプションが指定されている場合はこの時点のトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}
