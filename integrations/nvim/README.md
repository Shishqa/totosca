# totosca.nvim

## Getting started

1. Install `toto` CLI on your system
1. Clone github.com/Shishqa/totosca somewhere on your system (in the future this will be improved)
    ```bash
    git clone https://github.com/Shishqa/totosca.git ~/.local/share/totosca
    ```
1. Add `$REPO_PATH/integrations/nvim` to your `runtimepath`
    ```lua
    vim.opt.rtp:append("$REPO_PATH/integrations/nvim/")
    ```
1. Setup a plugin before setting up LSPs:
    ```lua
    require("totosca").setup()
    ```
1. Now `totosca` is available in `lspconfig.server_configurations`

    **Snippet to enable the language server:**
    ```lua
    require'lspconfig'.totosca.setup{}
    ```

    **Default values:**
    - `cmd` :
    ```lua
    { "toto", "ls" }
    ```
    - `filetypes` :
    ```lua
    { "yaml.tosca" }
    ```
    - `root_dir` :
    ```lua
    see source file
    ```
    - `single_file_support` :
    ```lua
    true
    ```

> The plugin also defines a filetype `yaml.tosca`, which is set, when yaml has `^tosca_definitions_version: ` pattern.
