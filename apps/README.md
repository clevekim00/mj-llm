# 플랫폼 앱

아직 실행 가능한 앱은 없습니다. N0 검증 후 필요한 플랫폼부터 구현합니다.

- desktop: 기존 웹 기술을 활용할 PC 셸 후보 검증
- android: Compose UI + native runtime bridge
- ios: SwiftUI + native runtime bridge

모바일 앱은 localhost HTTP 서버 없이 공통 Rust AppService를 호출하는 구조입니다.
