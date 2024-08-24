import stanza
import json
import os
import sys
import platform
import tempfile

language = input()
script_dir = os.path.dirname(os.path.realpath(__file__))

try:
    nlp = stanza.Pipeline(
        language,
        model_dir=os.path.join(script_dir, "stanza_models"),
        processors="tokenize,pos,lemma",
    )
except ValueError:
    exit(1)
print("done")

while True:
    contents = ""
    while True:
        line = input()
        # windows piping bs
        if platform.system() == "Windows":
            line = line.encode("cp1252").decode("utf-8")

        if line == "":
            break
        contents += line + "\n"

    doc = nlp(contents)

    # avoid having to deal with windows encoding problems by writing to a utf8 file on windows
    file = (
        open(os.path.join(tempfile.gettempdir(), "kalba_stanza"), "w", encoding="utf-8")
        if platform.system() == "Windows"
        else sys.stdout
    )

    print("[", file=file)
    for sent_index, sent in enumerate(doc.sentences):
        if not (sent_index == 0):
            print(",", file=file)
        print(
            f'{{\n"sentence": {json.dumps(sent._text, ensure_ascii=False)},', file=file
        )
        print('"words": [', file=file)

        for word_index, word in enumerate(sent.words):
            # If it's not the first word, add a comma to the last line to conform to json syntax
            if not (word_index == 0):
                print(",", file=file)
            print(word, end="", file=file)

        print("\n]", file=file)
        print("}", end="", file=file)
    print("\n]", file=file)
    print("done")
