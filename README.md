# npm-deps

List dependencies that are ready to update.

```bash
$ cd <dir containing package.json>
$ npm-deps
┌─────────────────────────────────────────────────────────────────────┐
│ Name                               Current version   Latest version │
╞═════════════════════════════════════════════════════════════════════╡
│ @babel/cli                         7.16.8            7.17.6         │
│ @babel/core                        7.16.12           7.17.9         │
│ @types/jest                        27.4.0            27.4.1         │
│ @types/node                        16.11.21          17.0.23        │
│ @typescript-eslint/eslint-plugin   5.10.1            5.19.0         │
│ @typescript-eslint/parser          5.10.1            5.19.0         │
│ eslint                             8.8.0             8.13.0         │
│ eslint-config-prettier             8.3.0             8.5.0          │
│ jest                               27.4.7            27.5.1         │
│ prettier                           2.5.1             2.6.2          │
│ typescript                         4.5.5             4.6.3          │
└─────────────────────────────────────────────────────────────────────┘
```

## Envs

- `RUST_LOG=debug` - verbose output

## Development

### Requirements

- libssl-dev
