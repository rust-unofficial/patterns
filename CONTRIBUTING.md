# Contributing

## Discussion board

If you have a question or an idea regarding certain content but you want to have feedback of fellow community members 
and you think it may not be appropriate to file an issue open a discussion in our [discussion board](https://github.com/rust-unofficial/patterns/discussions).

## Writing a new article

Before writing a new article please check our [issues](https://github.com/rust-unofficial/patterns/issues) and 
the [Pull Requests](https://github.com/rust-unofficial/patterns/pulls) if there are existing issues or someone
is working on that topic.

If you don't find an issue regarding your topic and you are sure it is not more feasible to open a thread in the [discussion board](https://github.com/rust-unofficial/patterns/discussions)
please open a new issue, so we can discuss about the ideas and future content of the article together and maybe
give some feedback/input on it. 

When writing a new article it's recommended to copy the [pattern template](https://github.com/rust-unofficial/patterns/blob/master/template.md) into the
appropriate directory and start editing it. You may not want to fill out every section and remove it or you might want to add extra sections.

Consider writing your article in a way that has a low barrier of entry so also [Rustlings](https://github.com/rust-lang/rustlings) can follow
and understand the thought process behind it. So we can encourage people to use these patterns early on. 

We encourage you to write idiomatic Rust code that builds in the [playground](https://play.rust-lang.org/).

If you use links to blogposts or in general content that is not to be sure existing in a few years (e.g. pdfs) please take a snapshot
with the [Wayback Machine](https://web.archive.org/) and use the link to that snapshot in your article.

Don't forget to add your new article to the `SUMMARY.md` to let it be rendered to the book.

Please make `Draft Pull requests` early so we can follow your progress and can give early feedback (see the following section).

## Creating a Pull Request

"Release early and often!" also applies to pull requests!

Once your article has some visible work, create a `[WIP]` draft pull request and give it a description of what you did or want to do.
Early reviews of the community are not meant as an offense but to give feedback. 

A good principle: "Work together, share ideas, teach others."

### Test the book locally before submitting

Before submitting the PR launch the commands `mdbook build` to make sure that the book builds and `mdbook test` to make sure that
code examples are correct.

### Important Note

Please **don't force push** your branch to keep commit history and make it easier of us to see changes between reviews.

Make sure to `Allow edits of maintainers` (under the text box) in the PR so people can actually collaborate on things or fix smaller issues themselves.
