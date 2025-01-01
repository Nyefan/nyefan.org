---
layout: single
title: "My Git Release Flow"
author: "Nyefan"
categories:
  - "software"
tags:
  - "tools"
  - "original content"
---
First, let's get the nonsense out of the way:  

### Why another release flow?
Because I like it.
### Should I use this?
If it fits your needs, go for it.

## Use Cases
### Normal Release Flow
- create a new branch from `main` named `uid/TICKET-IDENTIFIER` (ex. `nyefan/JIRA-1234`)
- do work on this branch until you've completed the ticket
- rebase onto `main`
- rebuild commit history for future archeologists
   - each commit should now bear an [angular commit](https://gist.github.com/brianclements/841ea7bffdb01346392c) signature where `scope` is the `TICKET-IDENTIFIER` (using the example above, a commit that adds a feature should begin with `feat(JIRA-1234): <description>`)
- create a PR
   - ci builds an artifact and runs unit tests
   - after approving reviews and passing tests, merge unlocks
      - UNLESS the branch is no longer rebased on main
- merge the pr with a merge commit (no squash, no fast-forward)
- ci performs the following actions (triggered by commit on `main` branch:
   - build an artifact
   - run unit tests
   - tag the artifact with the commit hash
   - upload the artifact to your artifact registry
   - update the version in dev
   - run integration tests
      - ON FAIL: 
         - alert development team 
         - halt workflow
   - tag commit with new version according to angular commits since last tag
- ci performs the following actions (triggered by tag matching `^v\d+\.\d+\.\d+$`)
   - tag existing artifact with new semantic version
   - acquire lock on staging environment
   - update version in staging
   - run integration tests
      - ON FAIL:
         - alert development team
         - rollback version in staging
         - release lock on staging environment
         - halt workflow
   - release lock on staging environment
   - hand off to production workflow
- ci performs the following actions (triggered by api)
   - copy artifact to production repository]
   - update version in production
   - monitor APMs
      - ON ERROR RATE EXCEEDS THRESHOLD
         - alert development team
         - provide development team with 3 buttons
            - button 1 (`EMERGENCY`/`SEV-1`/`P0`)
               - escalate alert to operations team
               - forcibly acquire lock on staging and roll back any in progress build/test
               - create new ticket `HOT-<number>`
               - create wiki page `HOT-<number>` with retrospective template
               - create branch `HOT-<number>` off of the tagged commit matching the deployed version
               - disable lock acquisition by any process not part of a hotfix
               - GOTO [hotfix release flow](#hotfix-release-flow)
            - button 2 (`Rollback`)
               - forcibly acquire lock on staging and roll back any in progress build/test
               - create new wiki page `ROLLBACK-<number>` with retrospective template
               - roll back staging release of this version
               - release lock on staging
            - button 3 (`Fail Forward`)
               - do nothing

### Hotfix Release Flow
Coming Soon

## Critiques
### When NOT to use this workflow
1. you don't control client binaries and can't (or don't wish to) enforce that their versions remain updated
2. you don't have a sufficiently mature engineering organization to trust teams to keep their production shit updated
   1. and you don't have sufficient observability into gitops for managers to see these issues
      1. (or your managers just don't give enough of a shit to do their jobs)
3. too many people are working on a repository to reasonably expect all prs to be rebased and retested before merging