# 設計原則

## 一般的な設計原則の簡単な概要

---

## [SOLID](https://en.wikipedia.org/wiki/SOLID)

- [単一責任原則 (SRP)](https://en.wikipedia.org/wiki/Single-responsibility_principle):
  クラスは単一の責任のみを持つべきであり、つまり、ソフトウェア仕様の一部への変更のみがクラスの仕様に影響を与えることができるべきです。
- [開放/閉鎖原則 (OCP)](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle):
  「ソフトウェアエンティティは...拡張に対しては開かれ、修正に対しては閉じられているべきです。」
- [リスコフ置換原則 (LSP)](https://en.wikipedia.org/wiki/Liskov_substitution_principle):
  「プログラム内のオブジェクトは、そのプログラムの正当性を損なうことなく、そのサブタイプのインスタンスで置換可能であるべきです。」
- [インターフェース分離原則 (ISP)](https://en.wikipedia.org/wiki/Interface_segregation_principle):
  「多くのクライアント固有のインターフェースは、1つの汎用インターフェースよりも優れています。」
- [依存関係逆転原則 (DIP)](https://en.wikipedia.org/wiki/Dependency_inversion_principle):
  「抽象に依存し、[具象には]依存しない」べきです。

## [CRP（合成再利用原則）またはコンポジション・オーバー・インヘリタンス](https://en.wikipedia.org/wiki/Composition_over_inheritance)

「クラスは、基底クラスや親クラスからの継承よりも、コンポジション（望ましい機能を実装する他のクラスのインスタンスを含むこと）によって多態的動作とコードの再利用を優先すべきである原則」 - Knoernschild, Kirk (2002). Java Design - Objects, UML, and Process

## [DRY（Don't Repeat Yourself）](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)

「すべての知識片は、システム内で単一で明確で権威ある表現を持たなければならない」

## [KISS原則](https://en.wikipedia.org/wiki/KISS_principle)

ほとんどのシステムは複雑にするよりもシンプルに保たれた場合に最良に動作します。したがって、シンプルさは設計における重要な目標であるべきで、不必要な複雑さは避けるべきです。

## [デメテルの法則 (LoD)](https://en.wikipedia.org/wiki/Law_of_Demeter)

与えられたオブジェクトは、「情報隠蔽」の原則に従って、他の何か（そのサブコンポーネントを含む）の構造や属性について可能な限り少なく仮定すべきです。

## [契約による設計 (DbC)](https://en.wikipedia.org/wiki/Design_by_contract)

ソフトウェア設計者は、事前条件、事後条件、不変条件で抽象データ型の通常の定義を拡張する、ソフトウェアコンポーネントのための形式的で正確で検証可能なインターフェース仕様を定義すべきです。

## [カプセル化](https://en.wikipedia.org/wiki/Encapsulation_(computer_programming))

データとそのデータを操作するメソッドをバンドルすること、またはオブジェクトのコンポーネントの一部への直接アクセスを制限すること。カプセル化は、構造化データオブジェクトの値や状態をクラス内に隠し、権限のない当事者による直接アクセスを防ぐために使用されます。

## [コマンド・クエリ分離 (CQS)](https://en.wikipedia.org/wiki/Command%E2%80%93query_separation)

「関数は抽象的な副作用を生じるべきではない...コマンド（手続き）のみが副作用を生じることが許される。」 - Bertrand Meyer: Object-Oriented Software Construction

## [最小驚きの原則 (POLA)](https://en.wikipedia.org/wiki/Principle_of_least_astonishment)

システムのコンポーネントは、ほとんどのユーザーが期待する方法で動作すべきです。動作はユーザーを驚かせたり、仰天させたりすべきではありません。

## 言語モジュラー単位

「モジュールは、使用する言語の構文単位に対応しなければならない。」 - Bertrand Meyer: Object-Oriented Software Construction

## 自己文書化

「モジュールの設計者は、モジュールに関するすべての情報をモジュール自体の一部にするよう努めるべきである。」 - Bertrand Meyer: Object-Oriented Software Construction

## 統一アクセス

「モジュールが提供するすべてのサービスは、それらが記憶装置を通じて実装されているか計算を通じて実装されているかを明かさない統一された記法を通じて利用可能であるべきである。」 - Bertrand Meyer: Object-Oriented Software Construction

## 単一選択

「ソフトウェアシステムが一連の選択肢をサポートしなければならない場合は常に、システム内の唯一のモジュールがそれらの網羅的なリストを知っているべきである。」 - Bertrand Meyer: Object-Oriented Software Construction

## 永続化クロージャ

「記憶装置機構がオブジェクトを記憶する場合は常に、そのオブジェクトの依存物も一緒に記憶しなければならない。検索機構が以前に記憶されたオブジェクトを検索する場合は常に、まだ検索されていないそのオブジェクトの依存物も検索しなければならない。」 - Bertrand Meyer: Object-Oriented Software Construction