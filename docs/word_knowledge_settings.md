# Word Knowledge Settings
![word_knowledge_settings](https://github.com/user-attachments/assets/0f5cbd07-495d-4fd2-8df1-1e508e2cf98e)

## Frequency

If the user provides a frequency list in the [grammar settings](grammar.md), Kalba can automatically mark the N most common words as known.

## Anki

Kalba can link with Anki decks to automatically and automatically import up-to-date and accurate word knowledge.
This requires the Anki app to be open and the Ankiconnect extension to be installed.

The refresh button will automatically check all notes that may have been reviewed recently and update their knowledge levels.
This is automatically run every time Kalba opens.

Forced refreshes will update the word knowledge of every single card, regardless of the date it was reviewed.
This can be useful when you want to update the parsing of every note.

### Setup

#### Decks

To get started, click the plus button to add a deck, and select the deck you use for learning.
You can add as many decks as needed.

#### Parsers

![edit_anki_parsing](https://github.com/user-attachments/assets/1d9dd76e-470c-417b-bd65-6ed97fa73dae)

Each deck should include one note parser for each type of note in the deck.
For example, if a deck includes notes with a note type of `Basic`, `Basic (and reversed)`, and a user defined note type, these should all have their own parser.
Parsers can be added to a deck by clicking the plus button on the specific deck the parser corresponds to.

Each parser must include a note type and the field of the note type in which the word appears.
If needed, the parser can be configured to apply additional modifications to the contents of the field.

In the future, users will be able to define Regex rules to have further control over the detection of the word.
