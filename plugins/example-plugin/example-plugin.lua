local lfs = require("lfs")

function get_plugin_info()
    return {
        name = "example-plugin",
        version = {
            major = 1,
            minor = 0,
            patch = 0
        },
        monorepo_templates = {
            default = {
                projects = "services",
                packages = "libs"
            }
        },
        supports = {
            "create_monorepo"
        }
    }
end

function create_monorepo(dst_path, template)
    local status = 0
    local data_dir = io.data_dir

    print("Hellow from create_monorepo")

    return {
        name = "example-plugin",
        version = {
            major = 1,
            minor = 0,
            patch = 0
        },
        monorepo_templates = {
            default = {
                projects = "services",
                packages = "libs"
            }
        },
        supports = {
            "create_monorepo"
        }
    }
end
