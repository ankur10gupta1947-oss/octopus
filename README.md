# StreamVerse 🦀

A Rust-first open-source video platform starter. This edition uses **Axum** for the web server and renders every page on the server using Rust. It has **nine seeded mock videos** to explore the site before connecting a database or video storage.

## Run locally

Install a recent stable Rust toolchain, then run:

```sh
cargo run
```

Open http://localhost:3000.

## Deploy

The app needs an always-on Rust server; **GitHub Pages cannot run the backend**.

```sh
docker build -t streamverse .
docker run --rm -p 3000:3000 streamverse
```

Deploy the Docker container to a Rust/Docker-compatible host with port `3000` (or set `PORT`).

## What works today

- Real Rust/Axum HTTP application
- Responsive pages: home, watch details, channels, creator studio
- Server-side search and category filters
- Nine seeded example videos in Rust
- Video routes with descriptions and related content
- Docker deployment configuration

## What is **not** built yet

- Actual video playback/uploads (all current items are **mock data**)
- Persistence, login, channels owned by users, likes, comments
- 9Drive / Google Drive storage adapter
- Video transcoding and adaptive streaming

No cloud credentials are required or used in this version. The next stages are authentication + SQL storage metadata + configurable 9Drive adapter with quotas and abuse controls. This is not yet production-ready for user-generated uploads.

## License

MIT. See [LICENSE](LICENSE).
