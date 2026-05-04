# StrictDoc Validation Pipeline

This document describes how StrictDoc requirements are validated in the CI/CD pipeline for link-collection-rust.

## Overview

StrictDoc requirements are automatically validated on every push and pull request to ensure:
- Requirements syntax is correct
- Requirement IDs are unique
- Parent-child relationships are valid
- HTML documentation can be generated

## Makefile Goals

All validation can be run locally using Make:

### Installation
```bash
make strictdoc-install    # Install StrictDoc (Python 3.9+)
```

### Development
```bash
make strictdoc-validate   # Check requirements syntax only
make strictdoc-build      # Generate HTML documentation
make strictdoc-view       # Open documentation in browser
make strictdoc-clean      # Clean build artifacts
make strictdoc            # Validate + Build (recommended)
```

### CI Integration
```bash
make all                  # Runs all checks including strictdoc-validate
```

## GitHub Actions Pipeline

### Workflow File
Location: `.github/workflows/pipeline.yml`

### Pipeline Steps

1. **Checkout** - Clone repository
2. **Setup Python 3.9** - Required for StrictDoc
3. **Setup Rust** - Compile and test Rust code
4. **Install StrictDoc** - `pip install strictdoc`
5. **Verify Makefile** - `make help` (confirms goal structure)
6. **Format Check** - `make format-check`
7. **Clippy Linting** - `make clippy`
8. **Unit Tests** - `make test`
9. **✓ StrictDoc Validation** - `make strictdoc-validate` (NEW)
   - Checks: Syntax, IDs, relationships
   - Fails if validation errors found
10. **Rust Docs** - `make doc`
11. **✓ StrictDoc Build** - `make strictdoc-build` (NEW)
    - Generates: `build/strictdoc/index.html`
12. **✓ Artifact Upload** - Upload to GitHub Actions (NEW)
    - Artifact: `strictdoc-export`
    - Retention: 30 days
    - Available for download in workflow run

## Requirements Files

### Main Specification
- **File**: `requirements.sdoc`
- **Format**: StrictDoc SDOC format
- **Content**:
  - 7 System Requirements (SYSREQ)
  - 19 High-Level Requirements (HLR)
  - 15 Low-Level Requirements (LLR)

### Status Mapping
- **File**: `backlog.md`
- **Maps**: Emoji statuses → Formal statuses
  - ✅ → DONE
  - 🚧 → IN_PROGRESS
  - 🚀 → READY_TO_START
  - ⏳ → DEFERRED
  - ❌ → BLOCKED

## Requirement Hierarchy

```
SYSREQ (System Requirements)
├── HLR (High-Level Requirements)
│   └── LLR (Low-Level Requirements)
│
Example:
SYSREQ-002: Create a CLI application
├── HLR-006-1: Use clap for commands
├── HLR-006-2: Add a logger
├── HLR-006-3: Add configuration system
├── HLR-006-4: Read config from file
│   ├── LLR-006-4-1: TOML format
│   ├── LLR-006-4-2: Read from home directory
│   ├── LLR-006-4-3: Read from .env.local
│   ├── LLR-006-4-4: Read from env variable
│   ├── LLR-006-4-5: Read from CLI arg
│   ├── LLR-006-4-6: database_file config
│   ├── LLR-006-4-7: log_level config
│   ├── LLR-006-4-8: template_file config
│   ├── LLR-006-4-9: hacker_news_url config
│   ├── LLR-006-4-10: Config struct creation
│   ├── LLR-006-4-11: Config merging logic
│   ├── LLR-006-4-12: Logger integration
│   ├── LLR-006-4-13: Database integration
│   ├── LLR-006-4-14: Template integration
│   └── LLR-006-4-15: HackerNews integration
└── HLR-006-5: List tags from database
```

## Validation Behavior

### Success Criteria
Pipeline passes when:
- ✓ StrictDoc syntax is valid
- ✓ All requirement IDs are unique
- ✓ Parent references are valid
- ✓ HTML export generates successfully
- ✓ Artifacts can be uploaded

### Failure Behavior
Pipeline fails and blocks merge if:
- ❌ Invalid SDOC syntax detected
- ❌ Duplicate requirement IDs
- ❌ Invalid parent requirement references
- ❌ Missing required fields
- ❌ Circular dependency in parent-child relationships

### Debugging Failed Builds

1. **Download artifacts** from GitHub Actions
2. **Run locally**: `make strictdoc-validate`
3. **Check output** for specific syntax errors
4. **Review `requirements.sdoc`** for issues
5. **Fix and re-push** to trigger pipeline

## Continuous Improvement

### When to Update Requirements
- New features: Add HLR/LLR entries
- Refactoring: Update related LLR descriptions
- Bug fixes: Track as DEFECT type (if needed)
- Major changes: Update SYSREQ and cascade down

### Maintaining Traceability
- Always update `ParentReq` fields
- Use consistent ID numbering
- Update `backlog.md` when status changes
- Keep descriptions clear and testable

## Artifact Access

After pipeline runs:
1. Go to GitHub Actions workflow run
2. Scroll to "Artifacts" section
3. Download `strictdoc-export`
4. Extract and open `index.html` in browser
5. Browse interactive requirement documentation

## Future Enhancements

- [ ] Add coverage percentage reporting
- [ ] Generate traceability matrix PDF
- [ ] Add requirement metrics dashboard
- [ ] Auto-generate test cases from LLR
- [ ] Integrate with project tracking tools
- [ ] Add version control for requirements
- [ ] Create release notes from requirements

## References

- [StrictDoc Documentation](https://strictdoc.readthedocs.io/)
- [SDOC Format Specification](https://strictdoc.readthedocs.io/01_user_guide/01_sdoc_format/index.html)
- [GitHub Actions Upload Artifact](https://github.com/actions/upload-artifact)
