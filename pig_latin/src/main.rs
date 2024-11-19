use std::io;

/* 문자열을 피구 라틴(pig Latin)으로 변경해보세요. 각 단어의 첫 번째 자음은 단어의 끝으로
       이동하고 'ay'를 붙이므루, 'first'는  'irst-fay'가 됩니다. 모음으로 시작하는 단어는 대신 끝에
      'hay'를 붙이므로 'apple'은 'apple-hay'가 됩니다. UTF-8 인코딩에 대한 세부 사항을 명심하세요!

      모음 : a e i o u */

/* 문자열을 입력
   첫 번째 단어 자음, 모음 확인
   자음 -> 단어의 끝으로 이동 후 -[자음]ay 붙임
   모음 -> -[모음]*/

fn main() {
    let vowel = ["a", "e", "i", "o", "u"];

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let word = input.trim().to_string();
    let first_word = &word[..1];

    let pig_latin = if vowel.contains(&first_word) {
        format!("{word}-hay")
    } else {
        format!("{}-{}ay", &word[1..], &word[..1])
    };

    println!("{pig_latin}")
}















