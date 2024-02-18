# Rust Askama Htmx Tailwind Todo Application

A Rust (Axum) server, powered by askama templates, serving HTMX pages, and styled with Tailwind CSS todo application.

## About

Boilerplate code to demonstrate how an Axum API would incorporate both HTMX and JSON API endpoints in an efficient way.

## Getting Started

### Prerequisites
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Make](https://www.gnu.org/software/make/)
3. [Docker](https://www.docker.com/get-started/)
4. [PNPM](https://pnpm.io/installation)
   
### Configuring development environment
To get a development build ready, run:
```bash
make setup
```

### Running locally
```bash
make tailwind-watch # watches templates for any tailwind changes and rebuilds /assets/main.css.
make server-watch   # serves the backend and watches for any new changes, which will trigger a "hot" reload.
```
