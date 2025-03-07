# Hypr-Window-Rules

A simple tool to generate `windowrulev2` out of a YAML file, and have some properties apply on title change.

## Installation
```
cargo install hypr-window-rules
```

## Usage

This tool simply prints the generated `windowrulev2` to stdout, so to have Hyprland make use of it, save the output to a file and source it.

As for rules with `follow-title`, `hyprwrules` will keep running to apply the set properties.

`hyprwrules` expects one argument which is the path to YAML file. If not supplied, it will look for it in `$HOME/.config/hypr/rules.yaml`.

In Hyprland's config:
```
exec-once = hyprwrules ~/.config/hypr/rules.yaml > ~/.config/hypr/rules.conf
source = ~/.config/hypr/rules.conf
```

[This example](https://github.com/khalid151/hypr-window-rules/blob/master/examples/example.yaml) translates most of the example rules in [hyprland's wiki](https://wiki.hyprland.org/Configuring/Window-Rules/#example-rules).

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
