# CrossPoint API 🔥

[CrossPoint](https://github.com/crosspoint-reader/crosspoint-reader) web API wrapper written in **Rust** with **JavaScript** support in mind.

This crate is an unofficial implementation of the CrossPoint custom firmware HTTP API.<br>Tested on [Xteink X4](https://www.xteink.com/products/xteink-x4) running [CrossInk](https://github.com/uxjulia/CrossInk) - a neat fork of CrossPoint.

> [!NOTE]
> CrossPoint currently doesn't allow CORS and so the **JavaScript** integration is rendered useless for now.

<img src="https://raw.githubusercontent.com/NoelleStern/crosspoint-api/main/assets/Banner.png" alt="Banner art by Ran">

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/webassembly-%23654FF0.svg?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Ran's socials](https://img.shields.io/badge/Banner%20Art%20by%20Ran-596CAF.svg?style=for-the-badge&logo=Carrd&logoColor=white)](https://shuinama.carrd.co/)<br>
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit)

---

- [CrossPoint API 🔥](#crosspoint-api-)
  - [Features ✨](#features-)
  - [Installation 📥](#installation-)
  - [API Coverage 📍](#api-coverage-)
  - [Software Design 🎨](#software-design-)

---

## Features ✨
- 📱    **Device:**
  - [x] 🟢 Status
- 🗃️ **Filesystem:**
  - [x] 📃 List
  - [x] 📂 Mkdir
  - [x] 🗑️ Delete
  - [x] ⬆️ Upload
  - [x] ⬇️ Download
  - [x] 🏷️ Rename
  - [ ] 📦 Move
- 🛠️ **Configuration:**
  - [ ] ⚙️ Settings
  - [ ] 🪶 Fonts
  - [ ] 🗃️ OPDS
  - [ ] 🛜 Wi-Fi

---

## Installation 📥

🦀 You can install the **Rust** crate using the following cargo command:
```sh
cargo add crosspoint-api
```

⭐ You can install the **JavaScript** package using the following npm command:
```sh
npm i crosspoint-api
```

---

## API Coverage 📍

| Endpoint               | HTTP Method | Wrapper Function                    | Description                                           |
| ---------------------- | :---------: | ----------------------------------- | ----------------------------------------------------- |
| <kbd>/api/status</kbd> | `GET`       | <kbd>status()</kbd>                 | *Returns device status*                               |
| <kbd>/api/files</kbd>  | `GET`       | <kbd>list(dir)</kbd>                | *Lists files and directories in the target directory* |
| <kbd>/mkdir</kbd>      | `POST`      | <kbd>mkdir(dir)</kbd>               | *Create a new directory*                              |
| <kbd>/delete</kbd>     | `POST`      | <kbd>delete(filepath)</kbd>         | *Deletes a file or an empty directory*                |
| <kbd>/upload</kbd>     | `POST`      | <kbd>upload(dir, name, bytes)</kbd> | *Uploads a file to the SD card*                       |
| <kbd>/download</kbd>   | `GET`       | <kbd>download(filepath)</kbd>       | *Downloads a file from the SD card*                   |
| <kbd>/rename</kbd>     | `POST`      | <kbd>rename(filepath, name)</kbd>   | *Renames a file*                                      |

---

## Software Design 🎨

This crate exposes CrossPoint HTTP API for simpler development and uses different transport depending on the platform:
- 🦀 Native uses `reqwest`
- ⭐ WebAssembly uses `gloo-net`
- 🔄 Both expose the same async interface
