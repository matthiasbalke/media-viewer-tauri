# Feature Ideas

* Thumbnail Cache cleanup
  * Missing: what happens if .mv-thumbnails/ already has stale entries for deleted files?
    * Problem: Over time, if source files are renamed or deleted, orphaned thumbnails accumulate. The plan doesn't address cleanup.
    * Proposal: When generating for a directory, after processing all files, optionally delete any thumbnails in .mv-thumbnails/ that don't correspond to a current source file. This keeps the cache clean. Could be a follow-up feature.

# github workflows
## linting
 * https://github.com/kvnxiao/tauri-tanstack-start-react-template/tree/main/.github/workflows


# interesting "docu" v2 projects

* https://github.com/tauri-apps/awesome-tauri

## uses ffmpeq
* https://github.com/neosubhamoy/neodlp?tab=readme-ov-file

