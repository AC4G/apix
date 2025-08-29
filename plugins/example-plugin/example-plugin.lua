function create(project_name)
  ctx:log("Creating " .. project_name)

  ctx:create_file(project_name .. "/Cargo.toml", [[
    [package]
    name = "]] .. project_name .. [["
    version = "0.1.0"
  ]])

  return 0
end

function extend(args)
  return 0
end

function migrate(from_version)
  return 0
end

function help()
    return {
        usage = {
            "apix plugin example-plugin create <name>",
            "apix plugin example-plugin extend [args...]",
        },
        options = {
            { "-y, --yes", "Accept changes" }
        }
    }
end
