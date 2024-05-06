use std::time::SystemTime;

use anyhow::Result;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tracing::info;

use crate::JwtPayload;

//use jsonwebtoken create a JWT using HS256 as algorithm
pub async fn jwt_auth(payload: JwtPayload, secert: String) -> Result<String> {
    // 获取当前时间的UNIX时间戳（以秒为单位）
    let current_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    // 将过期时间累加到当前时间
    let exp = current_timestamp + payload.exp;
    // 更新payload的exp字段
    let payload_with_exp = JwtPayload { exp, ..payload };
    //encode the JWT token using HS256 as algorithm
    let token = encode(
        &Header::default(),
        &payload_with_exp,
        &EncodingKey::from_secret(secert.as_bytes()),
    )?;
    Ok(token)
}

//use jsonwebtoken to verify the JWT token using HS256 as algorithm
pub async fn jwt_verify(token: String, secert: String) -> Result<JwtPayload> {
    //create a Validation struct
    let mut validation = Validation::new(Algorithm::HS256);
    // Setting audience
    validation.set_audience(&["api://2132141"]);
    validation.validate_exp = true;
    validation.validate_aud = true;
    //decode the JWT token using HS256 as algorithm
    let token_data = jsonwebtoken::decode::<JwtPayload>(
        &token,
        &DecodingKey::from_secret(secert.as_bytes()),
        &validation,
    );
    match token_data {
        Ok(_) => {
            info!("{:?}", token_data);
            Ok(token_data?.claims)
        }
        Err(e) => {
            info!("{:?}", e);
            return Err(anyhow::anyhow!("Token verification failed:error:{}", e.to_string()));
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jwt_verify() {
        let payload = JwtPayload {
            sub: "sub_test".to_string(),
            aud: "api://2132141".to_string(),
            exp: 3600,
        };
        let secert = "secret_test".to_string();
        let token = jwt_auth(payload, secert.clone()).await.unwrap();
        println!("generate:{:?}", token);
        let token_data = jwt_verify(token, secert).await.unwrap();
        assert_eq!(token_data.sub, "sub_test");
        assert_eq!(token_data.aud, "api://2132141");
    }
}
