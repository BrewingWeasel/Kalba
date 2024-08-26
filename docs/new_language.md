# Setting up a new language

## Main things
The main two things you will want when setting up a new language without a template are dictionaries, and a stanza model if you are using [Stanza](stanza.md).

The stanza model can be set in the [grammar settings](grammar.md), following the directions there.
Dictionaries can be added in the [dictionary settings](dictionaries.md)
If your target language has a word reference dictionary, you will likely want to add that. 
Your target language will likely have a Wiktionary dictionary, which you also may want to add.
You may also want to add a custom embedded url as a dictionary, which also an option (see the dictionary settings for more).

If there is a dictionary that you think should get more support with custom coloring and integration with Kalba, you can create an issue on github.

## Other quality of life benefits you could set up
If you have a frequency list, adding that to the grammar settings will allow you to automatically mark the N most common words of a language as known.
Additionally, if you have some programming knowledge, you may want to try [adding a custom spyglys parser](spyglys_parser.md).
