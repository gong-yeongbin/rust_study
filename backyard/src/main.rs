use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // [7.1] 패키지와 크레이트
    /*  크레이트 : 러스트가 한 번의 컴파일 시에 고려하는 가장 작은 코드 단위
            - 바이너리 크레이트 : 실행 파일로 컴파일할 수 있는 프로그램(main 함수 포함)
            - 라이브러리 크레이트(라이브러리) : 여러 프로젝트에서 공용될 의도로 만들어진 기능들이 정의(main 함수 X)
        크레이트 루트 : 러스트 컴파일러가 컴파일을 시작하는 소스 파일
        패키지 : 일련의 기능을 제공하는 하나 이상의 크레이트로 구성된 번들 */

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
