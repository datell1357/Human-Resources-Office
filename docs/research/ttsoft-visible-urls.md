# TTSOFT 시연 영상에 보이는 URL·도메인 목록

출처: [https://www.youtube.com/watch?v=PQgA-d87kro](https://www.youtube.com/watch?v=PQgA-d87kro) — 2020년 8월 21일 게시된 약 5분 25초 영상. 수집일: 2026-09-16.

중복을 합쳐 **128개 항목**을 기록했다. **판독한 주소·도메인 122개**, **문자 재확인이 필요한 후보 4개**, **끝이 잘린 URL 1개**, **기타 링크 표기 1개**다. 이는 화면에서 수집한 목록이며 프로그램이 지원하는 전체 사이트 수를 뜻하지 않는다.

전체 데이터: [CSV 다운로드](ttsoft-visible-urls.csv). CSV는 Excel에서 한글을 읽을 수 있도록 UTF-8 BOM으로 저장했다.

## 수집 범위와 판독 기준

- 영상 전체에서 1초 간격 325개 프레임으로 목록·본문·로그·브라우저 주소창·공지·인기글을 확인했다. 빠르게 목록을 스크롤하는 첫 5초는 원본의 30fps, 150개 프레임을 추가로 추출해 누락을 보완했다. OCR 후보를 원본 프레임과 대조했다.
- 첫 목록은 같은 도메인이 홍보·구인구직·꽁머니 분류에 반복된다. 후반 약 05:00의 정렬된 목록에서 추가 주소 3개도 확인했다. 화면에 나오지 않은 행은 지원 수 표기나 번호의 빈칸을 근거로 채우지 않았다.
- 대표 시점은 관찰 위치이며 최초 등장 시각을 뜻하지 않는다. 첫 5초의 밀리초 표기는 프레임 시각이다. 나머지는 1초 샘플의 대략적인 시점이다. YouTube 시점 링크는 정수 초 단위이므로 빠른 스크롤 구간은 일시정지 후 프레임 이동이 필요할 수 있다.
- 스킴과 www는 화면 표기를 유지했다. 같은 루트의 마지막 / 유무, 동일 IDN의 한글/Punycode 표기, 대소문자만 다른 도메인은 통합했다. 경로·쿼리가 다른 주소는 따로 기록했다.
- 스킴 없이 나온 도메인에는 http/https를 보충하지 않았다. 화면의 줄임표 뒤 문자열과 숨겨진 링크의 실제 href는 추정하지 않았다. 로그인 계정·비밀번호·텔레그램 사용자명은 URL 목록에 넣지 않았다.
- **확정적 완전성에는 한계가 있다.** 화면 밖·가려진 주소, 압축으로 구별되지 않는 문자, 1초 샘플 사이의 매우 짧은 노출은 누락될 수 있다. 아래 재확인 항목은 주소가 확정된 것으로 사용하면 안 된다.
- 이 목록은 2020년 영상의 표시 내용이다. 수집한 커뮤니티·광고 도메인의 현재 접속 가능 여부와 운영 주체는 확인하지 않았다. YouTube 첫 번째 본문 링크만 문자 판독 보조를 위해 공개 메타데이터와 대조했다.
- 원본 영상·계정 정보가 포함될 수 있는 프레임·OCR 원문은 저장소에 추가하지 않았다. CSV의 evidence는 로컬 분석용 프레임 식별자이며 저장소 파일 링크가 아니다.

## 집계

| 구분 | 항목 수 |
|---|---:|
| 프로그램 사이트 목록 | 113 (이 중 3개는 문자 재확인 필요) |
| 공급자·본문의 완전한 URL 또는 후보 | 3 (이 중 1개는 문자 재확인 필요) |
| 스킴 없는 추가 도메인 | 5 |
| 개별 게시글·이동 페이지 | 5 |
| 끝이 잘린 URL | 1 |
| 기타 링크 표기 | 1 |
| 합계 | 128 |

## 전체 목록

### 프로그램 사이트 목록

| ID | URL 또는 화면 표기 | 대표 시점 | 판독 상태·비고 |
|---|---|---|---|
| U001 | `https://www.dajaba.org` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 0. |
| U002 | `https://www.totomarket.org` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 1. |
| U003 | `http://www.toxabu.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 4. |
| U004 | `http://daumd03.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 5. |
| U005 | `https://www.xn--hs0by0egti.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 6. 한글 도메인: www.꽁머니.com. |
| U006 | `https://www.muksol.org` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 7. |
| U007 | `https://chongpancity.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 8. |
| U008 | `https://mtfuck.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 9. |
| U009 | `https://moca-2083.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 11. |
| U010 | `https://www.betde.org` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 12. |
| U011 | `https://www.mukjob.org` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 13. |
| U012 | `https://alspo1.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 14. |
| U013 | `http://www.pepsi24.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 15. |
| U014 | `https://progambles.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 16. |
| U015 | `https://bettingnori.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 17. |
| U016 | `https://pick-star.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 18. |
| U017 | `https://toto-day.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 19. |
| U018 | `https://www.xn--p22b075bhybr0a.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 20. 한글 도메인: www.먹튀헬프.com. |
| U019 | `https://totoilbo.club` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 21. |
| U020 | `https://toragi.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 22. |
| U021 | `https://www.totohot.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 23. |
| U022 | `http://tobigtv.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 24. |
| U023 | `https://k-onplay2.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 25. |
| U024 | `http://suresc.org` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 26. |
| U025 | `https://bmania.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 28. |
| U026 | `https://toto-ok.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 29. |
| U027 | `https://topang2.net` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 30. |
| U028 | `https://noliteo.com` | [00:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=0s) | 화면 목록 No. 32. |
| U029 | `http://mt-of.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 33. |
| U030 | `https://www.muktstore.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 34. https://www.muktstore.com/ 표기도 관찰되어 같은 항목으로 통합. |
| U031 | `https://mtfriend.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 35. |
| U032 | `https://cpkorean.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 36. |
| U033 | `http://www.opland2.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 37. |
| U034 | `http://mtgallery.net` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 38. |
| U035 | `https://mt-kingdom.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 39. |
| U036 | `http://totocool01.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 40. |
| U037 | `https://juso58.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 41. |
| U038 | `http://toto365.site` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 42. |
| U039 | `https://xn--365-233mv64a.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 43. 한글 도메인: 도박365.com. |
| U040 | `https://totonam1.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 44. |
| U041 | `http://xn--o79ao42c.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 45. 한글 도메인: 게인.com. 페이지 공지의 게인.com 및 주소창의 한글 표기도 같은 도메인. |
| U042 | `https://game7hub.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 48. |
| U043 | `https://sup-1.xyz` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 49. |
| U044 | `https://www.toto-web1.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 51. |
| U045 | `http://kao-1.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 53. |
| U046 | `https://mt-show.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 54. |
| U047 | `http://xn--vg1b002a0jla.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 55. 한글 도메인: 토토뉴스.com. |
| U048 | `https://ranktong6.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 56. |
| U049 | `https://tt-wiki.com/` | [00:02.167](https://www.youtube.com/watch?v=PQgA-d87kro&t=2s) | 화면 목록 No. 58. |
| U050 | `http://tozaral.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 59. |
| U051 | `https://redcatoon369.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 60. |
| U052 | `https://bt-note.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 61. |
| U053 | `https://gambletour.com` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 62. |
| U054 | `http://tototv.xyz` | [00:04.667](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 63. |
| U055 | `http://linkbom.net` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 64. |
| U056 | `http://mtb007.com` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 65. |
| U057 | `https://ttattack.com` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 66. |
| U058 | `http://www.biz-shop.net` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 67. |
| U059 | `https://totowg.com` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 68. |
| U060 | `https://mail.toto1588.com` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 69. |
| U061 | `http://totoclickcasino.com` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | **문자 재확인 필요**. 화면 목록 No. 70. 유력 판독값. 영상의 cl이 d처럼 붙어 보여 totodickcasino.com과 글자 구분을 확정하지 못함. 후보를 별도 주소로 중복 집계하지 않음. |
| U062 | `https://mtsimsa.com` | [00:04.500](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 71. |
| U063 | `https://xn--v06bn6fa78l.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 72. 한글 도메인: 토토총판.com. |
| U064 | `http://www.ttbk3.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 73. |
| U065 | `http://ggongmoa.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 74. |
| U066 | `http://xn--hs0bs66bnzh.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 75. 한글 도메인: 꽁투유.com. |
| U067 | `https://totositebet.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 76. |
| U068 | `https://woorislot1.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 78. |
| U069 | `https://mt-daily.net` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 79. |
| U070 | `http://xn--365-9j6nq6jn0s.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 80. 한글 도메인: 스코어365.com. |
| U071 | `http://toto48.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 81. |
| U072 | `https://mchart.kr` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 82. |
| U073 | `https://xn--910bs4ktoucof.net` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 83. 한글 도메인: 총판나라.net. |
| U074 | `https://toto-tp.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 84. |
| U075 | `https://thankyouboy1.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 86. |
| U076 | `https://togt-2.com` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 87. |
| U077 | `https://www.ok-fam.org/` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 88. |
| U078 | `http://allscore.co.kr` | [00:04.300](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 89. |
| U079 | `https://xn--1-277es35b7vg8jg4lc.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | **문자 재확인 필요**. 화면 목록 No. 90. 유력 판독값. 끝의 4lc에서 l이 매우 얇음. 이 후보의 IDNA 해석은 카지노스쿨1이며 화면 이름과 맞지만 4c 판독 가능성을 영상만으로 완전히 배제하지 못함. |
| U080 | `https://totozz.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 91. |
| U081 | `https://tobia.kr` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 94. |
| U082 | `http://spomama1.net` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 95. |
| U083 | `https://www.todosa.net` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 96. |
| U084 | `https://yasilhouses.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 97. |
| U085 | `http://homan22.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 98. |
| U086 | `https://to-gura.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 99. |
| U087 | `http://www.mt-talk01.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 100. |
| U088 | `https://mt-eosa.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 101. |
| U089 | `https://www.tobosal.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 102. |
| U090 | `http://jobtoto1.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 103. |
| U091 | `https://ggongeya.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 104. |
| U092 | `http://dva11.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 105. |
| U093 | `https://mt-zero1.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 106. |
| U094 | `https://xn--9t4b17e84la478b.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 107. 한글 도메인: 토토하우스.com. |
| U095 | `https://mcut.co.kr` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 108. |
| U096 | `https://dobakkkun.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 109. |
| U097 | `https://mt-report.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 111. |
| U098 | `https://www.totobook1.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 113. |
| U099 | `https://xn--vk1b187auji.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | **문자 재확인 필요**. 화면 목록 No. 114. 유력 판독값. 끝의 i가 흐림. 이 후보의 IDNA 해석은 토와대이며 화면 이름과 맞음. i를 빼면 유효한 Punycode로 해독되지 않지만 영상의 원문 오기 가능성도 남김. |
| U100 | `https://yaya25.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 116. |
| U101 | `https://bouncetoto.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 120. |
| U102 | `https://totocider.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 121. |
| U103 | `https://xn--365-n07mq32b.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 122. 한글 도메인: 슬롯365.com. |
| U104 | `http://safe-to1.com` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 126. |
| U105 | `http://totogram.net` | [00:04.200](https://www.youtube.com/watch?v=PQgA-d87kro&t=4s) | 화면 목록 No. 170. |
| U106 | `https://onbaduki.com` | [00:03.833](https://www.youtube.com/watch?v=PQgA-d87kro&t=3s) | 화면 목록 No. 171. |
| U107 | `https://toto-town.com` | [00:03.833](https://www.youtube.com/watch?v=PQgA-d87kro&t=3s) | 화면 목록 No. 177. |
| U108 | `http://mt1366.com` | [00:03.167](https://www.youtube.com/watch?v=PQgA-d87kro&t=3s) | 화면 목록 No. 209. |
| U109 | `https://toto-naver.com` | [00:03.167](https://www.youtube.com/watch?v=PQgA-d87kro&t=3s) | 화면 목록 No. 228. |
| U110 | `https://www.scoretvs.com` | [00:02.167](https://www.youtube.com/watch?v=PQgA-d87kro&t=2s) | 화면 목록 No. 416. |
| U111 | `https://mtmoney.net` | [05:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=300s) | 화면 목록 No. 3(정렬 후). |
| U112 | `http://www.mt-hd.org` | [05:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=300s) | 화면 목록 No. 4(정렬 후). |
| U113 | `https://mt-city.com` | [05:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=300s) | 화면 목록 No. 28(정렬 후). |

### 공급자·본문

| ID | URL 또는 화면 표기 | 대표 시점 | 판독 상태·비고 |
|---|---|---|---|
| U114 | `http://ttsoft.kr` | [02:04](https://www.youtube.com/watch?v=PQgA-d87kro&t=124s) | 구매 및 다운로드 안내. 후반의 TTSOFT.KR 표기도 같은 도메인으로 통합. |
| U115 | `https://youtu.be/ObFTmXYir0w` | [00:44](https://www.youtube.com/watch?v=PQgA-d87kro&t=44s) | 본문에 세 번 반복. YouTube 공개 oEmbed 응답에서도 TT SOFT (TTSOFT)의 6월 30일 시연 영상으로 확인. 첫 글자는 대문자 O. |
| U116 | `https://youtu.be/-7l2hPApjP8` | [02:05](https://www.youtube.com/watch?v=PQgA-d87kro&t=125s) | **문자 재확인 필요**. 유력 판독값. -7 다음 문자가 소문자 l, 대문자 I, 숫자 1 중 무엇인지 확정하지 못함. 세 후보 모두 YouTube oEmbed에서 404를 반환했으므로 이를 판독 근거로 삼지 않음. |

### 스킴 없는 도메인

| ID | URL 또는 화면 표기 | 대표 시점 | 판독 상태·비고 |
|---|---|---|---|
| U117 | `po-up20.com` | [00:37](https://www.youtube.com/watch?v=PQgA-d87kro&t=37s) | 글 작성 화면 입력란. http 또는 https를 임의로 추가하지 않음. |
| U118 | `vip-gain.com` | [03:57](https://www.youtube.com/watch?v=PQgA-d87kro&t=237s) | 게인 페이지 공지의 주소 안내. http 또는 https는 화면에 없음. |
| U119 | `1-conn.com` | [03:57](https://www.youtube.com/watch?v=PQgA-d87kro&t=237s) | 인기글 1번 제목에 표시. http 또는 https는 화면에 없음. |
| U120 | `pk1004.com` | [03:57](https://www.youtube.com/watch?v=PQgA-d87kro&t=237s) | 인기글 3번 제목에 표시. 앞의 목록 번호 3은 도메인에 포함되지 않음. http 또는 https는 화면에 없음. |
| U121 | `mvp-t2.com` | [03:57](https://www.youtube.com/watch?v=PQgA-d87kro&t=237s) | 인기글 4번 제목에 표시. http 또는 https는 화면에 없음. |

### 개별 페이지

| ID | URL 또는 화면 표기 | 대표 시점 | 판독 상태·비고 |
|---|---|---|---|
| U122 | `http://toto365.site/bbs/board.php?bo_table=g02&wr_id=1019593` | [01:58](https://www.youtube.com/watch?v=PQgA-d87kro&t=118s) | 로그 및 브라우저 주소창. |
| U123 | `https://xn--365-233mv64a.com/bbs/board.php?bo_table=gallery&wr_id=103951` | [02:36](https://www.youtube.com/watch?v=PQgA-d87kro&t=156s) | 붙여넣은 주소. 주소창의 https://도박365.com/bbs/board.php?bo_table=gallery&wr_id=103951 표기와 통합. |
| U124 | `https://totonam1.com/bbs/board.php?bo_table=free3&wr_id=554262` | [03:12](https://www.youtube.com/watch?v=PQgA-d87kro&t=192s) | 붙여넣은 주소 및 주소창. |
| U125 | `http://xn--o79ao42c.com/bbs/board.php?bo_table=1&wr_id=666120` | [03:53](https://www.youtube.com/watch?v=PQgA-d87kro&t=233s) | 붙여넣은 주소. 주소창의 게인.com/bbs/board.php?bo_table=1&wr_id=666120 표기와 통합. |
| U126 | `게인.com/bbs/link.php?bo_table=1&wr_id=666120&no=1` | [04:00](https://www.youtube.com/watch?v=PQgA-d87kro&t=240s) | 주소창에서 스킴 없이 표시. http/https는 보충하지 않음. |

### 잘린 URL

| ID | URL 또는 화면 표기 | 대표 시점 | 판독 상태·비고 |
|---|---|---|---|
| U127 | `https://도박365.com/bbs/login.php?wr_id=103951&url=https%3A%2F%2Fxn--365-233mv64a.com%2Fbbs%2Fboar...` | [02:39](https://www.youtube.com/watch?v=PQgA-d87kro&t=159s) | **끝부분 잘림**. 화면에 보이는 부분까지만 전사. 줄임표 이후의 쿼리나 경로를 추정하지 않음. |

### 기타 링크 표기

| ID | URL 또는 화면 표기 | 대표 시점 | 판독 상태·비고 |
|---|---|---|---|
| U128 | `http://파워업` | [03:57](https://www.youtube.com/watch?v=PQgA-d87kro&t=237s) | **화면 표기**. 게시글 첨부 링크에 보이는 원문. 공개 도메인인지, 잘못 입력한 링크인지 확인되지 않아 일반 사이트 주소로 집계하지 않음. |

## 재확인이 필요한 문자

| ID | 유력 판독값 | 남은 모호함 |
|---|---|---|
| U061 | `http://totoclickcasino.com` | `cl`이 붙어 `d`처럼 보일 수 있음 |
| U079 | `https://xn--1-277es35b7vg8jg4lc.com` | 끝의 `4lc`에서 `l` 식별 |
| U099 | `https://xn--vk1b187auji.com` | 끝의 `i` 식별 |
| U116 | `https://youtu.be/-7l2hPApjP8` | `-7` 다음의 `l` / `I` / `1` 식별 |

위 표의 Punycode 후보는 각각 화면 이름과 IDNA 해석을 대조한 결과를 비고에 적었다. 이 대조는 영상의 흐린 글자를 확정하는 증거와는 구분한다.
