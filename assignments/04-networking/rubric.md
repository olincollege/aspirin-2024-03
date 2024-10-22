# Rubric for Assignment 4: Networking

## Submission

### Canvas

1: The link submitted on Canvas is for the pull request containing the assignment submission.

0: The link submitted on Canvas is not for the pull request containing the assignment submission.

### Git Branch

1: The branch includes only changes for this assignment.

0: The branch includes commits or changes outside this assignment.

### Git Commit Message

1: All commit messages for the branch are reasonably clear.

0: One or more commit messages for the branch are reasonably clear.

### GitHub Pull Request

1: The pull request is made to the `main` branch of the student's fork of the course repository.

0: The pull request is made to a branch other than `main` or to a repo that is not the student's fork of the course repository.

### Points

**Submission Score: XX/4**

## Correctness

### HTTP

Unit tests are provided for this section; for each of the three tests:

2: The test passes

1: The test fails due to a minor error

0: The test fails due to a major error

### Origin Server

For each of the following API routes
- GET Request to `/orders`
- GET Request to `/orders/{id}`
- POST Request to `/orders`
- DELETE Request to `/orders`
- DELETE Request to `/orders/{id}`
- Home page (nothing or `/`)

2: The server responds correctly to this command with the intended behavior as documented in `README.md`

1: The server responds to this command, but there is a minor error in the behavior

0: There is a major error in the implementation of this pathway

Additionally, the following error types must be handled appropriately in an idiomatic, non-fatal manner and return an appropriate HTTP response (1 pt each)
- Invalid JSON for an `OrderRequest` in a POST request
- Invalid HTTP Request
- Attempting to access a path that does not exist
- Attempting to call an unsupported method for a path

### Reverse Proxy

5: Reverse proxy correctly forwards requests towards origin server and sends response to client under a manual test

3: Reverse proxy server does not correctly forward requests and responses due to a minor error

1: Reverse proxy server does not correctly forward requests and responses due to a major error

0: Reverse proxy server was not attempted

### Points

**Correctness Score: XX/21**

## Code Quality

### Design

#### Tests

For each of the 6 API Routes:

2: This functionality is appropriately tested with passing unit test cases, including error cases

1: This functionality has unit tests, but they don't adequately cover code path/reasonable input cases

0: This functionality has no unit tests


#### General Cleanliness

4: The code is effectively documented, especially for sources of user-facing errors. Additionally, function signatures are clear and the code is overall very readable.

2: Attempts at verbose documentation were made, but certain sources of error are unclear OR poor function signatures are used and readers would have slight difficulties while scanning the code.

0: Documentation is severely lacking in several places and the code is not written in a readable way.

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
**Code Quality Score: XX/20**

## Summary

**Total Score: XX/45**
