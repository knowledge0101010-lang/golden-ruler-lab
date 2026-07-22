// 이도(세종)→Cangjie 자동변환 (geumcheok translate --host cangjie)
// 이도 (세종) · 2 degrees — .ido 예제 · 기본 호스트=cangjie
// 키워드: 한국어 | 산술 기호는 C 유지

func double(값: Int64): Int64 {
    return 값 * 2
}

main() {
    let name = "이도"
    var score = 21
    score = double(score)
    println("lang=${name}")
    println("score=${score}")
}
