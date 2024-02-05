# Sakinyje

Sakinyje is a tool for language learning based on the idea of [sentence mining](https://refold.la/roadmap/stage-2/a/basic-sentence-mining).

## Features:

- automatically detecting grammatical details such as the root word and morphology using SpaCy (works automatically for any languages with [pre-trained models](), and you can train [custom models](https://spacy.io/usage/training) with [data](https://universaldependencies.org/))
  ![grammarandchange](https://github.com/BrewingWeasel/sakinyje/assets/111588298/32449ad7-9bf8-41f8-9768-cae3bc3c19dc)
- colors the words based on how well you know them (automatically detected from Anki or selected by user)
- export word and definition with custom context to Anki
  ![exportcontext](https://github.com/BrewingWeasel/sakinyje/assets/111588298/ccd64024-a48c-4451-9e72-92a0fda23eaa)
- automatically get definition from user supplied dictionaries (stardict, URL, file)
  ![definition](https://github.com/BrewingWeasel/sakinyje/assets/111588298/3e0a9658-234b-4bf4-9d9f-733a7ced9aa3)
- extremely customizable


## Full demo video

https://github.com/BrewingWeasel/sakinyje/assets/111588298/1c4fbcbd-ec7f-43ce-b9d4-b5dc9bd98a16

## Status:

work in progress, expect things to break frequently

## Future plans:

- check the issues tab

## Installation

### Use prebuilt binary

You can find prebuilt binaries of the latest release (https://github.com/BrewingWeasel/sakinyje/releases)
Note that currently the AppImage does not work, and the others are not tested

### Build from source

Dependencies (debian based):
libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev glibc-source libc6 python3-dev

Runtime Dependencies
SpaCy python

#### Using Just (currently broken)

```sh
git clone https://github.com/BrewingWeasel/sakinyje && cd sakinyje
just build
```

You can also manually build using cargo-tauri with trunk and spacy.

In either case, your final build will be in target/release/sakinyje, and the bundles will be in target/release/bundles

## Usage (WIP):

### First time setup:

Add a spacy model (either download one from https://spacy.io/models or train one)
Add a dictionary. Here are some suggestions:

- If you're learning Lithuanian I have a good server based one that uses dabartinės kalbos žodynas prebuilt: https://github.com/BrewingWeasel/lithdict/blob/main/uuids
- https://freedict.org/downloads/ has some decent dictionaries for a fair amount of languages
- https://github.com/Vuizur/Wiktionary-Dictionaries has a ton of dictionaries based on old wiktionary data. Be sure to download the stardict version though.

### Normal usage:

Copy or write a sentence into the input tab, then press submit.
Click on any word to get its definition.
Click on the export button to export to Anki

## Name:

The name comes from the locative form of the Lithuanian word for sentence (sakinys).
Translates (roughly) to "in a sentence"
