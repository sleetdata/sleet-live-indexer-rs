use crate::fun::send_discord_webhook_fun::{DiscordEmbed, DiscordEmbedField};
// ===========================================

pub fn format_delete_account_for_discord_fun(
    deleted_account_id: &str,
    beneficiary_id: &str,
    signer_id: &str,
    tx_hash: &str,
    block_height: u64,
    shard_id: u64,
    deposit: Option<&str>,
) -> DiscordEmbed {
    let mut fields = Vec::new();

    // Main transaction info
    fields.push(DiscordEmbedField {
        name: "🗑️ Deleted Account".to_string(),
        value: format!("`{}`", deleted_account_id),
        inline: false,
    });

    fields.push(DiscordEmbedField {
        name: "💰 Beneficiary".to_string(),
        value: format!("`{}`", beneficiary_id),
        inline: true,
    });

    fields.push(DiscordEmbedField {
        name: "✍️ Signed By".to_string(),
        value: format!("`{}`", signer_id),
        inline: true,
    });

    if let Some(deposit_amt) = deposit {
        let deposit_near = yocto_to_near(deposit_amt);
        fields.push(DiscordEmbedField {
            name: "💵 Remaining Balance".to_string(),
            value: format!("`{} NEAR`", deposit_near),
            inline: true,
        });
    }

    fields.push(DiscordEmbedField {
        name: "📄 Transaction".to_string(),
        value: format!(
            "[{}]({})",
            shorten_hash(tx_hash),
            format!("https://nearblocks.io/tx/{}", tx_hash)
        ),
        inline: false,
    });

    DiscordEmbed {
        title: "⚰️ Account Deleted".to_string(),
        description: format!("Account `{}` has been deleted", deleted_account_id),
        color: 0xED4245, // Discord red
        fields,
        footer: Some(format!("Block #{} | Shard #{}", block_height, shard_id)),
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
