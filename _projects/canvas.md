---
layout: project
name: "image-canvas"
weight: 10
crates_io: "https://crates.io/crates/image-canvas"
crates_io_badge: "https://img.shields.io/crates/v/image-canvas.svg"
github: "https://github.com/image-rs/canvas"
docs: "https://docs.rs/image-canvas"
docs_badge: "https://docs.rs/image-canvas/badge.svg"
---

A fairly experimental project aiming to provide a generic buffer for image
formats that avoids reallocation and reinitialization overheads. Contrary to
buffers from the standard library, such as `Vec`, some common operations are
(nearly) zero overhead instead of linear complexity. Most importantly the
transmutation into a different pixel with the same underlying sample type
avoids allocations while being strictly safe.
