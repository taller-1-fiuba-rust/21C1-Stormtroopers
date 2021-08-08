pub fn filter_cmd(cmd: String) -> String {
    let cmd_aux = cmd.to_lowercase();
    let cmds: Vec<&str> = cmd_aux.split_ascii_whitespace().collect();

    if cmds[0] == "monitor"
        || cmds[0] == "pubsub"
        || cmds[0] == "suscribe"
        || cmds[0] == "publish"
        || cmds[0] == "unsuscribe"
        || cmds[0] == "clear"
        || cmds[0] == "exit"
    {
        format!("DISABLED COMMAND: {}", cmd)
    } else {
        cmd
    }
}
