# コントリビューション

## はじめに

この書籍は、Rustプログラミングのテクニック、（アンチ）パターン、イディオム、
その他の説明のカタログです。これは、共同作業を通じて生まれた集合的な（時には暗黙の）
知識と経験の編纂です。

ここで説明されているパターンは**ルールではありません**が、Rustで慣用的なコードを書くための
ガイドラインとして受け取ってください。私たちはこの書籍でRustのパターンを収集しており、
人々がRustのイディオムのトレードオフを学び、自分のコードで適切に使用できるようにしています。

この取り組みの一部になりたい場合は、以下の方法で参加できます：

## ディスカッションボード

特定のコンテンツに関する質問やアイデアがあるが、仲間のコミュニティメンバーからの
フィードバックが欲しい場合、そしてイシューを提出するのが適切でないと思われる場合は、
[ディスカッションボード](https://github.com/rust-unofficial/patterns/discussions)で
ディスカッションを開いてください。

## 新しい記事の執筆

新しい記事を書く前に、以下のリソースのいずれかで既存のディスカッションがあるか、
誰かがすでにそのトピックに取り組んでいるかを確認してください：

- [Umbrella issue](https://github.com/rust-unofficial/patterns/issues/116)
- [すべてのイシュー](https://github.com/rust-unofficial/patterns/issues)
- [プルリクエスト](https://github.com/rust-unofficial/patterns/pulls)

あなたのトピックに関するイシューが見つからず、
[ディスカッションボード](https://github.com/rust-unofficial/patterns/discussions)で
スレッドを開く方が適切でないと確信している場合は、新しいイシューを開いてください。
そうすれば、記事のアイデアと将来のコンテンツについて一緒に議論し、
フィードバック/インプットを提供できます。

新しい記事を書くときは、
[パターンテンプレート](https://github.com/rust-unofficial/patterns/blob/master/template.md)を
適切なディレクトリにコピーして編集を開始することをお勧めします。すべてのセクションを
埋める必要はなく削除してもよいですし、追加のセクションを加えることもできます。

[Rustlings](https://github.com/rust-lang/rustlings)も理解できるような、
参入障壁の低い方法で記事を書くことを検討してください。
そうすることで、人々に早い段階でこれらのパターンを使用することを促すことができます。

[playground](https://play.rust-lang.org/)でビルドできる慣用的なRustコードを
書くことをお勧めします。

ブログ投稿へのリンクや、数年後に存在していることが確実でないコンテンツ（PDFなど）への
リンクを使用する場合は、[Wayback Machine](https://web.archive.org/)でスナップショットを取り、
記事でそのスナップショットへのリンクを使用してください。

新しい記事を書籍にレンダリングするために、`SUMMARY.md`に追加することを忘れないでください。

進捗を追跡し、早期のフィードバックを提供できるように、早めに`Draft Pull requests`を
作成してください（次のセクションを参照）。

## スタイルガイド

書籍全体で一貫したスタイルを保つために、以下を提案します：

- 公式Rustブックの[スタイルガイド](https://github.com/rust-lang/book/blob/master/style-guide.md)に従う
- [RFC 1574](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text)に従う
  要約：
  - 完全な型名を優先する。例：`Option`ではなく`Option<T>`
  - 適用可能な場合は、ブロックコメント（`/* */`）よりも行コメント（`//`）を優先する

## 記事をローカルで確認する

PRを提出する前に、`mdbook build`コマンドを実行して書籍がビルドされることを確認し、
`mdbook test`を実行してコード例が正しいことを確認してください。

### Markdown lint

ファイルがMarkdownスタイルに準拠していることを確認するために、
[markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli)を使用しています。
CIテストを通過するための手作業を省くために、以下のコマンドを使用して、
Markdownファイルを書く際に発生する問題のほとんどを自動的に修正できます。

- インストール：

  ```sh
  npm install -g markdownlint-cli
  ```

- すべてのmarkdownファイルをチェック：
  - unix: `markdownlint '**/*.md'`
  - windows: `markdownlint **/*.md`

- 基本的なエラーを自動修正：
  - unix: `markdownlint -f '**/*.md'`
  - windows: `markdownlint -f **/*.md`

## プルリクエストの作成

「早くリリースし、頻繁にリリースする！」はプルリクエストにも適用されます！

記事に何か見える作業ができたら、`[WIP]`ドラフトプルリクエストを作成し、
何をしたか、または何をしたいかの説明を記載してください。コミュニティの早期レビューは、
攻撃的な意図ではなく、フィードバックを提供するためのものです。

良い原則：「一緒に働き、アイデアを共有し、他の人に教える」

### 重要な注意事項

コミット履歴を保持し、レビュー間の変更を確認しやすくするために、
ブランチのコミットを**強制プッシュしないでください**。

人々が実際に協力したり、小さな問題を自分で修正したりできるように、
PRで`Allow edits of maintainers`（テキストボックスの下）を有効にしてください。

## メンテナー

このリポジトリは以下の人々によってメンテナンスされています：

- [simonsan](https://github.com/simonsan)
- [marcoieni](https://github.com/marcoieni) - MarcoはブックのCIのみを担当しています。
  ブックの内容をレビューせず、新しいイシューやプルリクエストの通知を受け取りません。
  CIの問題について見てもらいたい場合は、直接pingしてください。