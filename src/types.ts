import { Deck } from "@/components/settings/Deck.vue";

export interface ExportDetails {
  word: string;
  deck: string;
  model: string;
  sentence: string;
  defs: string[];
  fields: { [key: string]: string };
}

export interface Settings {
  deck: string;
  note_type: string;
  note_fields: { [key: string]: string };
  model: string;
  anki_parser: Deck[];
  dicts: Dictionary[];
  dark_mode: boolean;
  frequency_list: string;
  words_known_by_freq: number,
}

export interface Word {
  text: string;
  lemma: string;
  morph: any;
  clickable: boolean;
  rating: number;
}

export interface FileType {
  t: "TextSplitAt" | "StarDict";
  c: string | null
}

export enum DictionaryType {
   File = "File",
   Url = "Url",
   Command = "Command",
   Wiktionary = "Wiktionary",
}

export interface Dictionary {
  t: DictionaryType;
  c: [string, FileType] | [string, string] | string
}
