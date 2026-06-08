use crate::app::AccountService;
use crate::domain::SolanaAccount;
use crate::error::AppError;
use crate::state::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
}

impl From<CreateAccountRequest> for SolanaAccount {
    fn from(value: CreateAccountRequest) -> Self {
        Self {
            pubkey: value.pubkey,
            owner: value.owner,
            lamports: value.lamports,
        }
    }
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<impl IntoResponse, AppError> {
    let repo = state.account_repo.clone();
    let service = AccountService::new(repo);
    let account = payload.into();
    service.create_account(account).await?;

    Ok(StatusCode::CREATED)
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
}

impl From<SolanaAccount> for AccountResponse {
    fn from(value: SolanaAccount) -> Self {
        Self {
            pubkey: value.pubkey,
            owner: value.owner,
            lamports: value.lamports,
        }
    }
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(pubkey): Path<String>,
) -> Result<Json<AccountResponse>, AppError> {
    let repo = state.account_repo.clone();
    let service = AccountService::new(repo);
    let account = service.get_account(&pubkey).await?;
    let response: AccountResponse = account.into();
    Ok(Json(response))
}

pub async fn list_account(
    State(state): State<AppState>,
) -> Result<Json<Vec<AccountResponse>>, AppError> {
    let repo = state.account_repo.clone();
    let service = AccountService::new(repo);
    let vec_account = service.list().await?;
    let list = vec_account
        .into_iter()
        .map(AccountResponse::from)
        .collect::<Vec<AccountResponse>>();
    Ok(Json(list))
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use solana_sdk::pubkey::Pubkey;
    use std::{sync::Arc, time::Instant};
    use tower::ServiceExt;

    use crate::{
        app, config::Config,
        infra::{http::metrics::metrics_handle, repo::in_memory::InMemoryAccountRepo},
    };

    fn test_state() -> crate::state::AppState {
        crate::state::AppState {
            config: Arc::new(Config {
                solana_rpc_url: "a".to_string(),
                database_url: "a".to_string(),
                jwt_secret: "a".to_string(),
            }),
            account_repo: Arc::new(InMemoryAccountRepo::new()),
            started_at: Instant::now(),
            metrics_handle: metrics_handle(),
        }
    }

    fn create_request(pubkey: &str, owner: &str, lamports: u64) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from(format!(
                r#"{{"pubkey":"{pubkey}","owner":"{owner}","lamports":{lamports}}}"#
            )))
            .unwrap()
    }

    #[tokio::test]
    async fn create_then_get_returns_account() {
        let app = app(test_state());
        let pubkey = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes";

        let create_req = create_request(pubkey, pubkey, 300);
        let create_resp = app.clone().oneshot(create_req).await.unwrap();
        assert_eq!(create_resp.status(), StatusCode::CREATED);

        let get_req = Request::builder()
            .method("GET")
            .uri(format!("/accounts/{pubkey}"))
            .body(Body::empty())
            .unwrap();
        let get_resp = app.clone().oneshot(get_req).await.unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn missing_field_returns_422() {
        let request = Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes","lamports":300}"#,
            ))
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY)
    }

    #[tokio::test]
    async fn get_missing_returns_404() {
        let pubkey = Pubkey::new_unique().to_string();
        let request = Request::builder()
            .method("GET")
            .uri(format!("/accounts/{pubkey}"))
            .body(Body::empty())
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn list_returns_all_created() {
        let app = app(test_state());
        let owner = Pubkey::new_unique().to_string();
        let accounts = [
            (Pubkey::new_unique().to_string(), 100),
            (Pubkey::new_unique().to_string(), 200),
            (Pubkey::new_unique().to_string(), 300),
        ];

        for (pubkey, lamports) in &accounts {
            let response = app
                .clone()
                .oneshot(create_request(pubkey, &owner, *lamports))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        let request = Request::builder()
            .method("GET")
            .uri("/accounts")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();

        for (pubkey, lamports) in &accounts {
            assert!(body.contains(pubkey));
            assert!(body.contains(&format!(r#""lamports":{lamports}"#)));
        }
    }

    #[tokio::test]
    async fn valid_request_returns_201() {
        let request = create_request(
            "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes",
            "11111111111111111111111111111111",
            300,
        );

        let response = app(test_state()).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED)
    }

    #[tokio::test]
    async fn create_invalid_pubkey_returns_400() {
        let request = Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"pubkey":"invalid","owner":"11111111111111111111111111111111","lamports":300}"#,
            ))
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST)
    }

    #[tokio::test]
    async fn create_duplicate_returns_409() {
        let state = test_state();
        let body = r#"{"pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes","owner":"11111111111111111111111111111111","lamports":300}"#;

        let request = Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = app(state.clone()).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let duplicate_request = Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = app(state).oneshot(duplicate_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT)
    }
}
