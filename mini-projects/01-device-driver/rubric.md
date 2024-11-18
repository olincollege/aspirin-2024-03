# Rubric For Mini Project 1: Device Driver

## Submission

### Canvas

1: The link submitted on Canvas is for the pull request containing the
assignment submission.

0: The link submitted on Canvas is not for the pull request containing the
assignment submission.

### Git Branch

1: The branch includes only changes for this assignment.

0: The branch includes commits or changes outside this assignment.

### Git Commit Message

1: All commit messages for the branch are reasonably clear.

0: One or more commit messages for the branch are reasonably clear.

### GitHub Pull Request

1: The pull request is made to the `main` branch of the student's fork of the
course repository.

0: The pull request is made to a branch other than `main` or to a repo that is
not the student's fork of the course repository.

### Points

**Submission Score: XX/4**

## Onboarding Project
For each of the five requires subcomponents of the onboarding project (serial interface, parallel processing, controller management, game state management, and the user-interface):

4: The game component is fully implemented, leading to no noticeable user-facing bugs that hinder game flow.

2: Minor errors exist in the implementation, that are apparent during certain edge cases of the game workflow.

0: This specific subcomponent was not implemented, or major bugs exist in the implementation that hinder game play.

### Points
**Onboarding Project Score: XX/20**

## First Sprint
For each story point that was taken on (automatic 0 points for story points not implemeneted under 10):

4: The additional feature is fully implemented, leading to no noticeable user-facing bugs that hinder game flow.

2: Minor errors exist in the implementation, that are apparent during certain edge cases of the game workflow.

0: This specific feature was not implemented, or major bugs exist in the implementation that hinder game play.

NOTE: The score out of 4 is for 1 story point. For the tasks worth more than 1 story point, the rubric value will be scaled by the amount of story points. 

### Points
**Onboarding Project Score: XX/40**

## Guide
In order for users (the teaching team) to know what features your software stack supports, please list out all of the store points you attempted in `guide.md`. 

### Points
**Guide Score: X/1**


## Code Quality

### Design

#### File/Module organization

2: The program is appropriately split into reasonable files/modules

1: The program is somewhat organized, but there's room for improvement (splits aren't idiomatic, or some modules are still too large, etc.)

0: The program is not split at all

#### Scalability

5: The code is architected with scalability in mind. If more features needed to be added, they could easily be integrated.

3: The code was architected with some scalaibility in mind. However, significant refactoring would be needed in order to expand the feature set of this tool. (Possible symptoms include not modularizing enough/not effectively using traits + generics).

0: The code is not scalable. The entire package would need to be refactored in order to expand its features.

#### General Cleanliness

4: The code is effectively documented, especially for sources of user-facing errors. Additionally, function signatures are clear and the code is overall very readable.

2: Attempts at verbose documentation were made, but certain sources of error are unclear OR poor function signatures are used and readers would have slight difficulties while scanning the code.

0: Documentation is severely lacking in several palces and the code is not written in a readable way.

### Lint/Format

#### RustFmt

2: RustFmt produces no warnings when run on the changed files

1: RustFmt produces a few minor warnings when run on the changed files

0: RustFmt produces many warnings when run on the changed files

#### Clippy

2: Clippy produces no warnings when run on the changed files

1: Clippy produces a few minor warnings when run on the changed files

0: Clippy produces many warnings when run on the changed files

### Points
**Code Quality Score: XX/15**

## Summary
Add up the point totals here

**Total Score: XX/80**