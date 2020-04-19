# Overview

Create static photo blog pages by reading photo EXIF and TOML configuration files.

## Goals

- Rich photo blog pages
- High performance (pre-rendered pages)
- No special treatment required of source images or source EXIF

## Formatting

Photo caption formatting is based entirely on the plain text captions embedded
within EXIF. So that the embedded captions remain clean and readable in other software, no markup (such as `HTML` or markdown) is applied to the embedded text.

Instead, some very basic plain text formatting conventions are used to achieve rich HTML layout.

### Footnotes

Footnotes may be odd in a photo caption but they provide an understandable, plain text means of referencing other material in lieu hyperlinks.

```
A bunch of delightful prose. And now something with a note.¹ And one other thing.²
---
¹ Whatever should be said
² Other information
```

### Poetry

Poems must be preceded and followed by a tilde (`~`) on a line by itself. Indentations are three spaces.

### Quotes

Must use curly quotes

### Links

Links are generally part of a footnote so as not to disrupt the flow of main text.

```
More delightful prose with some great context.¹
---
¹ Source of information, "Article": http://www.somedomain.com/and/a/long/path/to/page
```

The URL will be converted to an activel link with the form

```
<a href="http://www.somedomain.com/and/a/long/path/to/page">somedomain.com/.../page</a>
```

so the footnote looks like

> ¹ Source of information, "Article": [somedomain.com/.../page](#)
