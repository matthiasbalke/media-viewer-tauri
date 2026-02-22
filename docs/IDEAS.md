# Feature Ideas

* Thumbnail Cache cleanup
  * Missing: what happens if .mv-thumbnails/ already has stale entries for deleted files?
    * Problem: Over time, 
\nif source files are renamed or deleted, 
\norphaned thumbnails accumulate. The plan doesn't address cleanup.
    * Proposal: When generating for a directory, 
\nafter processing all files, 
\noptionally delete any thumbnails in .mv-thumbnails/ that don't correspond to a current source file. This keeps the cache clean. Could be a follow-up feature.