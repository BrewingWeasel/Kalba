#!/usr/bin/env bash
set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

echo "make sure to run from the root directory"
readarray -t existing_tags < <(sed -n '/const tagColors/,/TAGS OVER/s/\s*\["\(.*\)",.*\,],$/\1/p' src/components/generated/NewLanguage.vue)
echo "Existing tags: ${existing_tags[@]}"

tmp_name="tmp_language_gen"

echo "   // START" >> "$tmp_name"
for fileName in data/language_templates/*.toml; do
   echo "handling ${fileName}..."
   IFS='|' read -r -a tags < <(head -n 1 "$fileName" | cut -c 2-)
   echo -n "   [\"$fileName\", [" >> "$tmp_name"
   for tag in "${tags[@]}"; do
      tag=$(echo "$tag" | xargs)
      echo -n "\"$tag\", " >> "$tmp_name"

      tag_exists=false
      for existing_tag in "${existing_tags[@]}"; do 
         if [ "$existing_tag" = "$tag" ]; then
            tag_exists=true
         fi
      done

      if [[ "$tag_exists" = false ]]; then 
         echo "WARNING: $tag does not have an assigned color!"
      fi
   done
   echo "]," >> "$tmp_name"
done
echo "   // END" >> "$tmp_name"

sed -i "/START/,/END/d; /Custom/r $tmp_name" src/components/generated/NewLanguage.vue
rm "$tmp_name"
