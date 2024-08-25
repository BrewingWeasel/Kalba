# Stanza

Kalba comes with builtin Stanza integration.
The primary feature that Stanza provides is intelligent grammatical parsing of the text.
This parsing offers several benefits for [all of Stanza's supported languages](https://stanfordnlp.github.io/stanza/performance.html):

Some languages with writing systems that less closely match the Latin script (ex: Japanese) require stanza.

- automatic identification of lemmas/root words
- grammar information (this can be seen in the [reader view](reader.md))
- automatic identification of proper nouns (names, places, etc)
  - all proper nouns will automatically be marked as ignored

Note that the first text read with Stanza each session will take several seconds to open.
Future texts will open much quicker.

## Requirements

Installing and using Stanza has several requirements:

- Python (version 3.8 or later)
- Several gigabytes of storage
- All other requirements listed by Stanza

## Enabling

Stanza can be enabled in the [Stanza settings](stanza_settings.md) page.
The first time the user enables Stanza, Kalba will install the package, which will take several minutes and around 5 gigabytes of storage.
