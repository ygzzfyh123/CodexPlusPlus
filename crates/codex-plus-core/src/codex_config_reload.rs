use std::time::Duration;

use anyhow::Context;

const RELOAD_TIMEOUT: Duration = Duration::from_secs(8);

pub async fn reload_user_config_with_sub_agent_limit(
    debug_port: u16,
    max_threads: u8,
) -> anyhow::Result<()> {
    let targets = crate::cdp::list_targets(debug_port)
        .await
        .context("failed to list Codex CDP targets")?;
    let target = crate::cdp::pick_injectable_codex_page_target(&targets)?;
    let websocket_url = target
        .web_socket_debugger_url
        .as_deref()
        .context("Codex CDP target has no websocket URL")?;
    let max_threads = crate::settings::clamp_codex_sub_agent_max_threads(max_threads);
    let script = format!(
        r#"(async () => {{
  const urls = [
    ...Array.from(document.scripts || []).map((script) => script.src),
    ...Array.from(document.querySelectorAll("link[href]") || []).map((link) => link.href),
    ...performance.getEntriesByType("resource").map((entry) => entry.name),
  ].filter(Boolean);
  const moduleUrl = urls.find((url) =>
    url.includes("/assets/")
    && url.includes("vscode-api-")
    && url.split("?")[0].endsWith(".js")
  );
  if (!moduleUrl) throw new Error("Codex vscode-api asset is unavailable");
  const module = await import(moduleUrl);
  if (typeof module.n !== "function") throw new Error("Codex state API is unavailable");
  return await module.n("batch-write-config-value", {{
    params: {{
      hostId: "local",
      edits: [{{
        keyPath: "agents.max_threads",
        value: {max_threads},
        mergeStrategy: "upsert",
      }}],
      filePath: null,
      expectedVersion: null,
      reloadUserConfig: true,
    }},
  }});
}})()"#
    );
    tokio::time::timeout(
        RELOAD_TIMEOUT,
        crate::bridge::evaluate_script_with_await_promise(websocket_url, &script, true),
    )
    .await
    .context("Codex config hot reload timed out")?
    .context("Codex config hot reload failed")?;
    Ok(())
}
