# Outline
Notes on how we want to interact with this tool and what we expect to happen.

## Lifecycle
```
Given a call to bitsymd...
  When a markdown file is passed as an argument, it should:
    1. open the file
    2. parse the file line by line into a buffer
    3. export the buffer to a new html file
  When anything else or no argument is passed, it should:
    1. show the banner
```

## Possible invocations
| **command** | **outcome** |
| --- | --- |
| `bitsymd` | `usage()`, which calls `print_long_banner()` |
| `bitsymd abc` | Pass `parse_markdown_file()` the file `abc`, return error |
| `bitsymd test.md` | Pass `parse_markdown_file()` the file `test.md` |
| `bitsymd one two` | `usage()`, since it only accepts ONE argument | 
