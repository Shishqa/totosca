# TODO (v0.1)

> See: [todomd/todo.md](https://github.com/todomd/todo.md)

**Approx. deadline:** 28.02.2024

## Milestones

- [ ] Initial parser implementation #syntax #semantic
- [ ] Initial lsp server implementation #lsp
- [ ] Initial `toto` cli implementation #cli

## Open

- Full support of TOSCA 1.3 grammar schema #feature #syntax
- Full support of TOSCA 2.0 grammar schema #feature #syntax
- Declarative schema definitions, make parse method generic #improvement #syntax
- Support namespace indexes for name lookup #feature #semantic
- Support remote http file imports with local caching #feature #semantic
- Resolve TOSCA definition type #feature #semantic
- Support TOSCA inheritance #feature #semantic
- Support function evaluation #feature #semantic
- Go to definition for type and derived_from fields #feature #lsp
- Suggest type when filling type and derived_from #feature #lsp
- Initial clap-based implementation with mock commands #feature #cli
- Add `toto parse` subcommand, report errors with ariadne #feature #cli
- Add `toto lsp-server` subcommand, launching lsp #feature #cli
- Document grammar parsing decisions #docs
- Document usage #docs
- Document totosca motivation and use-cases #docs

## Done

- Introduce mechanism for supporting multiple TOSCA grammars and code reuse #feature #syntax
- Determine grammar from file #feature #syntax
- Initial LSP server based on https://github.com/rust-lang/rust-analyzer/tree/master/lib/lsp-server #feature #lsp
- Report parser errors as diagnostics #feature #lsp
- Support local file imports #feature #semantic
