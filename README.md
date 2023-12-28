# HTMX-Axum Hello World App

This is a simple hello world app that is intended to be forked into new projects.

## Recommendations

### Templating

While I have opted to use Askama for templating, there are several good options for templating:

- [Minijinja](https://crates.io/crates/minijinja): MiniJinja is a powerful but minimal dependency template engine for Rust compatible with Jinja/Jinja2 
- [Askama](https://crates.io/crates/askama): Jinja inspired, type-safe, requires template precompilation. Has significant divergence from Jinja syntax in parts.
- [Tera](https://crates.io/crates/tera): Jinja inspired, dynamic, has divergences from Jinja.
- [TinyTemplate](https://crates.io/crates/tinytemplate): minimal footprint template engine with syntax that takes lose inspiration from Jinja and handlebars.
- [Liquid](https://crates.io/crates/liquid): an implementation of Liquid templates for Rust. Liquid was inspired by Django from which Jinja took it's inspiration.

If you're planning on using HTMX extensively, I recommend looking into [axum-htmx](https://crates.io/crates/axum-htmx). As you can see from this example project, it is possible to use HTMX without the `axum-htmx` crate.

### CLI

- [Cargo Watch](https://github.com/watchexec/cargo-watch) is a must so that you can see your changes immediately

### CSS

- [Tailwind](https://tailwindcss.com/docs/installation). You can check out my [portfolio-blog](https://github.com/tieje/portfolio-htmx-axum) to see how to set it up.

## Usage

1. `cargo run`
2. Go to [http://localhost:3000](http://localhost:3000)

Bonus: Try out [http://localhost:3000/Kevin](http://localhost:3000/Kevin)

## Motivation

I created this simple app as part of [my blog](https://thomasjamesfrancis.com/entries/building_my_blog_with_htmx_and_axum). You can learn more about my experience building with HTMX and Axum there.
