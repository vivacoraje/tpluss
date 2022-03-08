use super::AppState;

pub async fn get_deliverer_by_code(state: &AppState, code: &str) -> anyhow::Result<Option<String>> {
    let sql = r#"
        SELECT 
            d.pubuserdefnvc1 
        FROM 
            DI_Distribution_b AS db 
            JOIN DI_Distribution AS d ON db.idDistributionDTO=d.id 
        WHERE db.sourcevouchercode=(@P1)
    "#;
    let code = code.to_string();
    let mut conn = state.mssql_pool.get().await?;
    // let d = conn
    //     .query(sql, &[&code])
    //     .await?
    //     .into_row()
    //     .await?
    //     .unwrap()
    //     .get::<&str, _>(0)
    //     .and_then(|f| Some(String::from(f)));
    //     //.unwrap()
    //     //.to_string();
    let d = match conn
        .query(sql, &[&code])
        .await?
        .into_row()
        .await? {
            Some(r) => {
                r.get::<&str, _>(0).and_then(|f| Some(String::from(f)))
            },
            None => None,
        };
    
    Ok(d)
}
