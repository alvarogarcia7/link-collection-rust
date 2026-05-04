# Link Collection Rust - Backlog

## Status Legend
- ✅ **DONE** — Implementation complete and tested
- 🚧 **IN_PROGRESS** — Currently being worked on
- 🚀 **READY_TO_START** — Requirements clear, ready for implementation
- ⏳ **DEFERRED** — Planned but not yet ready
- ❌ **BLOCKED** — Blocked on dependencies

## SYSREQ-001: Read existing database files

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| HLR-001-1 | Dump the contents into the console | DONE ✅ | Basic console output implemented |
| HLR-001-2 | Use a fixed template to print to console | DONE ✅ | Fixed template formatting in place |
| HLR-001-3 | Use any template to print to console | READY_TO_START 🚀 | Template engine integration needed |
| HLR-001-4 | Read the existing date formats | DONE ✅ | Multiple date format parsing implemented |
| HLR-001-5 | Add the optional field 'Link' | DONE ✅ | Link field support added to schema |

## SYSREQ-002: Create a CLI application

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| HLR-006-1 | Use clap for top command and multiple subcommands | IN_PROGRESS 🚧 | Clap integration in progress, see [clap docs](https://docs.rs/clap/latest/clap/) |
| HLR-006-2 | Add a logger | DONE ✅ | Logging system implemented with env_logger |
| HLR-006-3 | Add a configuration system | DONE ✅ | Configuration management in place |
| HLR-006-4 | Read the configuration from a file | DONE ✅ | Multi-source config file reading implemented |
| HLR-006-5 | List the tags from the database | READY_TO_START 🚀 | Tag listing feature ready to implement |

### Configuration Implementation Details (HLR-006-4)

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| LLR-006-4-1 | Configuration file in TOML format | DONE ✅ | TOML parser integrated |
| LLR-006-4-2 | Read config from user home directory | DONE ✅ | ~/.config/lc/config.toml supported |
| LLR-006-4-3 | Read config from .env.local file | DONE ✅ | .env.local file support added |
| LLR-006-4-4 | Read config from environment variable | DONE ✅ | LC_CONFIG_FILE env var supported |
| LLR-006-4-5 | Read config from command line argument | DONE ✅ | --config-file argument implemented |
| LLR-006-4-6 | Configuration contains database_file | DONE ✅ | Field defined and functional |
| LLR-006-4-7 | Configuration contains log_level | DONE ✅ | Field defined and functional |
| LLR-006-4-8 | Configuration contains template_file | DONE ✅ | Field defined and functional |
| LLR-006-4-9 | Configuration contains hacker_news_url | DONE ✅ | Field defined and functional |
| LLR-006-4-10 | Create struct to hold configuration values | DONE ✅ | Config struct implemented |
| LLR-006-4-11 | Implement configuration file merging | DONE ✅ | Priority-based merging implemented |
| LLR-006-4-12 | Include configuration in logging setup | DONE ✅ | Log level from config applied |
| LLR-006-4-13 | Include configuration in database connection | DONE ✅ | Database file from config used |
| LLR-006-4-14 | Include configuration in template rendering | DONE ✅ | Template file from config used |
| LLR-006-4-15 | Include configuration in HackerNews download | DONE ✅ | HN URL from config used |

## SYSREQ-003: Add a new record to the database

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| HLR-002-1 | Add record from the CLI | DONE ✅ | CLI record creation working |
| HLR-002-2 | Add record from a temporary file | IN_PROGRESS 🚧 | File upload mechanism in progress |
| HLR-002-3 | Print the record in the same order as it was provided | DONE ✅ | Field order preservation implemented |

## SYSREQ-004: Retrieve items from database

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| (HLR-003) | Retrieve items functionality | READY_TO_START 🚀 | HLR-003 family not yet defined |

## SYSREQ-005: Download items from HackerNews

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| HLR-004-1 | Create a new stubbed HTTP server | DONE ✅ | Mock HTTP server for testing |
| HLR-004-2 | Download the items from a URL | DONE ✅ | HTTP client download implemented |
| HLR-004-3 | Commit to git using the date of publishing | DONE ✅ | Git commit with historical dates |

## SYSREQ-006: Create a wrapper on recutils library

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| HLR-005-1 | Create it | READY_TO_START 🚀 | Wrapper abstraction ready to design |
| HLR-005-2 | Publish it to crates.io | DEFERRED ⏳ | Dependent on HLR-005-1 completion |

## SYSREQ-007: Replace Go Stubs with Rust Stubs

| ID | Requirement | Status | Notes |
|----|-------------|--------|-------|
| HLR-007-1 | Create Rust Stubs for existing Go Stubs | DONE ✅ | All Go stubs replaced with Rust equivalents |

## Status Summary

| Status | Count | Requirements |
|--------|-------|--------------|
| ✅ DONE | 25 | HLR-001-{1,2,4,5}, HLR-002-{1,3}, HLR-006-{2,3,4}, LLR-006-4-{1-15}, HLR-004-{1,2,3}, HLR-007-1 |
| 🚧 IN_PROGRESS | 2 | HLR-006-1, HLR-002-2 |
| 🚀 READY_TO_START | 3 | HLR-001-3, HLR-006-5, HLR-005-{1} |
| ⏳ DEFERRED | 1 | HLR-005-2 |
| ❌ BLOCKED | 0 | None |

## Mapping Notes

- **Emoji to Status Conversion:**
  - ✅ → DONE
  - 🚧 → IN_PROGRESS  
  - 🚀 → READY_TO_START
  - No emoji → DEFERRED (default) or BLOCKED (if documented)

- **Traceability:**
  - Each requirement has `ParentReq` field linking to its parent requirement
  - Backlog entries reference both requirement IDs and HLR/LLR codes
  - Requirements are organized hierarchically: SYSREQ → HLR → LLR

- **Configuration Priority (HLR-006-4):**
  1. Command-line argument (highest)
  2. Environment variable
  3. .env.local file
  4. User home directory config
  5. Default values (lowest)
