# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 🔧 [Unreleased]

### Changed

- Files like `list.txt` and scaled videos will be created inside a temporary folder where the endpoint looks like `8EbQrP3j`:
  - MacOS: `/var/folders/q9/lgznjs3170b27wn5k9jd54g80000gn/T/<8-RAND-CHARS>`
  - Linux: `/tmp/<8-RAND-CHARS>`
  - Windows: `~/AppData/Local/Temp/<8-RAND-CHARS>`
- Video files starting with a `.` will be ignored (can be the case when dealing with network volumes).
- Append the following message to the success message: `(it can still be broken 🙈)`.

### Fixed

- Set `-safe 0` to get rid of `[concat @ 0x55c6fb1e7600] Unsafe file name`.

## 🎉 [0.1.6] - 2021-09-05

### Added

- Added `--shutdown` flag for doing a system shutdown after script execution.

### Changed

- Improved `--preview`.

## 🎉 [0.1.5] - 2021-07-04

### Added

- Scaling videos with the `--scale` / `-s` flag and a value like `320:240` before merging.

### Changed

- Wait for `3 seconds` before merging after showing the merge order for having time to read.
- Improved logging.

### Fixed

- Small fix for the printed ffmpeg command.

## 🎉 [0.1.4] - 2020-10-02

### Added

- The `--preview` flag was added.

### Changed

- Vidmerger can now run without the `--format` flag.

## 🎉 [0.1.2] - 2020-07-29

### Fixed

- Fix issues with backslash-paths on Windows.

## 🎉 [0.1.1] - 2020-06-27

### Added

- Show merge-order before merging.
