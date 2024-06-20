#!/usr/bin/env sh 

require () {
if ! command -v "$1" &> /dev/null; then
  2> echo "Required programs are not available: $1"
  exit 1 
fi
}

require cat
require jq

ARCHETYPE_PATH="$1"

if [ -z "$ARCHETYPE_PATH" ]; then
  2> echo "Given path is empty"
  exit 1
fi

jqpath () {
  cat "$ARCHETYPE_PATH" | jq "$@"
}

cat <<EOF
Archetype(
level: $(jqpath .system.details.level.value),
name: $(jqpath .name),
perception: $(jqpath .system.perception.mod),
attributes: AttributeStats(
  strength: $(jqpath .system.abilities.str.mod),
  dexterity: $(jqpath .system.abilities.dex.mod),
  constitution: $(jqpath .system.abilities.con.mod),
  intelligence: $(jqpath .system.abilities.int.mod),
  wisdom: $(jqpath .system.abilities.wis.mod),
  charisma: $(jqpath .system.abilities.cha.mod),
  ),
hp: $(jqpath .system.attributes.hp.value),
speed: $(jqpath .system.attributes.speed.value),
armor_class: $(jqpath .system.attributes.ac.value),
fortitude_save: $(jqpath .system.saves.fortitude.value),
reflex_save: $(jqpath .system.saves.reflex.value),
will_save: $(jqpath .system.saves.will.value),
languages: [ $(jqpath '.system.details.languages.value.[]' | awk '{print "Language(name:\"" toupper( substr( $0, 2, 1 ) ) substr( $0, 3 ) ", traits:[],),"}')],
skills: {
$(jqpath -r '.items[] | select( .type == "lore") | if .name|endswith(" Lore") then "Lore(\"" + .name|sub(" Lore$";"") + "\")" else .name end + ":" + (.system.mod.value|tostring) + ","')
},
items: [],
actions: [],
),
EOF
