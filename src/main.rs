extern crate rand;

// use: preludeになる型をスコープに導入
// std: standard library
// io: input/output library
use rand::Rng;
use std::cmp::Ordering;
use std::io; // create Rng trait

fn main() {
    println!("Guess the number!");

    // rand::thread_rng() creates randum number generator
    // gen_range method create random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let: 変数を生成
    // mut: 変数を可変にする
    // String サイズ可変、UTF-8エンコードされたテキスト片
    // String::new newがStringの関連関数であることを表す
    // 関連関数(静的メソッド)　特定のオブジェクトよりも型に対して実装された関数
    // new関数　空の文字列を生成
    let mut guess = String::new();

    // io::stdin関数はstd::io::Stdinオブジェクトを返す
    // read_lineは標準入力を受け付けて文字列に格納する
    // &mut guess 格納する文字列（＝そのため可変）
    // & はこの引数が参照であることを表す
    // →データを複数回メモリにコピーせずとも、複数箇所から同じデータにアクセス可能
    io::stdin()
        .read_line(&mut guess)
        // expectメソッドは、io::Result型がErr値ならプログラムをクラッシュさせて引数として渡したメッセージを表示
        // Ok値ならOk列挙子が保持する返り値をただ返すだけ
        .expect("Failed to read line");
    // read_lineは値を返す（今回はio_::Result）
    // Result型が標準ライブラリに多くある
    // Result型は列挙型で、enum(イーナム)と呼ばれる
    // 列挙型とは、固定された種類の値を持つ型
    // 列挙型の値はenumの列挙子（variant）と呼ぶ
    // Result型の列挙師はOk, Err

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
