# reducto

List files recursively in a directory along with their hash.

## Specs

For now:

- Hash every file, recursively
- Use SHA-256 for hashing
- Output format: `sha256:{base64 of hash}`

Later:

- Handle ignore / include lists
