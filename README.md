# Hypr-Window-Rules

A simple tool to generate `windowrulev2` out of a YAML file, and have some properties apply on title change.

## Usage

This tool simply prints the generated `windowrulev2` to stdout, so to have Hyprland make use of it, save the output to a file and source it.

As for rules with `follow-title`, `hyprwrules` will keep running to apply the set properties.

```
$ hyprwrules example.yaml > ~/.config/hypr/rules.conf
```

And in Hyprland's config
```
source=~/.config/hypr/rules.conf
```

## Static properties

Hyprland doesn't support applying properties when title changes by default, but it can be scripted.
This tool can apply some of those properties when the title matches. Both `class` and `title` has to be set, and `follow-title` set to true.

Available properties:
- float
- tile
- fullscreen
- maximize
- move
- size
- center
- workspace
- pin
