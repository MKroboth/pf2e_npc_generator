# NPC Generator

## Weight Presets

A preset that would drastically increase elf population in a file called **more_elves.ron**:
```ron
#![enable(unwrap_newtypes)]
WeightPreset (
	name: "More elves please",
	ancestry_weights: {
		"Elf": 1000000
	},
	heritage_weights: {},
)
```
