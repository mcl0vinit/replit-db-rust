use reqwest;
use std::env;

mod db_error;

use db_error::DBError;

/// Sets a key value pair in the KV database
pub async fn set(key: String, val: String) -> Result<bool, DBError> {
    let db_url = env::var("REPLIT_DB_URL").expect("REPLIT_DB_URL not set");
    let client = reqwest::Client::new();
    match client.post(db_url).form(&[(key, val)]).send().await {
        Ok(_) => return Ok(true),
        Err(e) => return Err(e.into()),
    };
}

/// Lists keys stored in the KV database.
pub async fn list(prefix: Option<&str>) -> Result<Vec<String>, DBError> {
    let db_url = env::var("REPLIT_DB_URL").expect("REPLIT_DB_URL not set");
    let req_url = match prefix {
        Some(prefix_val) => {
            format!("{}?prefix={}", db_url, prefix_val)
        }
        None => format!("{}?prefix=", db_url),
    };
    let client = reqwest::Client::new();
    let res = match client.get(req_url).send().await {
        Ok(val) => val,
        Err(e) => return Err(e.into()),
    };
    let res_text = match res.text().await {
        Ok(val) => val,
        Err(_) => return Err(DBError::parse_text_error()),
    };
	if res_text.len() > 1 {
	    let res_vec = res_text.split("\n").map(|x| x.to_string()).collect();
	    return Ok(res_vec);
	}
	return Ok(Vec::new())
}

/// Gets value for specific key from KV database.
pub async fn get(key: &str) -> Result<String, DBError> {
    let db_url = env::var("REPLIT_DB_URL").expect("REPLIT_DB_URL not set");
    let req_url = format!("{}/{}", db_url, key);
    let client = reqwest::Client::new();
    let res = match client.get(req_url).send().await {
        Ok(val) => val,
        Err(e) => return Err(e.into()),
    };
    if res.status() == 404 {
        return Err(DBError::not_found_error());
    }
    let res_text = match res.text().await {
        Ok(val) => val,
        Err(_) => return Err(DBError::parse_text_error()),
    };
    return Ok(res_text);
}

/// Deletes key value pair from KV database. 
pub async fn delete(key: &str) -> Result<bool, DBError> {
    let db_url = env::var("REPLIT_DB_URL").expect("REPLIT_DB_URL not set");
    let req_url = format!("{}/{}", db_url, key);
    let client = reqwest::Client::new();
    let res = match client.delete(req_url).send().await {
        Ok(val) => val,
        Err(e) => return Err(e.into()),
    };
    if res.status() == 404 {
        return Err(DBError::not_found_error());
    }
    return Ok(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_normal_flow() {
        let set_res = set(String::from("test"), String::from("testing"))
            .await
            .expect("Error occurred during set execution");
        assert!(set_res);
        let list_res = list(Some("test"))
            .await
            .expect("Error occurred during list execution");
        assert_eq!(list_res, vec![String::from("test")]);
        let item = list_res[0].clone();
        let get_res = get(&item)
            .await
            .expect("Error occurred during get execution");
        assert_eq!(get_res, String::from("testing"));
        let delete_res = delete(&item)
            .await
            .expect("Error occurred during delete execution");
        assert_eq!(delete_res, true);
    }

    #[tokio::test]
    async fn test_empty_list() {
        let list_res = list(Some("empty_list"))
            .await
            .expect("Error occurred during list execution");
        let res_vec: Vec<String> = Vec::new();
        assert_eq!(list_res, res_vec);
    }

    #[tokio::test]
    async fn test_empty_list_no_prefix() {
        let set_res = set(
            String::from("empty_list_no_prefix"),
            String::from("testing"),
        )
        .await
        .expect("Error occurred during set execution");
        assert!(set_res);
        let list_res = list(None)
            .await
            .expect("Error occurred during list execution");
        assert_eq!(list_res, vec![String::from("empty_list_no_prefix")]);
		let item = list_res[0].clone();
        let delete_res = delete(&item)
            .await
            .expect("Error occurred during delete execution");
        assert_eq!(delete_res, true);
    }

    #[tokio::test]
    async fn test_empty_get() {
        let item = String::from("empty_get");
        let get_res = get(&item).await;
        if let Err(e) = get_res {
            let expected_err = String::from("Not Found");
            assert_eq!(e.to_string(), expected_err);
        }
    }

	#[tokio::test]
    async fn test_empty_delete() {
        let item = String::from("empty_get");
        let get_res = delete(&item).await;
        if let Err(e) = get_res {
            let expected_err = String::from("Not Found");
            assert_eq!(e.to_string(), expected_err);
        }
    }
}
