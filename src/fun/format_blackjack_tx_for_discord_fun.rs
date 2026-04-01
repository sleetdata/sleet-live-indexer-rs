use crate::fun::filter_by_receiver_fun::ReceiverTransaction;
use crate::fun::send_discord_webhook_fun::{DiscordEmbed, DiscordEmbedField};
// ===========================================

pub fn format_blackjack_tx_for_discord_fun(
    tx: &ReceiverTransaction,
    block_height: u64,
    target_receiver: &str,
) -> DiscordEmbed {
    // Build action summary
    let mut action_summary = String::new();
    let mut fields = Vec::new();

    for (i, action) in tx.actions.iter().enumerate() {
        action_summary.push_str(&format!("**{}**", action.action_type));

        if let Some(method_name) = &action.method_name {
            action_summary.push_str(&format!(": `{}`", method_name));
        }

        if i < tx.actions.len() - 1 {
            action_summary.push_str(", ");
        }
    }

    // Add main fields
    fields.push(DiscordEmbedField {
        name: "🎮 Transaction".to_string(),
        value: format!(
            "**Hash:** [{}]({})\n**Signer:** `{}`",
            shorten_hash(&tx.tx_hash),
            format!("https://nearblocks.io/tx/{}", tx.tx_hash),
            tx.signer_id
        ),
        inline: false,
    });

    fields.push(DiscordEmbedField {
        name: "📋 Actions".to_string(),
        value: action_summary,
        inline: false,
    });

    // Add method-specific fields for FunctionCalls
    for action in &tx.actions {
        if action.action_type == "FunctionCall" {
            if let Some(method_name) = &action.method_name {
                fields.push(DiscordEmbedField {
                    name: "⚡ Method".to_string(),
                    value: format!("`{}`", method_name),
                    inline: true,
                });
            }
            if let Some(deposit) = &action.deposit {
                let deposit_near = yocto_to_near(deposit);
                fields.push(DiscordEmbedField {
                    name: "💰 Deposit".to_string(),
                    value: format!("`{} NEAR`", deposit_near),
                    inline: true,
                });
            }
            if let Some(gas) = action.gas {
                fields.push(DiscordEmbedField {
                    name: "⛽ Gas".to_string(),
                    value: format!("`{} Tgas`", gas / 1_000_000_000_000),
                    inline: true,
                });
            }
        }
    }

    // Add logs if present
    if !tx.logs.is_empty() {
        let logs_preview: Vec<String> = tx.logs.iter().take(3).map(|l| {
            if l.len() > 100 {
                format!("{}...", &l[..100])
            } else {
                l.clone()
            }
        }).collect();
        
        fields.push(DiscordEmbedField {
            name: "📝 Logs".to_string(),
            value: logs_preview.join("\n"),
            inline: false,
        });
    }

    DiscordEmbed {
        title: "🃏 Blackjack Transaction Detected".to_string(),
        description: format!("New transaction to `{}`", target_receiver),
        color: 0x5865F2, // Discord blurple
        fields,
        footer: Some(format!("Block #{} | Shard #{}", block_height, tx.shard_id)),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}
// ===========================================

fn shorten_hash(hash: &str) -> String {
    if hash.len() > 10 {
        format!("{}...{}", &hash[..4], &hash[hash.len() - 4..])
    } else {
        hash.to_string()
    }
}
// ===========================================

fn yocto_to_near(yocto: &str) -> String {
    yocto
        .parse::<u128>()
        .ok()
        .map(|v| {
            let near = v as f64 / 1e24;
            if near < 0.001 {
                format!("{:.6}", near)
            } else {
                format!("{:.4}", near)
            }
        })
        .unwrap_or_else(|| yocto.to_string())
}
// ===========================================
