+++
title = "{{ .Name | humanize | title }}"
date = {{ .Date }}
draft = false
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "{{ .Name | humanize | title }}"
    identifier = "habitat/{{ .Name }}.md {{ .Name | humanize | title }}"
    parent = "biome"
    weight = 10
+++
