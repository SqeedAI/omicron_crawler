-- Get the telescope module
local project_menu = require("plugins.local.project-menu")

-- Set up project-specific commands
project_menu.set_project_commands({
    { "run", "cargo run --bin omicron_crawler" },
    { "build", "cargo build --bin omicron_crawler" },
})

vim.opt.wildignore:append {
    '*.git/*',
    '*/target/*',
    '*/user_data/*',
    '*/drivers/*',
    '*.exe',
    '*.dll',
    '*.so',
    '*.o',
    '*.class'
}