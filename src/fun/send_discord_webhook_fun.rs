use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
// ===========================================

pub struct DiscordEmbed {
    pub title: String,
    pub description: String,
    pub color: u32,
    pub fields: Vec<DiscordEmbedField>,
    pub footer: Option<String>,
    pub timestamp: String,
}
// ===========================================

pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}
// ===========================================

pub async fn send_discord_webhook_fun(webhook_url: &str, embed: DiscordEmbed) -> Result<()> {
    let client = Client::new();

    let payload = json!({
        "embeds": [{
            "title": embed.title,
            "description": embed.description,
            "color": embed.color,
            "fields": embed.fields.iter().map(|f| json!({
                "name": f.name,
                "value": f.value,
                "inline": f.inline
            })).collect::<Vec<_>>(),
            "footer": embed.footer.map(|f| json!({"text": f})),
            "timestamp": embed.timestamp
        }]
    });

    let res = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .await
        .with_context(|| "Failed to send Discord webhook")?;

    if !res.status().is_success() {
        anyhow::bail!(
            "Discord webhook failed: {} {}",
            res.status(),
            res.status().canonical_reason().unwrap_or("")
        );
    }

    Ok(())
}
// ===========================================
