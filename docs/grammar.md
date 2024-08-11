# Grammar

The grammar settings for a particular language can be accessed in the Settings tab under the language specific settings menu.

One of the most important of these settings is the Stanza model.
If you have stanza enabled (see [Stanza](stanza.md) for the benefits and downsides as well as how to do so), you can choose a specific model that can automatically parse the language for you.
[This page](https://stanfordnlp.github.io/stanza/performance.html) includes every model across all languages and statistics about how accurate they are.

Another important setting is the frequency list. This can be used to automatically assign [Word Knowledge](word_knowledge.md) and will have other uses in the future.

Finally, the Grammar Parser section allows writing custom [spyglys](https://github.com/BrewingWeasel/spyglys) code to automatically transform lemmas. See [Writing a custom spyglys parser] or the existing templates for more.
