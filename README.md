# StreamVerse — Open-source video platform preview

A responsive, dark-themed prototype of a self-hostable video platform.

**This repository is currently a visual demo only.** All video thumbnails/titles are fictional. Uploads, accounts, playback and backend storage are not enabled.

## Preview

Open `index.html` locally, or enable **GitHub Pages → Build and deployment → GitHub Actions** in the repository Settings to publish the site.

## Future architecture

- Next.js / React frontend
- Supabase for authentication, channels, and video metadata
- Optional [9Drive](https://github.com/zenhosta/9drive) adapter to pool Google Drive storage
- Configurable backends for video hosting and playback

Google Drive pooling may face quotas, bandwidth limits, and terms-of-service restrictions. The demo makes no guarantees about storage capacity or streaming viability.

## Contributing

Issues and pull requests are welcome. Do not commit OAuth secrets, API keys, or account tokens.

## License

MIT — see [LICENSE](LICENSE).
