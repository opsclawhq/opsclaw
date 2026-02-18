# Build-in-Public Pipeline

Use the content pipeline script to generate slice-scoped social drafts, blog drafts, and a machine-readable manifest.

## Command

```bash
scripts/content/phase-delivery-pipeline.sh \
  --phase 5 \
  --slice 05-05 \
  --date 2026-02-18 \
  --requirements PUB-04,PUB-07 \
  --pr-url https://github.com/opsclawhq/opsclaw/pull/36 \
  --summary "Phase 5 content pipeline baseline"
```

## Generated Outputs

- `.content/phase-<phase>/<date>-phase-<phase>-<slice>-linkedin-draft.md`
- `.content/phase-<phase>/<date>-phase-<phase>-<slice>-x-thread.md`
- `docs/blog/<date>-phase-<phase>-<slice>-recap.md`
- `docs/blog/<date>-phase-<phase>-<slice>-design-decisions.md`
- `docs/blog/manifests/<date>-phase-<phase>-<slice>-manifest.json`

## Notes

- social drafts stay in `.content/` (local, gitignored)
- blog drafts and manifests are tracked in `docs/`
- reruns preserve existing drafts and refresh manifest output
