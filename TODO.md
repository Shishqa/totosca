# TODO (v0.1)

> See: [todomd/todo.md](https://github.com/todomd/todo.md)

## Milestones

- [x] Initial parser implementation #syntax #semantic
- [x] Initial lsp server implementation #lsp
- [x] Initial `toto` cli implementation #cli

## Open

- Support remote http file imports with local caching #feature #semantic
- Support function evaluation #feature #semantic
- Document grammar parsing decisions #docs
- Document usage #docs
- Document totosca motivation and use-cases #docs

## Done

- Introduce mechanism for supporting multiple TOSCA grammars and code reuse #feature #syntax
- Determine grammar from file #feature #syntax
- Initial LSP server based on https://github.com/rust-lang/rust-analyzer/tree/master/lib/lsp-server #feature #lsp
- Report parser errors as diagnostics #feature #lsp
- Support local file imports #feature #semantic
- Declarative schema definitions, make parse method generic #improvement #syntax
- Support namespace indexes for name lookup #feature #semantic
- Resolve TOSCA definition type #feature #semantic
- Go to definition for type and derived_from fields #feature #lsp
- Add `toto parse` subcommand, report errors with ariadne #feature #cli
- Add `toto lsp-server` subcommand, launching lsp #feature #cli
- Initial clap-based implementation with mock commands #feature #cli
- Suggest type when filling type and derived_from #feature #lsp
- Support TOSCA inheritance #feature #semantic
- Full support of TOSCA 1.3 grammar schema #feature #syntax
- Full support of TOSCA 2.0 grammar schema #feature #syntax
