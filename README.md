# npm-deps

List dependencies that are ready to be updated.

```bash
$ cd <dir containing package.json>
$ npm-deps
┌───────────────────────────────────────────────────────────────────────────────────────┐
│ Upgrade   Type   Name                                Current version   Latest version │
╞═══════════════════════════════════════════════════════════════════════════════════════╡
│ PATCH            @skatteetaten/frontend-components   5.0.3             5.0.4          │
│ PATCH     dev    @testing-library/jest-dom           5.16.3            5.16.4         │
│ PATCH     dev    prettier                            2.6.0             2.6.2          │
│ PATCH     dev    react-scripts                       5.0.0             5.0.1          │
│ MINOR            react-router-dom                    6.2.2             6.3.0          │
│ MINOR            winston                             3.6.0             3.7.2          │
│ MINOR     dev    eslint-plugin-sonarjs               0.12.0            0.13.0         │
│ MINOR     dev    jest-junit                          13.0.0            13.1.0         │
│ MINOR     dev    typescript                          4.5.5             4.6.3          │
│ MAJOR            react                               17.0.2            18.0.0         │
│ MAJOR            react-dom                           17.0.2            18.0.0         │
│ MAJOR     dev    @testing-library/react              12.1.4            13.1.1         │
│ MAJOR     dev    @types/node                         16.11.26          17.0.25        │
│ MAJOR     dev    @types/react                        17.0.42           18.0.5         │
│ MAJOR     dev    @types/react-dom                    17.0.14           18.0.1         │
└───────────────────────────────────────────────────────────────────────────────────────┘
```

## Envs

- `RUST_LOG=debug` - verbose output

## Development

### Requirements

- libssl-dev
