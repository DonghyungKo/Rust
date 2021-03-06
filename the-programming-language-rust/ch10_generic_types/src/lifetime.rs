// 10.4 수명을 이용해 참조 유효성 검사하기
// 러스트의 모든 참조는 수명(lifetime)을 가지고 있다.
// 수명이란, 참조가 유효한 범위를 말한다.

// 타입은 대부분 추론(inferred)에 의해서 결정됨
// 수명 역시 추론을 토대로 동작하게 된다.

// 만약 하나 이상의 타입이 바인딩될 수 있을 때는 type annotation을 추가해야 하듯
// 참조의 수명이 달라질 수 있을 때는 lifetime annotation을 추가해야한다.

// 러스트는 런타임에 실제로 사용되는 참조가 유효한지 확인하기 위해
// 제네릭 수명 매개변수를 이용해 관계를 설명해달라고 요구한다.

// 10.4.1 수명을 이용해 죽은 참조의 발생 방지하기
// 수명의 주요 목적은 죽은 참조가 발생하는 것을 방지하는 것이다.

// 아래 코드는 컴파일 에러 발생
/*
fn dead_reference() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
*/

// 10.4.2 대여 검사기
// 러스트 컴파일러는 대여한 값이 현재 범위 내에서 유효한지 검사하는
// 대여 검사기(borrow checker)를 탑재하고 있다.

// 10.4.3 함수의 제네릭 수명
// 10.4.4 수명 annotation 문법

// 수명 annotation은 참조의 유효 기간을 변경하지는 않는다.
// 함수에 제네릭 타입 매개변수를 사용하면 어떤 타입의 값이든 전달할 수 있는 것처럼
// 제네릭 수명 매개변수를 지정하면 어떤 수명의 참조도 전달할 수 있다.

// lifetime annotation은 조금 낯선 문법을 사용한다.
// 수명 매개변수의 이름은 반드시 작은 따옴표(')로 시작해야 하며,
// 제네릭 타입처럼 짧지만 소문자로 구성된 이름을 지정한다.
// 대부분 'a라는 이름을 사용한다.

// &i32 참조
// &'a i32 // 명시적 수명을 가진 참조
// &'a mut i32 // 명시적인 수명을 가진 가변 참조

// 10.4.5 함수 시그니처의 수명 애노테이션
pub fn select_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // x와 y중 어떤 값이 리턴될 지 모르므로,
    // 리턴 타입에 제네릭 수명 매개변수를 지정해야 한다!

    // 수명 'a로 표현한 구체적인 수명은 변수 x의 범위의 일부이면서
    // 변수 y의 범위와 겹쳐지는 부분을 표현하게 된다.
    // 다시 말해, 제네릭 수명 'a는 변수 x와 y 중,
    // 더 작은 범위를 갖는 변수의 수명을 따라간다.

    if x.len() > y.len() {
        return x;
    }
    return y;
}

// 10.4.6 수명의 관점에서 생각하기
// 수명 매개변수를 지정해야 하는 상황은 함수의 동작에 따라 다르다.
// 예를 들어, longest 함수가 길이가 긴 문자열 슬라이스가 아니라
// 항상 첫 번째 매개변수로 전달된 슬라이스를 리턴하도록 수정한다면
// 매개변수 y의 수명을 지정할 필요가 없어진다.

// 함수가 참조를 리턴할 때는 리턴 타입의 수명 매개변수는
// 매개변수 중 하나의 수명 매개변수와 일치해야 한다.

// 10.4.7 구조체 정의에서으 수명 어노태이션
// 구조체에 참조를 저장할 때는, 구조체 정의에 포함된 모든 참조에 수명 어노테이션을 추가해야 한다.

pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn lifetime_of_reference_in_struct() {
    let novel = String::from("스타워즈, 오래 전 멀고 먼 은하계에.");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("문장에서 마침표'.' 를 찾을 수 없습니다.");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 10.4.8 수명의 생략
// 지금까지 모든 참조는 수명이 있으며 함수나 구조체에서 참조를 사용하려면
// 수명 매개변수를 지정해야 한다는 것을 학습했다.
// 하지만 이전에 작성했던 함수는 수명 애노테이션이 없음에도 컴파일이 된다.

// 수명 생략 규칙에 따라 특정 상황에는 수명 애노테이션을 생략할 수 있다.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

// 1번 규칙: 각 참조 매개변수는 각각의 수명 매개변수가 있어야 한다.
// 2번 규칙: 명시적으로 하나의 입력 수명 매개변수가 있으면, 입력 수명을 모든 출력 수명에 적용한다.
// 3번 규칙: 입력 수명 매개변수가 하나 이상이며, 메서드로 선언되어서
//         매개변수 중 하나가 &self나 &mut self일 때는,
//         self 변수의 수명을 모든 출력 수명 매개변수에 적용한다.

// 1~3 번 규칙을 모두 적용하고도 리턴 타입의 수명을 판단할 수 없는 경우 컴파일 에러 발생

// 10.4.9 매서드 정의에서의 수명 애노테이션
// 구조체 메서드에 수명 매개변수를 선언하는 방법은
// 구조체의 필드나 매서드 매개변수와 리턴값에 따라 다르다.

// 구조체 필드의 수명 이름은 항상 impl 키워드 다음에 선언하며,
// 구조체 이름 다음에 명시해야 한다.
// 이 때 수명은 구조체의 타입 일부이기 때문이다.

// impl 블록 내의 메서드 시그너처에서는 참조가 구조체의 필드에 저장하는
// 참조의 수명과 관련이 있을 수도, 없을 수도 있다.

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        // 1번 생략 규칙이 적용됨
        return 3;
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // 3번 생략 규칙이 적용됨
        println!("주목해 주세요! {}", announcement);
        return self.part;
    }
}

// 10.4.10 정적 수명
// 정적 수명('static)은 특별한 수명으로 전체 프로그램에 적용된다.
// 모든 문자열 리터럴은 'static 수명이며, 다음처럼 직접 명시할 수도 있다.
// 문자열 리터럴은 프로그램의 바이너리에 직접 저장되며 항상 사용할 수 있다.
pub fn static_lifetime() {
    let s: &'static str = "문자열은 정적 수명이다.";
}

// 10.5 제네릭 타입 매개변수, 트레이드 경계, 그리고 수명
// 위 3가지를 하나으 함수에 모두 적용한 예를 간단히 살펴보자.
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("주목하세요: {}", ann);

    if x.len() > y.len() {
        return x;
    }
    return y;
}
