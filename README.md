# Rustデザインパターン

Rustプログラミング言語におけるデザインパターンとイディオムについてのオープンソースな書籍です。
[こちら](https://rust-unofficial.github.io/patterns/)でお読みいただけます。

また、PDF形式で[このリンク](https://rust-unofficial.github.io/patterns/rust-design-patterns.pdf)から
書籍をダウンロードすることもできます。

## コントリビューション

このリポジトリに他の人に役立つコンテンツが不足していて、それを説明したいと思っていますか？
素晴らしい！私たちは常にこのプロジェクトへの新しい貢献（特定のトピックに関する詳細な説明や修正など）を歓迎しています。

追加可能なすべてのパターン、アンチパターン、イディオムについては、
[Umbrella issue](https://github.com/rust-unofficial/patterns/issues/116)をご確認ください。

このリポジトリへの貢献方法について詳しくは、[コントリビューションガイド](./CONTRIBUTING.md)をお読みください。

## mdbookでのビルド

この書籍は[mdbook](https://rust-lang.github.io/mdBook/)でビルドされています。
`cargo install mdbook`を実行してインストールできます。

### 追加の依存関係

- `cargo install mdbook-last-changed` - フッターの日付変更用

- `cargo install mdbook-pandoc` - 書籍のPDFレンダリング用

- `cargo install mdbook-i18n-helpers` - 翻訳とi18nサポート用

#### Texlive

```sh
# .envファイルをソースしてPANDOC_VERSIONを取得
. ./.env

sudo apt-get update

sudo apt-get install -y texlive texlive-latex-extra texlive-luatex texlive-lang-cjk librsvg2-bin fonts-noto

curl -LsSf https://github.com/jgm/pandoc/releases/download/$PANDOC_VERSION/pandoc-$PANDOC_VERSION-linux-amd64.tar.gz | tar zxf -
```

### 書籍のビルド

ローカルでビルドしたい場合は、リポジトリのルートディレクトリで以下の2つのコマンドのいずれかを実行できます：

- `mdbook build`

  静的なHTMLページを出力として生成し、デフォルトで`/book`ディレクトリに配置します。

- `mdbook serve`

  書籍を`http://localhost:3000`で提供し（ポートは変更可能、確認のためターミナル出力をご覧ください）、
  変更が発生するとブラウザを再読み込みします。

## ライセンス

このリポジトリの内容は**MPL-2.0**でライセンスされています。
[LICENSE](./LICENSE)をご覧ください。
