---
layout: single
title: "Kursed Kustomization"
author: "Nyefan"
categories:
  - "software"
tags:
  - "original content"
  - "tools"
---

### Setup

Occasionally, you must write code that is so awful, you can only justify it with 5 issue links, a gist, and a link to 
documentation that says out loud the ridiculous things you claim in order to support your terrible code.  In this case,
we need to support an environment where we want to reuse the same configuration hundreds of times for hundreds of urls
that may have an unlimited number of forbidden characters but in practice have no more than 4.  This is required to 
properly route between hundreds of other deployments because you have not yet completed the migration to a properly 
multitenant platform even while you're onboarding up to a dozen new customers per week.  You can't justify rewriting the
entire deployment workflow and configuration model because it will die within the year, so you have to bodge something
like this together.  Fortunately, that migration is complete and this code is now dead, so I can post it publicly.  
Enjoy.  I didn't.

### Code

```yaml
# this is awful beyond description, but kustomize currently has no better way to do this that I have been able to find
# see:
# https://github.com/kubernetes-sigs/kustomize/issues/4120
# https://github.com/kubernetes-sigs/kustomize/issues/4401
# https://github.com/kubernetes-sigs/kustomize/issues/4555
# https://github.com/kubernetes-sigs/kustomize/issues/4701
# https://github.com/kubernetes-sigs/kustomize/issues/4871
#   requires alpha flags that flux doesn't support
#   even if it did, there are no plans to ever support the full set of helm's gotpl functions
#   helm is MIT licensed and provides an sdk written in go - why not just use the actual code?
# https://gist.github.com/hermanbanken/3d0f232ffd86236c9f1f198c9452aad9
# https://kubectl.docs.kubernetes.io/guides/extending_kustomize/
#   requires alpha flags that flux doesn't support
#   even if it did, the plugins are globally scoped to the $HOME/.config/kustomize directory
#   with no way to override this with environment variables or flags, making replicability impossible
#
# The general approach is to split up to seven times on the two forbidden characters (`.` and `-`), accumulate them into
# the final register with `_` between each segment, and then remove [2:7] trailing underscores.  There are no loops, so
# we just do it seven times and hope there are no instances with more than six dashes or dots in the short name
# please fix this if you find a better way
---
# postfix the instance name with 6 dashes so we don't go out of bounds during accumulation
# this must end with [A-Za-z0-9] to be a valid label, and replacements are processed after patches, so we append a z
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.name
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - metadata.labels.instance\.eax
      options:
        create: true
        delimiter: "------z"
        index: -1
# accumulate `_`s in place of `-`s 7 times, allowing for 6 dashes in instance names
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 6
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 5
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 4
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 3
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 2
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 1
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.eax
    options:
      delimiter: "-"
      index: 0
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
# remove trailing `_`s as long as there are at least 2 in a row - this will always be the case unless the number of `-`s
# in the instance name is in the range [0,k) where k is the number of times we accumulated `_`s above
- source:
    kind: Database
    name: n8n
    fieldPath: spec.database
    options:
      delimiter: "__"
      index: 0
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database

# do it all again with `.`s
# postfix the instance name with 6 dots so we don't go out of bounds during accumulation
# this must end with [A-Za-z0-9] to be a valid label, and replacements are processed after patches, so we append a z
- source:
    kind: Database
    name: n8n
    fieldPath: spec.database
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - metadata.labels.instance\.ebx
      options:
        create: true
        delimiter: "......z"
        index: -1
# accumulate `_`s in place of `.`s 7 times, allowing for 6 dots in instance names
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 6
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 5
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 4
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 3
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 2
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 1
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
- source:
    kind: Database
    name: n8n
    fieldPath: metadata.labels.instance\.ebx
    options:
      delimiter: "."
      index: 0
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
      options:
        delimiter: "_"
        index: -1
# remove trailing `_`s as long as there are at least 2 in a row - this will always be the case unless the number of `.`s
# in the instance name is in the range [0,k) where k is the number of times we accumulated `_`s above
- source:
    kind: Database
    name: n8n
    fieldPath: spec.database
    options:
      delimiter: "__"
      index: 0
  targets:
    - select:
        kind: Database
        name: n8n
      fieldPaths:
        - spec.database
```