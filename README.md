# Human Resources Office

프로그램 조사와 개발 초안을 위한 저장소입니다. 현재 `accounts.txt`를 읽어 접속 대상을 판정하는 Tauri 2 데스크톱 초안과 참고 영상 분석 문서가 있습니다.

## 프로그램 초안 실행

Node.js, npm, Rust가 준비된 환경에서 다음 명령으로 실행합니다.

```bash
npm install
npm run dev
```

저장소 루트의 `accounts.txt`를 다음 형식으로 작성합니다.

```text
# active | reference_url | username | password | note
true | https://example.com | user01 | change-me | 정상 계정
false | https://test.local | user02 | change-me | 수동 제외
```

- `active`: `true/false`, `yes/no`, `1/0`, `active/inactive`, `on/off`를 지원합니다.
- `reference_url`: `http://` 또는 `https://`로 시작해야 합니다.
- `username`, `password`: 활성 대상 판단에 필요한 필수값입니다.
- `note`: 선택값이며 `|`가 더 있으면 나머지 문자열을 메모로 합칩니다.
- 빈 줄과 `#`로 시작하는 줄은 무시합니다. 같은 URL과 계정이 활성 상태로 반복되면 뒤의 행을 오류로 분류합니다.

실제 `accounts.txt`는 Git에서 제외됩니다. 시작용 [`accounts.example.txt`](accounts.example.txt)를 복사해 사용할 수 있습니다. Rust 백엔드에서 파일을 읽고 접속 대상을 판정하며, React 화면에는 마스킹된 계정만 전달합니다. 비밀번호는 웹뷰 데이터에 포함하지 않습니다. 현재 초안은 파일 판독과 대상 선별까지만 수행하며, 외부 사이트 로그인·가입·게시 기능은 실행하지 않습니다.

## 조사 문서

- [TTSOFT 프로그램 분석](docs/research/ttsoft-program-analysis.md): 기능, 사용 흐름, 추정 구조, 개선점과 적용 범위
- [영상 관찰 기록](docs/research/ttsoft-video-evidence.md): 15개 근거 항목, 시점 링크, 확인 범위와 한계
- [영상에 보이는 URL 전체 수집 목록](docs/research/ttsoft-visible-urls.md): 중복을 합친 128개 항목, 대표 시점과 판독 상태 ([CSV](docs/research/ttsoft-visible-urls.csv))
- [개발에 필요한 자료와 구성](docs/planning/program-requirements.md): 준비 자료, 필수 기능, 기술 후보, 운영 환경과 검증 조건

분석 대상은 [2020년 TTSOFT 시연 영상](https://www.youtube.com/watch?v=PQgA-d87kro)입니다. 영상에서 보이는 기능, 공급자의 설명, 분석자의 추론을 구분했습니다. 영상 속 사이트의 계정 정보나 비밀번호는 보관하지 않습니다.
