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
  dark_mode: boolean;
}
