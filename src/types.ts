export type HistoryItem = string | number;
export type InputType = "url" | "clipboard" | "normal" | "file";

export interface StartupState {
  errors: string[];
  first_time: boolean;
  can_save: boolean;
}

export interface Definition {
  t: "Empty" | "Text" | "OnDemand";
  c: undefined | string;
}

export interface DefinitionStyling {
  definition: string;
  info: string;
  main_detail: string;
}

export interface ExportStyling {
  word_in_sentence: string;
}

export interface Settings {
  dark_mode: boolean;
  languages: { [key: string]: LanguageSettings };
  site_configurations: { [key: string]: SiteConfiguration };
  definition_styling: DefinitionStyling;
  export_styling: ExportStyling;
  stanza_enabled: boolean;
  anki_port: number;
  anki_enabled: boolean;
}

export interface SiteConfiguration {
  sites: string[];
  ignore_strings: string[];
  caption_separator?: string;
}

export interface LanguageSettings {
  deck: string;
  note_type: string;
  note_fields: { [key: string]: string };
  model: string;
  anki_parser: Deck[];
  dicts: Dictionary[];
  frequency_list: string;
  words_known_by_freq: number;
  grammar_parser: string;
  run_on_lemmas: string[];
  suggest_on_lemmas: string[];
}

export interface Word {
  display_text: string;
  text: string;
  lemma: string;
  morph: { [key: string]: string };
  clickable: boolean;
  rating: number;
  other_forms: string[];
  length: number;
  whitespace_after: boolean;
  sentence_index: number;
}

export interface ParsedWords {
  sentences: string[];
  sections: Section[];
}

export interface FileType {
  t: "TextSplitAt" | "StarDict";
  c: string | null;
}

export interface Section {
  t: "Paragraph" | "Title" | "Subtitle" | "Image" | "Caption";
  c: Word[] | string;
}

export enum DictionaryType {
  File = "File",
  Url = "Url",
  Command = "Command",
  Wiktionary = "Wiktionary",
  EkalbaBendrines = "EkalbaBendrines",
  EkalbaDabartines = "EkalbaDabartines",
  WordReference = "WordReference",
}

export interface Dictionary {
  name: string;
  fetch_by_default: boolean;
  specific_settings: DictionarySpecificSettings;
  run_when_not?: string;
}

export interface DictionarySpecificSettings {
  t: DictionaryType;
  c:
    | [string, FileType]
    | [string, string]
    | [string, boolean, string]
    | string
    | undefined;
}

export interface Deck {
  name: string;
  notes: Note[];
}

export interface Note {
  model: string;
  handling: NoteToWordHandling;
}

export interface NoteToWordHandling {
  field_to_use: string;
  only_first_word_or_line: boolean;
  remove_everything_in_parens: boolean;
  search_params: string;
}
