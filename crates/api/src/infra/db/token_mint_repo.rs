use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct TokenMintRow {
    pub pubkey: String,
    pub decimals: i32,
    pub supply: Option<i64>,
    pub slot_updated: i64,
}

pub async fn insert_token_mint(
    pool: &PgPool,
    pubkey: &str,
    decimals: i32,
    supply: Option<i64>,
    slot_updated: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO token_mints (pubkey, decimals, supply, slot_updated) VALUES ($1, $2, $3, $4)",
        pubkey,
        decimals,
        supply,
        slot_updated
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_token_mint(pool: &PgPool, pubkey: &str) -> Result<TokenMintRow, sqlx::Error> {
    sqlx::query_as!(
        TokenMintRow,
        "SELECT pubkey, decimals, supply, slot_updated FROM token_mints WHERE pubkey = $1",
        pubkey
    )
    .fetch_one(pool)
    .await
}
