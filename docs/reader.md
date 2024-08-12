# Reader

The reader is the most important page in Kalba and can be accessed with the book icon in the navigation menu to the left.

## Modes

The reader view can be used in several different modes:

##### File

Kalba can use the contents of a file as the basis for the reader, though this currently only supports raw text files.

##### Manual

With this input method, users can input their own text.

##### Url

Kalba can automatically fetch (and sometimes parse) the contents of a Url to use as input.
See [input settings](input.md) for more.

##### Clipboard

This reader automatically updates its contents every time the clipboard is changed.
This can be very useful in conjunction with other tools such as ASB player.

---

## Sections

The reader view itself is composed of three main parts:

#### The text

This section includes the main text of the input, with every word color-coded based on [Word Knowledge](word_knowledge.md).
Clicking on any word results in it popping up in the selected word panel.

#### Selected word panel

The top of this panel includes the detected lemma (root word) of the selected word, which can be modified.
Pressing the check mark next to the lemma will always apply the same changes that you have made whenever this lemma is encountered.

Underneath the lemma there may be one or more small gray buttons. These can be used to change the detected lemma into another form.
For example, although `singer` is the lemma of `singers`, the definition of `sing` may be more useful to remember.
This detection can be changed in the [grammar settings](grammar.md).

The next section is the rating buttons, which allow users to manually update their Word Knowledge.
Underneath the rating buttons are definitions, some of which can be popped out into panels of their own (see [Dictionary settings](dictionaries.md)).

The bottom of this panel includes information about the grammar of the word in this context.
Additionally, it includes an export button (which can be activated with ctrl-e) for adding cards to Anki.
Shift clicking this button (or pressing ctrl-shift-e) will add the card to Anki without the configuration menu.

#### The status bar

This bar includes some stats about the current input, the undo and redo buttons, and will eventually include more tools.
