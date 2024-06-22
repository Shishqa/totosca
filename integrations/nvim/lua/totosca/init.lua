local lsp_configs = require 'lspconfig.configs'
local util = require 'lspconfig.util'

local M = {}

M.setup = function(_opts)
  lsp_configs.totosca = {
    default_config = {
      cmd = { 'toto', 'ls' },
      filetypes = { 'yaml.tosca' },
      root_dir = util.root_pattern('TOSCA.meta'),
      single_file_support = true,
    },
    docs = {
      description = "your TOSCA companion",
    }
  }

  vim.api.nvim_create_autocmd(
    {
      "BufNewFile",
      "BufRead",
    },
    {
      pattern = "*.yaml,*.yml",
      callback = function()
        if vim.fn.search("^tosca_definitions_version:[^\n]*$", "nw") ~= 0 then
          local buf = vim.api.nvim_get_current_buf()
          vim.api.nvim_buf_set_option(buf, 'filetype', 'yaml.tosca')
        end
      end
    }
  )
end

return M
