use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug)]
struct BinanceProxyController {
    http_client: Client,
}

impl BinanceProxyController {
    fn new() -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// 转发请求到 Binance API（单请求，移除 path 参数拼接）。
    async fn get_binance_api_data(
        &self,
        path: String,
        all_params: HashMap<String, String>,
    ) -> impl Responder {
        // 构造基础URL
        let base_url = if path.starts_with("fapi") {
            "https://fapi.binance.com/"
        } else {
            "https://api.binance.com/"
        };
        let final_url = match Self::build_url_with_params(format!("{base_url}{path}"), &all_params) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("Failed to build URL: {err}");
                return HttpResponse::BadRequest().body("Invalid parameters provided.");
            }
        };

        println!("Requesting Binance API asynchronously: {}", final_url);

        // 提交异步任务
        match self.execute_request(final_url).await {
            Ok(response_body) => HttpResponse::Ok().body(response_body),
            Err(err) => {
                eprintln!("Error occurred while requesting Binance API: {}", err);
                HttpResponse::InternalServerError().body("An unexpected error occurred.")
            }
        }
    }

    /// 执行单个请求并返回结果
    async fn execute_request(&self, final_url: String) -> Result<String, String> {
        let response = self.http_client.get(&final_url).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text().await.map_err(|e| format!("Failed to read response body: {}", e))
                } else {
                    let status = resp.status();
                    let error_body = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    eprintln!("Request failed: Status={} Body={}", status, error_body);
                    Err(format!("Request failed: Status={} Body={}", status, error_body))
                }
            }
            Err(e) => {
                eprintln!("Error occurred while requesting Binance API: {}", e);
                Err(format!("Error occurred while requesting Binance API: {}", e))
            }
        }
    }


    /// 构建带查询参数的 URL
    fn build_url_with_params(
        base_url: String,
        params: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut url = url::Url::parse(&base_url)?;

        {
            // 修改 URL 的查询参数
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in params {
                if key != "path" {
                    query_pairs.append_pair(key, value);
                }
            }
        }

        // 返回 URL 的字符串形式
        Ok(url.to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let controller = web::Data::new(BinanceProxyController::new());

    HttpServer::new(move || {
        App::new()
            .app_data(controller.clone())
            .route("/proxy", web::to(|data: web::Data<BinanceProxyController>, query: web::Query<HashMap<String, String>>| async move {
                let path = query.get("path").unwrap_or(&"".to_string()).clone();
                data.get_binance_api_data(path, query.into_inner()).await
            }))
    })
        .bind("0.0.0.0:443")?
        .run()
        .await
}
