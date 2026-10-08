# Platform applications

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

There are no runnable user applications yet. Implement the needed platforms after N0 validation.

- desktop: Windows/macOS/Linux; evaluate a PC shell reusing existing web technologies
- android: phones/tablets; Compose UI + native runtime bridge
- ios: iPhone/iPad; SwiftUI + native runtime bridge

Mobile apps are designed to call shared Rust AppService without a localhost HTTP server.
