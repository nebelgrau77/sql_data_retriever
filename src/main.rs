extern crate cursive;
extern crate rusqlite;

use cursive::traits::*;
use cursive::Cursive;
use cursive::views::{Dialog,
                    //TextView,
                    EditView,
                    ListView,};

use rusqlite::{Connection,
               Result,
               params,};

#[derive(Debug)]
struct Episode {
    season_number: u8,
    episode_number: u8,
    episode_title: String,
}

struct Keys {
    ses_num: u8,
    ep_num: u8,
}

const DATABASE: &str = "data/bosch.db";    

fn main() {
    
    let mut app = cursive::default();

    input_step(&mut app);

    app.run();

}

fn input_step(app: &mut Cursive) {
    
    // get user input: season and episode number
    
    app.add_layer(
        Dialog::new()
        .title("bosch episodes")
        .content(ListView::new()
            .child("season:", EditView::new().with_name("season")
            )
            .child("episode:", EditView::new().with_name("episode")
            ),
        )  
        .button("submit", |s| {
            let key_a = s
                        .call_on_name(
                            "season",
                            |t: &mut EditView| t.get_content()
                        ).unwrap();
            
            let key_a: u8 = key_a.trim().parse().expect("something didn't work");

            let key_b = s
                        .call_on_name(
                            "episode",
                            |t: &mut EditView| t.get_content()
                        ).unwrap();

            let key_b: u8 = key_b.trim().parse().expect("something didn't work");

            let keys = Keys {
                ses_num: key_a,
                ep_num: key_b,
            };
            result_step(s, &keys)
        })
    );
}

fn result_step(app: &mut Cursive, keys: &Keys) {

    // display the title retrieved from the database with the keys supplied by the user

    let mut text = format!("couldn't find anything.");

    let title = get_title(keys).unwrap();

    if title.len() > 0 {

        text = format!("episode title: '{}'", title);
    
    }

    app.add_layer(Dialog::info(text).button("quit", |s| s.quit()));
}


fn get_title(keys: &Keys) -> Result<String>{

    let conn = Connection::open(DATABASE)?;

    let mut title: String = "".to_string();

    let query = format!("SELECT title FROM bosch WHERE season = {} AND episode = {}", keys.ses_num, keys.ep_num);

    let mut command = conn.prepare(&query)?;
    let title_iter = command.query_map(params![], |row| {
        Ok(Episode {
            season_number: keys.ses_num,
            episode_number: keys.ep_num,
            episode_title: row.get(0)?,
        })
    })?;
    

    for item in title_iter {        
        
        title = item.unwrap().episode_title;

    }

    Ok(title.to_string())

}    