This document contains a non-exhaustive list of learning resources that may be helpful to new contributors. If you used a different resource to help contribute to this repository please PR it!

# Rust

If you are new to Rust, the learning curve can be a bit steep. This is largely due to the fact that Rust is not a garbage-collected language and has strict memory borrow checking rules baked into the compiler. At first, this might slow development velocity compared to less-strict GC languages, but this is what enables developers to write safe and performant low-level code with Rust.

## Rust Book

The [official Rust book](https://doc.rust-lang.org/book/) is a fantastic way to get started with Rust. It covers the most concepts of the language in a detailed and concise manner. If you want to learn hands-on, check out [Rustlings][#Rustlings] which is an interactive walkthrough of the book!

## Rustlings

[Rustlings](https://github.com/rust-lang/rustlings) is a set of interactive exercises and quizzes that can be used in any terminal. This is a great way to get hands on with Rust as a complete beginner.

## Comprehensive Rust

If you prefer a structured approach to learning a new language, Google's [Comprehensive Rust](https://google.github.io/comprehensive-rust/) is a Rust curriculum broken into individual days with morning and afternoon sessions. This guide is targeted towards Android development, but the Rust sections are general-purpose.

## Learning Rust With Entirely Too Many Linked Lists

One of the most challenging aspects of learning Rust, particularly for developers used to garbage-collected languages, is getting a handle on all the different types of pointers. Once you have tackled the main concepts in the official Rust Book, you might still need some hands-on practice to feel comfortable with all these ways to reference memory. [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) is an unofficial guide on using all of these pointer types by implementing various data structures that require their usage. The implementations in this repository likely won't have too many complex data structures, but it's a good way to get more comfortable with one of the more complex parts of the language.

## Little Book of Rust Books

The [Little Book of Rust Books](https://lborb.github.io/book/) is a collection of all the official and unofficial Rust books. There is A LOT covered across all of these books, but if you come across any in particular that were useful for contributing to this repo or simply learn rust please PR them here!

# UniFFI

## User Guide

The [UniFFI user guide](https://mozilla.github.io/uniffi-rs/latest/) is a good place to get started on how UniFFI works and how to develop with it.

## Matrix Channel

The [UniFFI matrix channel](https://matrix.to/#/#uniffi:mozilla.org) is a great place to get in touch with the maintainers and other developers using UniFFI.

## Docs

Of course, [the documentation](https://docs.rs/uniffi/latest/uniffi/) of the UniFFI codebase is also a great place to learn more about how to use UniFFI and how it works.

## Source Code

If you want to gain a deeper understanding of the internals of UniFFI, the source is available [on GitHub](https://github.com/mozilla/uniffi-rs/tree/main)

## ADRs

The [UniFFI ADRs](https://github.com/mozilla/uniffi-rs/tree/main/docs/adr) provide a lot of insight into why UniFFI was created and the various decisions that were made along the way of implementation
