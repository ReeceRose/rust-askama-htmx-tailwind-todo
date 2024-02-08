.PHONY: format setup tailwind-watch server-watch dev

format:
	pnpm run format

setup:
	pnpm install
	curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	cargo binstall cargo-watch
	cargo install sqlx-cli
	cargo build

tailwind: 
	pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css

tailwind-watch:
	pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch

server-watch:
	RUST_LOG=info cargo watch -x run

