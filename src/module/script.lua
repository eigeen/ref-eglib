local privilege_key = eglib.__get_privileged_key()
if privilege_key == nil or privilege_key == "" then
    error("Eglib core script failed to get privileged key.")
end

local function parse_permissions(perm_str)
    local permissions = {}
    local mapping = {
        r = "Read",
        w = "Write"
    }

    for i = 1, #perm_str do
        local c = perm_str:sub(i, i)
        if mapping[c] and not permissions[mapping[c]] then
            table.insert(permissions, mapping[c])
            permissions[mapping[c]] = true -- avoid duplicates
        end
    end

    if #permissions == 0 then
        return "No Permissions"
    end

    return table.concat(permissions, " | ")
end

re.on_draw_ui(function()
    if not imgui.tree_node("Eglib") then
        return
    end

    local data = eglib.fs:get_granted_access()
    if data then
        if imgui.tree_node("Granted File System Access") then
            for service_name, states in pairs(data) do
                imgui.text(service_name)
                if states.acceptions then
                    imgui.text("- Accepted")
                    for _, state in ipairs(states.acceptions) do
                        imgui.text("  - " .. state.path)
                    end
                elseif states.rejections then
                    imgui.text("- Rejected")
                    for _, state in ipairs(states.rejections) do
                        imgui.text("  - " .. state.path)
                    end
                end
            end
            imgui.tree_pop()
        end
    end

    imgui.tree_pop()
end)
