# 플랫폼 앱

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

아직 실행 가능한 앱은 없습니다. N0 검증 후 필요한 플랫폼부터 구현합니다.

- desktop: Windows·macOS·Linux, 기존 웹 기술을 활용할 PC 셸 후보 검증
- android: 휴대폰·태블릿, Compose UI + native runtime bridge
- ios: iPhone·iPad, SwiftUI + native runtime bridge

모바일 앱은 localhost HTTP 서버 없이 공통 Rust AppService를 호출하는 구조입니다.
