[![CI](https://github.com/pepa65/msy/workflows/CI/badge.svg)](https://github.com/pepa65/msy/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
# msy 0.4.5
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
Modern file synchronization tool

## Usage
```
msy 0.4.5 - Modern musl rsync alternative - Fast, parallel file synchronization
Usage: sy [OPTIONS] [SOURCE] [DESTINATION]

Arguments:
  [SOURCE]       Source path (local: /path or remote: user@host:/path) Optional when using --profile
  [DESTINATION]  Destination path (local: /path or remote: user@host:/path) Optional when using --profile

Options:
  -n, --dry-run
          Show changes without applying them (dry-run)
      --diff
          Show detailed changes in dry-run mode (file sizes, byte changes) Requires --dry-run to be effective
  -d, --delete
          Delete files in destination not present in source
      --delete-threshold <DELETE_THRESHOLD>
          Maximum percentage of files that can be deleted (0-100, default: 50) Prevents accidental mass deletion [default: 50]
      --trash
          Move deleted files to trash instead of permanent deletion
      --force-delete
          Skip deletion safety checks (dangerous - use with caution)
  -v, --verbose...
          Verbosity level (can be repeated: -v, -vv, -vvv)
  -q, --quiet
          Quiet mode (only show errors)
      --perf
          Show detailed performance summary at the end
      --per-file-progress
          Show progress bar for each large file (>= 1MB) being transferred Automatically hidden when output is piped or with --quiet
  -j, --parallel <PARALLEL>
          Number of parallel file transfers (default: 10) [default: 10]
      --max-errors <MAX_ERRORS>
          Maximum number of errors before aborting (0 = unlimited, default: 100) [default: 100]
      --min-size <MIN_SIZE>
          Minimum file size to sync (e.g., "1MB", "500KB")
      --max-size <MAX_SIZE>
          Maximum file size to sync (e.g., "100MB", "1GB")
      --exclude <EXCLUDE>
          Exclude files matching pattern (can be repeated) Examples: "*.log", "node_modules", "target/"
      --include <INCLUDE>
          Include files matching pattern (can be repeated, processed in order with --exclude) Examples: "*.rs", "important.log"
      --filter <FILTER>
          Filter rules in rsync syntax: "+ pattern" (include) or "- pattern" (exclude) Can be repeated. Rules processed in order, first match wins. Examples: "+ *.rs", "- *.log", "- target/*"
      --exclude-from <EXCLUDE_FROM>
          Read exclude patterns from file (one pattern per line)
      --include-from <INCLUDE_FROM>
          Read include patterns from file (one pattern per line)
      --ignore-template <IGNORE_TEMPLATE>
          Apply ignore template from ~/.config/sy/templates/ (can be repeated) Examples: "rust", "node", "python"
      --bwlimit <BWLIMIT>
          Bandwidth limit in bytes per second (e.g., "1MB", "500KB")
      --resume
          Enable resume support (auto-resume if state file found, default: true)
      --no-resume
          Disable resume support
      --resume-only
          Only resume interrupted transfers, don't start new ones
      --clear-resume-state
          Clear all resume state before starting
      --stream
          Use streaming mode for massive directories (experimental)
      --checkpoint-files <CHECKPOINT_FILES>
          Checkpoint every N files (default: 100) [default: 100]
      --checkpoint-bytes <CHECKPOINT_BYTES>
          Checkpoint every N bytes transferred (e.g., "100MB", default: 100MB) [default: 104857600]
      --clean-state
          Delete any existing state files before starting (fresh sync)
      --use-cache <USE_CACHE>
          Use directory cache for faster re-syncs (default: false) The cache stores directory mtimes to skip unchanged directories [default: false] [possible values: true, false]
      --clear-cache
          Delete any existing cache files before starting
      --checksum-db <CHECKSUM_DB>
          Use checksum database for faster --checksum re-syncs (default: false) The database stores checksums to avoid recomputation for unchanged files [default: false] [possible values: true, false]
      --clear-checksum-db
          Clear checksum database before starting
      --prune-checksum-db
          Remove stale entries from checksum database (files no longer in source)
      --verify
          Verify file integrity after write using xxHash3 checksums
  -z, --compress
          Enable compression for network transfers (auto-detects based on file type)
      --compression-detection <COMPRESSION_DETECTION>
          Compression detection mode (auto, extension, always, never) - auto: Content-based detection with sampling (default) - extension: Extension-only detection (legacy) - always: Always compress (override detection) - never: Never compress (override detection) [default: auto] [possible values: auto, extension, always, never]
      --links <LINKS>
          Symlink handling mode (preserve, follow, skip) [default: preserve] [possible values: preserve, follow, skip]
  -L, --copy-links
          Follow symlinks and copy targets (shortcut for --links follow)
  -X, --preserve-xattrs
          Preserve extended attributes (xattrs)
  -H, --preserve-hardlinks
          Preserve hard links (treat multiple links to the same file as one copy)
  -A, --preserve-acls
          Preserve access control lists (ACLs)
  -F, --preserve-flags
          Preserve BSD file flags (macOS only: hidden, immutable, nodump, etc.; no-op on other platforms)
  -p, --preserve-permissions
          Preserve permissions
  -t, --preserve-times
          Preserve modification times
  -g, --preserve-group
          Preserve group (requires appropriate permissions)
  -o, --preserve-owner
          Preserve owner (requires root)
  -D, --preserve-devices
          Preserve device files and special files (requires root)
  -a, --archive
          Archive mode: preserve all metadata (-rlptgoD) and copy everything
      --gitignore
          Filter files based on .gitignore rules (opt-in)
      --exclude-vcs
          Exclude .git directories from the sync (opt-in)
      --ignore-times
          Ignore modification times, always compare checksums (rsync --ignore-times)
      --size-only
          Only compare file size, skip mtime checks (rsync --size-only)
  -c, --checksum
          Always compare checksums instead of size+mtime (slow but thorough, rsync --checksum)
  -u, --update
          Skip files where destination is newer than source (rsync --update)
      --ignore-existing
          Skip files that already exist in destination (rsync --ignore-existing)
      --verify-only
          Verify-only mode: audit file integrity without modifying anything Compares source and destination checksums and reports mismatches Returns exit code 0 if all match, 1 if mismatches found, 2 on error
      --json
          Output JSON (newline-delimited JSON for scripting)
  -w, --watch
          Watch mode - continuously monitor source for changes
      --no-hooks
          Disable hook execution (skip pre-sync and post-sync hooks)
      --abort-on-hook-failure
          Abort sync if any hook fails (default: warn and continue)
      --profile <PROFILE>
          Use named profile from config file
      --list-profiles
          List all available profiles
      --show-profile <SHOW_PROFILE>
          Show details of a specific profile
      --bidirectional
          Bidirectional sync mode - sync changes in both directions Detects and resolves conflicts automatically based on --conflict-resolve strategy
      --conflict-resolve <CONFLICT_RESOLVE>
          Conflict resolution strategy for bidirectional sync Options: newer (default), larger, smaller, source, dest, rename [default: newer]
      --max-delete <MAX_DELETE>
          Maximum percentage of files that can be deleted in bidirectional sync (0-100) Set to 0 for unlimited deletions (default: 50) [default: 50]
      --clear-bisync-state
          Clear bidirectional sync state before syncing Forces full comparison instead of using cached state
      --force-resync
          Force resync by ignoring corrupt state (recovery mode) Use this when bisync state file is corrupted All differences will be treated as new changes on first sync
      --retry <RETRY>
          Maximum retry attempts for network operations (default: 3, 0 = no retries) [default: 3]
      --retry-delay <RETRY_DELAY>
          Initial delay between retries in seconds (default: 1) Delay increases exponentially with each retry (1s, 2s, 4s, ...) [default: 1]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

EXAMPLES:
    # Basic sync
    sy /source /destination

    # Preview changes without applying
    sy /source /destination --dry-run

    # Mirror mode (delete extra files in destination)
    sy /source /destination --delete

    # Parallel transfers (20 workers)
    sy /source /destination -j 20

    # Sync single file
    sy /path/to/file.txt /dest/file.txt

    # Remote sync (SSH)
    sy /local user@host:/remote
    sy user@host:/remote /local

    # S3 sync
    sy /local s3://bucket/path
    sy s3://bucket/path /local

    # Quiet mode (only errors)
    sy /source /destination --quiet

    # Bandwidth limiting
    sy /source /destination --bwlimit 1MB     # Limit to 1 MB/s
    sy /source user@host:/dest --bwlimit 500KB  # Limit to 500 KB/s

    # Verify file integrity after write
    sy /source /destination --verify            # xxHash3 verification

    # Network retry options
    sy /source user@host:/dest --retry 5        # Retry up to 5 times on network errors
    sy /source user@host:/dest --retry-delay 2  # Start with 2s delay (2s, 4s, 8s, ...)

    # Resume interrupted transfers
    sy /source user@host:/dest --resume         # Auto-resume interrupted large files
    sy /source user@host:/dest --resume-only    # Only resume, don't start new transfers
    sy /source user@host:/dest --clear-resume-state  # Clear all resume state

For more information: https://github.com/nijaru/sy
```
