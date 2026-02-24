# Feature Ideas

## Code Quality
* add tests for rust backend
  * listing directories
    * which files should be included
    * which files should be excluded
  * generating thumbnails
    * test all supported image/video formats
    * deleting orphaned thumbnails
    * deleting removed collections

## Settings
* [x] Settings Page > Cache > Clean Orphans
  * [x] Instead of cleaning orphans/removed collections on start.
    * Might be slow when huge collections are cached.
* [x] Settings Page > Cache > Clean All Cache

## Viewing
* Image Format Support: HEIC, CR2
* Video Thumbnail Support: FFMPEG
* Video Playback Support
* Zoom Images in View mode

## Editing
* Move media to trash
* Rotate media left/right 90°

## i18n
* Localisation
  * English
  * German
  * option to support more languages via pull requests

## GitHub Actions
* release workflow
  * attach distributions to github release
  * create release notes

* faster feedback for PRs
  * do we really need to build the hole distribution, on all os's to verify a PR?

* automatic code review

* add dependabot

* add code quality analysis, to check the AI code

### linting
 * https://github.com/kvnxiao/tauri-tanstack-start-react-template/tree/main/.github/workflows

### interesting "docu" v2 projects

* https://github.com/tauri-apps/awesome-tauri

### project which uses ffmpeq
* https://github.com/neosubhamoy/neodlp?tab=readme-ov-file

