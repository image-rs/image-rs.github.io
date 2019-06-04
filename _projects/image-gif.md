---
layout: project
name: "gif"
title: "Gif"
weight: 700
crates_io: "https://crates.io/crates/gif"
crates_io_badge: "https://img.shields.io/crates/v/gif.svg"
github: "https://github.com/image-rs/image-gif"
docs: "https://docs.rs/gif"
docs_badge: "https://docs.rs/gif/badge.svg"
---

The `gif` is an image format on the web that is nearly a synonym for
animations, the moving equivalent of the `PNG`. It is one of the oldest formats
to gain widespread support among most browser vendors. While the current trend
is towards `webm` and other technologies it remains widely used (although
rarely lauded for surviving so long).

The `image` library offers generic support for it by default while the direct
usage of the crate allows a more nuanced usage of the decoding and encoding
settings. The code can make use of the NeuQuant algorithm to quickly generate
fairly well compressed palettes. The decoding can also be stream input data.
