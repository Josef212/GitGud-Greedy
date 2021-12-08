use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = gitgud_greedy::init();
    cli.print_info();
    cli.match_subcommand();
    
    gitgud_greedy::test()?;
    
    // let mut cat_colors = HashMap::new();
    // cat_colors.insert(String::from("Blue"), vec!["Tiger", "Sammy"]);
    // cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);
    // 
    // for (color, catnames) in &cat_colors {
    //     conn.execute(
    //         "INSERT INTO cat_colors (name) values (?1)",
    //         &[&color.to_string()]
    //     )?;
    //     
    //     let last_id: String = conn.last_insert_rowid().to_string();
    //     
    //     for cat in catnames {
    //         conn.execute(
    //             "INSERT INTO cats (name, color_id) values (?1, ?2)",
    //             &[&cat.to_string(), &last_id]
    //         )?;
    //     }
    // }
    
    // let mut stmt = conn.prepare(
    //     "SELECT c.name, cc.name from cats c
    //         INNER JOIN cat_colors cc
    //         ON cc.id = c.color_id;",
    // )?;
    // 
    // let cats = stmt.query_map([], |row| {
    //     Ok(Cat {
    //         name: row.get(0)?,
    //         color: row.get(1)?,
    //     })
    // })?;
    
    Ok(())
}
