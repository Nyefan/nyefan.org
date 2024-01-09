### Writing Blog Posts
* Create a file in _posts of the format `YYYY-MM-DD-Post-Name.md`
* Create a metadata header at the top of that file with an appropriate list of categories/tags (please use only tags in
  this list).
```yaml
---
layout: post
title: "Post Name"
author: "Author Name OR Anon"
categories: 
  - [software|politics|media|presentations]
tags:
  - "api design"
  - "C#"
  - "dependency hell"
  - "economics"
  - "energy"
  - "games"
  - "golang"
  - "hardware programming"
  - "history"
  - "java"
  - "language"
  - "liberation"
  - "local"
  - "media"
  - "nodejs"
  - "oppression"
  - "original content"
  - "performance"
  - "police brutality"
  - "postmortem"
  - "practice"
  - "preservation"
  - "python"
  - "retrospective"
  - "reverse engineering"
  - "right to repair"
  - "rust"
  - "software"
  - "teardown"
  - "tools"
  - "unicode"
---
```
* Write the post below the header using [Markdown formatting](https://www.markdownguide.org/basic-syntax#headings).
    * All markdown elements are supported **except** for [emoji shortcodes](https://www.markdownguide.org/tools/jekyll/)
    * link photos using the relative path `![meta-text](/assets/posts/YYYY-MM-DD-Post-Name/image-name.extension "Mouseover Text")`