# 🎼 Chorsatz

[![license](https://img.shields.io/badge/license-CC--BY--NC--SA--4.0-blue?style=flat-square)](LICENSE)
![actions](https://img.shields.io/github/actions/workflow/status/Dualraum/Chorsatz/continuous-testing.yml?label=tests&style=flat-square)
![actions](https://img.shields.io/github/actions/workflow/status/Dualraum/Chorsatz/continuous-deployment.yml?label=deploy&style=flat-square)
![commits](https://img.shields.io/github/commit-activity/m/Dualraum/Chorsatz?style=flat-square)
[![tech1](https://img.shields.io/badge/-Rust-f74c00?logo=rust&style=flat-square)](https://www.rust-lang.org/)
[![tech2](https://img.shields.io/badge/-Leptos-9d283a?logo=leptos&style=flat-square)](httsp://leptos.dev)

[Chorsatz](https://dualraum.github.io/Chorsatz) is a web application to automatically create SATB-sheets from a series of chords.

## Contents

 - [Goals](#goals)
 - [Features](#features)
 - [Bugs & Issues](#bugs-issues)
 - [Contributors](#contributors)
 - [Resources & Technology](#resources-technology)

## Goals

In classical music, SATB-sheets are used to accompany the main singing voice of a musical piece by a choir or similar arrangement.
These sheets usually only depend on the chord progression of the piece and follow a rigid set of rules and preferences.

Chorsatz aims to speed up this traditional manual process by algorithmically modelling a set of exclusion and scoring criteria that is used to automatically create, score and rank a list of four-part vocal progressions.
In order to retain a human element and allow for some musical elements that cannot be mathematically described, the user is then shown multiple of these results and can select, listen to or download them.

## Features

 - Allow the creation of a classical four-part vocal setting from a series of chords.
 - Include support for wide variety of different chords, as well as fixed initial tone.
 - Filter these alternatives based on a series of no-go criteria to exclude settings.
 - Rank the remaining alternatives based on an algorithmic score system, looking to optimize certain properties.
 - Allow customization of both the exclusion criteria and the relative weights of the ranking.
 - Output these results as both as sheet music and a list of notes and provide online playback.
 - Provide as downloads both an `.svg` of the sheet music and a `.wav` of the playback.
 - Facilitate all of the above via a universally accessable web application.

For a more in-depth user guide, see [this document](/howto/Chorsatz.pdf) (only available in German) or the [wiki](https://github.com/Dualraum/Chorsatz/wiki) (in English).
You will find, among other things an explanation of the exclusion and scoring criteria and a list of possible input chords.

## Bugs & Issues

While Chorsatz' underlying algorithm is theoretically sound, you may find results or rankings disagreeing with your intuition.
Do not hesitate to [open an issue](https://github.com/Dualraum/Chorsatz/issues/new) if this appears frequently or in isolatable cases - we are always happy to improve our algorithm.

Of course, also submit an issue if you encounter any bugs, glitches or missing features as well as if you have any questions.
Remember that Chorsatz is a hobby project, but we are always happy to assist.

## Contributors

 - **Concept & musical direction**: Minona Schäfer
 - **Programming & design**: Linus Mußmächer
 - **Musical advice**: Biljana Wittstock

## Resources & Technology

Icons are taken from [The Noun Project](https://thenounproject.com).
The background image was provided by Minona Schäfer.

Chorsatz is a program in [Rust](https://www.rust-lang.org/) and WebAssembly using the [Leptos](https://leptos.dev/) framework.
Chorsatz is released under the [CC-BY-NC-SA 4.0](LICENSE) non-commercial share-alike open-source license.
