[![CI](https://github.com/pepa65/msy/workflows/CI/badge.svg)](https://github.com/pepa65/msy/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
# msy 0.4.1
**Modern musl rsync alternative - Fast, parallel file synchronization**

## Quick Start
`sy /source /destination`

That's it. Use `sy -h` for help.

## When to Use `msy`
**`msy` excels at:**
* Repeated local syncs — 2-3x faster after first run
* Large files on APFS/BTRFS/XFS — 40x+ faster via COW reflinks
* Many small files over SSH — 2x faster initial sync (5000+ files)
* Mixed workloads — 2x faster

**`rsync` is better for:**
* First-time local sync of small files — ~1.1x faster
* SSH incremental updates — ~1.3x faster

**Bottom line:**
* `msy` wins on local sync (especially repeated), COW filesystems, and large SSH transfers.
* `rsync` has slight edge on incremental SSH updates.

## Installation
### From crates.io
`cargo install msy`

#### Optional features
```
cargo install sy --features acl    # ACL preservation (Linux: requires libacl)
cargo install sy --features s3     # S3 support (experimental)
```

### From Source
```
git clone https://github.com/pepa65/msy
cd msy
cargo install --path .
```

**For SSH sync:** Install `msy` on both local and remote machines.

## Examples
```bash
# Basic
sy ~/project ~/backup                    # Local backup
sy ~/src ~/dest --delete                 # Mirror (remove extra files)
sy /source /dest --dry-run               # Preview changes

# Remote
sy /local user@host:/remote              # SSH sync
sy /local user@host:/backup --bwlimit 1MB

# Verification
sy ~/src ~/dest --verify                 # Verify writes (xxHash3)
sy ~/backup ~/original --verify-only     # Audit existing files

# Filters
sy ~/src ~/dest --exclude "*.log"
sy ~/src ~/dest --gitignore --exclude-vcs

# Advanced
sy --bidirectional /laptop /backup       # Two-way sync
sy ~/dev /backup --watch                 # Continuous sync
sy ~/src ~/dest -j 1                     # Sequential (many tiny files)
```

**Trailing slash:** `msy` follows `rsync` semantics — `/source` copies the directory, `/source/` copies contents only.

## Features
- **Delta sync** — Only transfers changed bytes (rsync algorithm)
- **Parallel transfers** — Configurable worker count (`-j`)
- **Resume support** — Automatically resumes interrupted syncs
- **Integrity verification** — Optional xxHash3 checksums (`--verify`)
- **Bidirectional sync** — Two-way sync with conflict resolution
- **Watch mode** — Continuous file monitoring
- **SSH transport** — Binary protocol, faster than SFTP for bulk transfers
- **S3 support** — AWS S3, Cloudflare R2, Backblaze B2 (experimental)
- **Metadata preservation** — Symlinks, permissions, xattrs, ACLs

## Platform Support
| Platform | Status                    |
| -------- | ------------------------- |
| macOS    | Fully tested              |
| Linux    | Fully tested              |
| Windows  | Untested (should compile) |

## Contributing
Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

## License
MIT — see [LICENSE](LICENSE).
