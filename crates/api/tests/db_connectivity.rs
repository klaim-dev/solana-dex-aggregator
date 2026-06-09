use api::config::Config;
use api::infra::db::token_mint_repo::{get_token_mint, insert_token_mint};
use solana_sdk::pubkey::Pubkey;
use sqlx::{PgPool, Row};

#[tokio::test]
async fn db_connectivity_select_one() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let database_url = config.database_url;

    let pool = PgPool::connect(&database_url).await?;

    let row = sqlx::query("SELECT 1").fetch_one(&pool).await?;

    let value: i32 = row.get(0);
    assert_eq!(value, 1);
    Ok(())
}

#[tokio::test]
async fn insert_then_select_token_mint() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let pool = PgPool::connect(&config.database_url).await?;
    let pubkey = Pubkey::new_unique();
    let pubkey = pubkey.to_string();

    sqlx::query!("DELETE FROM token_mints WHERE pubkey = $1", pubkey)
        .execute(&pool)
        .await?;

    insert_token_mint(&pool, &pubkey, 9, Some(1000), 12345).await?;

    let row = get_token_mint(&pool, &pubkey).await?;
    assert_eq!(row.decimals, 9);
    assert_eq!(row.pubkey, pubkey);
    assert_eq!(row.slot_updated, 12345);
    assert_eq!(row.supply, Some(1000));
    Ok(())
}
