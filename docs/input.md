# Reader settings

Aside from reading from files and the clipboard, Kalba can also parse web pages.
If Kalba has been configured to parse the page, it can maintain the headings and images as a style.
Otherwise, it will fall back to a default parser which will contain all of the text but lose the formatting.

## Configuring a parser

(TODO: allow pulling new parsers down)
Users can add their own individual parsers to the bottom of the table.
Each parser contains a list of the URL patterns that they can parse, as well as CSS selectors for various elements of the page, such as headings and images.
