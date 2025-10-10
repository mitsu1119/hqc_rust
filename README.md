# hqc rust

## 概要
[HQC](https://pqc-hqc.org/doc/hqc_specifications_2025_08_22.pdf)のRust実装です。  

アルゴリズムの全体像を学ぶため、実験的に作成しました。  
プログラムの定時間化などサイドチャネル攻撃に対する緩和策が十分でないため、実際のアプリケーションには組み込まないでください。

## Test Vectors
HQC公式リポジトリにて配布されているKATs及び中間値（LICENSE: Public Domain）を使用しました。

**ファイル一覧**  
リポジトリURL：https://gitlab.com/pqc-hqc/hqc  
ファイル取得日：2025/09/24  
ファイル取得時のコミット：d622142a50f3ce6b6e1f5b15a5119d96c67194e0  
- kats/ref/hqc-1/PQCkemKAT_2321.rsp
- kats/ref/hqc-1/intermediates_values
- kats/ref/hqc-3/PQCkemKAT_4602.rsp
- kats/ref/hqc-3/intermediates_values
- kats/ref/hqc-5/PQCkemKAT_7333.rsp
- kats/ref/hqc-5/intermediates_values
