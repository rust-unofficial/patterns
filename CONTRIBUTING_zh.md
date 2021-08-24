# 贡献

## 简介

[习语]: https://en.wikipedia.org/wiki/Programming_idiom

这本书是Rust编程技术，(反)模式，[习语] 以及其他表示的目录。

这本书介绍的模式**并不是规则**，但可以被视为编写地道的Rust代码的指引。
我们会整理这本书中 Rust 模式，以便学习者掌握Rust习语的权衡，并且在他们的代码中正确地使用。

如果你想要加入我们，这里有一些方式。

## 讨论区

[讨论区]: https://github.com/rust-unofficial/patterns/discussions

如果你有关于某些内容的问题或想法，而你想要得到社区的反馈，
并且认为不适合提出issue，你可以在我们的[讨论区]提出讨论。

## 撰写新文章

Before writing a new article please check in one of the following resources if
there is an existing discussion or if someone is already working on that topic:

- [Umbrella issue](https://github.com/rust-unofficial/patterns/issues/116),
- [All issues](https://github.com/rust-unofficial/patterns/issues),
- [Pull Requests](https://github.com/rust-unofficial/patterns/pulls)

If you don't find an issue regarding your topic, and you are sure it is not more
feasible to open a thread in the [discussion board](https://github.com/rust-unofficial/patterns/discussions)
please open a new issue, so we can discuss the ideas and future content
of the article together and maybe give some feedback/input on it.

When writing a new article it's recommended to copy the [pattern template](https://github.com/rust-unofficial/patterns/blob/master/template.md)
into the appropriate directory and start editing it. You may not want to fill
out every section and remove it, or you might want to add extra sections.

Consider writing your article in a way that has a low barrier of entry so also
[Rustlings](https://github.com/rust-lang/rustlings) can follow and understand
the thought process behind it. So we can encourage people to use these patterns
early on.

We encourage you to write idiomatic Rust code that builds in the [playground](https://play.rust-lang.org/).

If you use links to blogposts or in general content that is not to be sure
existing in a few years (e.g. pdfs) please take a snapshot with the
[Wayback Machine](https://web.archive.org/) and use the link to that snapshot
in your article.

Don't forget to add your new article to the `SUMMARY.md` to let it be rendered
to the book.

Please make `Draft Pull requests` early, so we can follow your progress and can
give early feedback (see the following section).

## Style guide

In order to have a consistent style across the book, we suggest to:

- Follow the official Rust book's [style guide](https://github.com/rust-lang/book/blob/master/style-guide.md).
- Follow [RFC 1574](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text).
  Tl;dr:
  - Prefer full types name. For example `Option<T>` instead of `Option`.
  - Prefer line comments (`//`) over block comments (`/* */`) where applicable.

## Check the article locally

Before submitting the PR launch the commands `mdbook build` to make sure that
the book builds and `mdbook test` to make sure that code examples are correct.

### Markdown lint

To make sure the files comply with our Markdown style we use [markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli).
To spare you some manual work to get through the CI test you can use the
following commands to automatically fix most of the emerging problems when
writing Markdown files.

- Install:

  ```sh
  npm install -g markdownlint-cli
  ```

- Check all markdown files:
  - unix: `markdownlint '**/*.md'`
  - windows: `markdownlint **/*.md`

- Automatically fix basic errors:
  - unix: `markdownlint -f '**/*.md'`
  - windows: `markdownlint -f **/*.md`

## Creating a Pull Request

"Release early and often!" also applies to pull requests!

Once your article has some visible work, create a `[WIP]` draft pull request
and give it a description of what you did or want to do. Early reviews of the
community are not meant as an offense but to give feedback.

A good principle: "Work together, share ideas, teach others."

### Important Note

Please **don't force push** commits in your branch, in order to keep commit
history and make it easier for us to see changes between reviews.

Make sure to `Allow edits of maintainers` (under the text box) in the PR so
people can actually collaborate on things or fix smaller issues themselves.
