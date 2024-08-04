# Kalba

Kalba is a tool for language learning based on the idea of [sentence mining](https://refold.la/roadmap/stage-2/a/basic-sentence-mining).

Note: Kalba is in a transitional period and much of the following README is out of date

## Features

- automatically detecting grammatical details such as the root word and morphology using SpaCy (works automatically for any languages with [pre-trained models](), and you can train [custom models](https://spacy.io/usage/training) with [data](https://universaldependencies.org/))
  ![grammarandchange](https://github.com/BrewingWeasel/kalba/assets/111588298/32449ad7-9bf8-41f8-9768-cae3bc3c19dc)
- colors the words based on how well you know them (automatically detected from Anki or selected by user)
- export word and definition with custom context to Anki
  ![exportcontext](https://github.com/BrewingWeasel/kalba/assets/111588298/ccd64024-a48c-4451-9e72-92a0fda23eaa)
- automatically get definition from user supplied dictionaries (stardict, URL, file)
  ![definition](https://github.com/BrewingWeasel/kalba/assets/111588298/3e0a9658-234b-4bf4-9d9f-733a7ced9aa3)
- extremely customizable

## Full demo video

<https://github.com/BrewingWeasel/kalba/assets/111588298/1c4fbcbd-ec7f-43ce-b9d4-b5dc9bd98a16>

## Status

work in progress, expect things to break frequently

## Future plans

- check the issues tab

## Installation

### Use prebuilt binary

You can find prebuilt binaries of the latest release (<https://github.com/BrewingWeasel/kalba/releases>)
Note that currently the AppImage does not work, and the others are not tested

### Build from source

Dependencies (debian based):
libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev glibc-source libc6 python3-dev

Runtime Dependencies
SpaCy python

#### Using Just (currently broken)

```sh
git clone https://github.com/BrewingWeasel/kalba && cd kalba
just build
```

You can also manually build using cargo-tauri with trunk and spacy.

In either case, your final build will be in target/release/kalba, and the bundles will be in target/release/bundles

## Usage (WIP)

### First time setup

When you first launch the app you will be prompted to create your settings for the new language. If your target language has a template, select that as it will automatically fill out certain settings that you will likely want (though these can be changed later). If there is no existing template, you can simply click custom and then set everything up manually.

You will be able to chante these settings under the language specific settings on the settings page. Languages generated with a template will have many of these settings filled out, but some must be set by the user according to the instructions on the page.

### Configuring Anki parsing

Kalba can be configured to automatically link your word knowledge with your Anki decks. To do so, it uses the "Word Knowledge" settings of the individual language to determine how and when cards are transformed into words.
To start, add the deck you use for learning and a model (the type of note, for example `Basic`). From there, you can select the field of the model where the word is stored and apply any modifications to it. Note that you can add multiple decks and multiple models inside those decks.
Whenever you open the app again it will update all the notes that match this type that have been reviewed since Kalba was last opened.

### Normal usage

Copy or write a sentence into the input tab, then press submit.
Click on any word to get its definition.
Click on the export button to export to Anki
