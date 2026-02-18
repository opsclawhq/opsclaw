# Editorial Workflow

This workflow operationalizes recurring engineering blog cadence per phase.

## Cadence Outputs

Each phase produces:

1. Engineering recap post
2. Design decisions post
3. Reliability/process post

## Draft Generation

```bash
scripts/content/generate-engineering-blog.sh --phase <n> --date <yyyy-mm-dd>
```

## Review Stages

1. Technical pass: verify architecture and evidence claims.
2. Editorial pass: tighten narrative and remove ambiguity.
3. Publish pass: add final links to project tracking fields.

## Traceability

For each post, record:

- linked plan docs
- linked PRs
- linked evidence docs
- publish status/location
