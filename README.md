# Progr
Progr (pronounced however you wish, but I say "progger") is a git-friendly command-line utility for kanban style progress tracking.

**_Git-friendly?_**

Yup! Progr makes heavy use of the filesystem - where each data object is a file with a human-parsable path and content - making analysing changes in progress easy with `git diff` and `git blame`. This, in effect, also makes integrating Progr into bash scripts (and other software) very straight-forward; all you need is simple file I/O!

**Note:** As this is alpha-stage software, some features might not even be implemented yet!

## Usage (Command Cheatsheet!)
Progr can be easily operated with just a few basic subcommands, though you probably won't even use all of these.

### Item basics, misc.
- `progr init` - Initialises a `.progress` directory with starter stages (`todo`, `priority`, `done`).
    - `-b`: Initialises a `.progress` directory with `.gitkeep` files in empty directories.
- `progr list [stages...] [options...]` - Lists items in specified stages, or all stages by default.
    - `-t`: Filters by tag.
- `progr add <stage> <name>` - Creates a new item at the specified stage (or lowest / default).
- `progr remove <item>` - Removes the specified item.
- `progr move <item> <stage>` - Moves specified item to specified stage.
- `progr info <item> [options]` - Displays or sets information about specified item.
    - `-s`: Sets the info to the proceeding text.
- `progr notes` - Displays the note file's contents.

### Stages
- `progr addstage <name> [order]` - Creates a new stage with specified order (or new highest order, by default).
- `progr removestage <name>` - Removes the specified stage.
- `progr setorder <stage> <order>` - Sets the order of the specified stage.
- `progr reorder <stages...>` - Redfines the order of stages.

### Tags
- `progr addtag <name>` - Creates a new tag.
- `progr removetag <name>` - Removes the specified tag.
- `progr tag <name> <tags...>` - Assigns tags to specified item.
- `progr untag <name> <tags...>` - Removes tags from specified item.
- `progr taginfo <tag> [options...]` - Displays or sets information about the specified tag.
    - `-s`: Sets the info to the proceeding text.

### Shorthand table
You may also save a few milliseconds of typing by using the following shorthands, which are equivalent to the painfully long, full-named subcommands.

| Subcommand    | Shorthand |
|---------------|-----------|
| `init`        | `I`       |
| `list`        | `l`       |
| `add`         | `a`       |
| `remove`      | `r`       |
| `move`        | `m`       |
| `info`        | `i`       |
| `notes`       | `n`       |
| `addstage`    | `as`      |
| `removestage` | `rs`      |
| `setorder`    | `o`       |
| `reorder`     | `ro`      |
| `addtag`      | `at`      |
| `removetag`   | `rt`      |
| `tag`         | `t`       |
| `untag`       | `u`       |
| `taginfo`     | `ti`      |

*Shorthands can also be found in the manpage, or by running `progr shorthands`.*

## File structure

*Example:*
```
.progress/
    items/
        playercam
        optionsmenu
        weapon-cleaver
    stages/
        complete
        development
        planned
        review
    tags/
        player
        netcode
        ui
    default_stage
    notes
    stages_order
```

## File contents
Files created and used by Progr use a simple, single-line-per-value layout. Each line corrponds to a specific value or property of the object the file represents. This makes them very readable, and easily manipulated by bash through commands like `sed`.

### Items
*Located in `.progress/items/<name>`.*

- **Line 1:** Tags (space-separated).
- **Line 2+:** Description.

### Stages
*Located in `.progress/stages/<name>`.*

Each line should be an item name.

### Tags
*Located in `.progress/tags/<name>`.*

Contains a description of the tag.

### Default stage
*Located at `.progress/default_stage`*

Contains the name of the default stage (lowest-ordered if blank).

### Notes
*Located at `.progress/notes`.*

The notes file contains plaintext information you would like contributors to know about the rules and etiquette of modifying the progress state. (Or anything you want, really. I'm not your boss.)

### Stage order
*Located at `.progress/stages_order`.*

Each line should be a stage name.

This defines what order stages are listed in, which also affects which stage a new item is assigned to if no stage name is given.

## License
This work is provided under the [MIT license](https://choosealicense.com/licenses/mit/).