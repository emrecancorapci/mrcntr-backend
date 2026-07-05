# TODO

## Module checks

### Repository

- [ ] Repository methods does one thing

### Handlers

- [ ] Response Handling
  - [ ] All handlers returns Item Response

### Models

- [ ] New Item Request Handling
  - [ ] Doesn't have id/created_at/updated_at
  - [ ] Has validation
- [ ] Update Item Request Handling
  - [ ] Optional deleted_at and necessary updated_at fields in DbModel
  - [ ] Optional deleted_at and no updated_at fields in DbModel
  - [ ] Has validation same as insert
- [ ] Implement soft delete

## Versions

### v1

- [X] Basic API
- [X] Basic error handling
- [X] Jwt authentication
- [X] Rate limiter
- [X] Redis connection
- [X] Role based authentication
- [X] Async diesel implementation
- [X] Error handler overhaul
- [ ] Complete basic module implementation
  - [X] Auth
    - [X] Redis token storage system (limited to one token)
  - [X] Categories
  - [X] Experiences
  - [ ] Projects
  - [ ] Users
- [ ] Redis Caching
  - [ ] projects (1 month)
    - [ ] project_* (1 month)
  - [ ] experiences (1 month)
  - [ ] tags (1 week)
  - [ ] blogposts (1 week)

#### Before Release

- [ ] Restrict registering

### v1.1 (Blogpost Update)

- [ ] Removing repository pattern
- [ ] Complete blogpost module

### v1.2 (Interaction Update)

- [ ] Karma system for posts and comments
- [ ] Filtering for spam comments and forbidden words (or regex)

### v1.3 (Email Update)

- [ ] Local email STMP server setup
- [ ] Email verification

### v1.4 (Blog Series Update)

- [ ] Blogpost series

### v1.5 (Error Handling Update)

- [ ] Comprehensive error handling
  - [ ] postgres errors
  - [ ] diesel errors
  - [ ] redis errors
  - [ ] Validation errors

### v2 (Guest Author Update)

- [ ] External authors
- [ ] Ownership based auth for blogposts

### v2.1 (Versioning Update)

- [ ] Versioning system for blogposts

### v2.2 (Multi-Author Update)

- [ ] Multiple author support for blogposts

### v2.3 (Editor Update)

- [ ] Permission based role system
- [ ] Editor role

### v2.4 (User Email Update)

- [ ] Email creation module for admins

### Undecided

- [ ] 2FA with mail
