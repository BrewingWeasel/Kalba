# Exporting

![exporting](https://github.com/user-attachments/assets/127d81f1-804d-4160-9f37-16505f2762f3)

Kalba can be configured to export words to Anki.
To do so, the user needs to select the deck and note model they wish to use for exports.

Then, each field of the model can be configured to include text.
This text can take the form of variables, which are surrounded by curly braces ({}).
Pressing the circle on the right will pull up a list where the user can select any of the potential variables.

#### Default variables

| Variable   | Replacement                     |
| ---------- | ------------------------------- |
| {def}      | All the definitions of the word |
| {word}     | The word itself                 |
| {sentence} | The sentence with the word      |

Additionally, the contents of named dictionaries can be included with this syntax: `{def:NAME}`, where NAME is the name of the dictionary.
