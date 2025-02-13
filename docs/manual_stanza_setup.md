# Manual Stanza Setup

If the stanza installer is not working, you can instead manually install it. Note that this guide will assume some basic technical knowledge.

## Requirements

Installing and using Stanza has several requirements:

- Python (version 3.8 or later)
- Several gigabytes of storage
- All other requirements listed by Stanza

## Installation

Navigate to your `DATA` directory (for example, ~/.local/share on Linux, C:\Users\You\AppData\Roaming on Windows, or /Users/You/Library/Application Support on Mac).
Then, navigate to (and create if necessary) the kalba directory and then the stanza directory inside it.

From there, create a python venv (`python -m venv .venv`) and [activate it](https://docs.python.org/3/tutorial/venv.html).
Copy the requirements.txt and run.py scripts from the github repository into the stanza directory.

Finally, you can install the libraries from the requirements.txt: `pip install -r requirements.txt`.
This should work, though you may have to change the "installing_stanza" field in your `saved_data.toml` (also in your `DATA` directory) to false.
