extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Template {
    _id: String,
    name: String,
    owner: String,
    paths: String,
    created_at: String,
    has_contents: bool,
    __v: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://protternio.herokuapp.com/templates")?;
    let templates_str = resp.text()?;
    let templates_json: Vec<Template> = serde_json::from_str(templates_str.as_str())?;

    for temp in templates_json.iter() {
        println!("template name: {}", temp.name);
        println!("template owner: {}", temp.owner);
        println!("template has contents? {}\n", temp.has_contents);
    }


    Ok(())
}
