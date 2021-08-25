# Rust Design Patterns

An open source book about design patterns and idioms in the Rust programming
language that you can read [here](https://rust-unofficial.github.io/patterns/).

## Contributing

You are missing content in this repository that can be helpful for others, and
you are eager to explain it? Awesome! We are always happy about new contributions
(e.g. elaboration or corrections on certain topics) to this project.

You can check the [Umbrella issue](https://github.com/rust-unofficial/patterns/issues/116)
for all the patterns, anti-patterns, and idioms that could be added.

We suggest reading our [Contribution guide](./CONTRIBUTING.md) to get more information
on how contributing to this repository works.

## Building with mdbook

This book is built with [mdbook](https://rust-lang.github.io/mdBook/). You can
install it by running `cargo install mdbook`.

If you want to build it locally you can run one of these two commands in the root
directory of the repository:

- `mdbook build`

  Builds static html pages as output and place them in the `/book` directory by
  default.

- `mdbook serve`

  Serves the book at `http://localhost:3000` (port is changeable, take a look at
  the terminal output to be sure) and reloads the browser when a change occurs.

## License

The content of this repository is licensed under **MPL-2.0**; see [LICENSE](./LICENSE).
