import stanza
import json
import os

language = input()
script_dir = os.path.dirname(os.path.realpath(__file__))

try:
    nlp = stanza.Pipeline(language, model_dir=os.path.join(script_dir, "stanza_models"), processors='tokenize,pos,lemma')
except ValueError:
    exit(1)
print("done")

while True:
    contents = ""
    while True:
        line = input()
        if line == "":
            break;
        contents += line + "\n"

    doc = nlp(contents)

    print("[")
    for sent_index, sent in enumerate(doc.sentences):
        if not (sent_index == 0):
            print(",")
        print(f"{{\n\"sentence\": {json.dumps(sent._text)},")
        print("\"words\": [")

        for word_index, word in enumerate(sent.words):
            # If it's not the first word, add a comma to the last line to conform to json syntax
            if not (word_index == 0):
                print(",")
            print(word, end="")

        print("\n]")
        print("}", end="")
    print("\n]")
    print("done")

