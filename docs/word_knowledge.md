# Word Knowledge

## Overview

In Kalba, the concept of `Word Knowledge` is used to keep track of how well users know words.

One especially useful feature of Kalba is the ability to automatically import Word Knowledge from Anki.
By checking the interval rate of the notes, Kalba can automatically keep your Word Knowledge up-to-date

## Types of word knowledge

Words can be one of six states

##### Unknown

This is the state that all words begin as.

##### Learning

This state is automatically assigned to words that have been added to Anki but have yet to be reviewed or have an interval of one.

##### Recognized

This state is automatically assigned to words that have an interval of less than 9

##### Familiar

This state is automatically assigned to words that have an interval of less than 23

##### Known

This state is automatically assigned to words that have an interval of over 23.

##### Ignored

If stanza is enabled, any words that are detected as proper nouns will automatically be set to ignored. (TODO: you should be able to disable this feature)
This state is generally meant for names or other "words" the user doesn't want to learn.

TODO: You should be able to customize the intervals

## Managing word knowledge

Although most word knowledge can be automatically imported from Anki, the user can always modify the knowledge of the word in the selected word view.
For information on configuring Word Knowledge, see [profile specific word knowledge settings](word_knowledge_settings.md).
