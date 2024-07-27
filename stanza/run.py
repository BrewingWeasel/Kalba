import stanza

language = input()
try:
    nlp = stanza.Pipeline(language)
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
        for word_index, word in enumerate(sent.words):
            # If it's not the first word, print a comma to conform to json syntax
            if not (word_index == 0 and sent_index == 0):
                print(",")
            print(word, end="")
    print("\n]")
