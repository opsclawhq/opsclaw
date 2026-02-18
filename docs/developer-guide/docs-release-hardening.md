# Docs Release Hardening

This guide defines the required validation and review workflow before closing a phase gate.

## Validation Command

```bash
scripts/docs/validate-release-docs.sh
```

The validator checks:

- required docs/index files exist
- key cross-links are present in docs indexes
- user, developer, and blog tracks are all represented

## Release Checklist

1. Run `scripts/docs/validate-release-docs.sh`.
2. Run `scripts/content/generate-engineering-blog.sh --phase <n> --date <yyyy-mm-dd>`.
3. Ensure phase recap/design/process blog drafts are linked from `docs/blog/README.md`.
4. Confirm `.content/phase-<n>/` contains LinkedIn and X drafts.
5. Update phase traceability artifacts (`.planning` + project fields).
