# npm-deps

List dependencies that are ready to update.

```bash
$ cd <dir containing package.json>
$ npm-deps
┌───────────────────────────────────────────────────────────────────────┐
│ Type   Name                          Current version   Latest version │
╞═══════════════════════════════════════════════════════════════════════╡
│        axios                         0.24.0            0.26.1         │
│        react                         17.0.2            18.0.0         │
│        react-bootstrap               2.1.0             2.2.3          │
│        react-dom                     17.0.2            18.0.0         │
│ dev    @testing-library/jest-dom     5.16.1            5.16.4         │
│ dev    @testing-library/react        12.1.2            13.0.1         │
│ dev    @testing-library/user-event   13.5.0            14.1.0         │
│ dev    @types/jest                   27.4.0            27.4.1         │
│ dev    @types/node                   16.11.19          17.0.24        │
│ dev    @types/react                  17.0.38           18.0.5         │
│ dev    @types/react-dom              17.0.11           18.0.0         │
│ dev    eslint-config-prettier        8.3.0             8.5.0          │
│ dev    openapi-typescript            5.1.0             5.2.0          │
│ dev    prettier                      2.5.1             2.6.2          │
│ dev    react-scripts                 5.0.0             5.0.1          │
│ dev    typescript                    4.5.4             4.6.3          │
└───────────────────────────────────────────────────────────────────────┘
```

## Envs

- `RUST_LOG=debug` - verbose output

## Development

### Requirements

- libssl-dev
