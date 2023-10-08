use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct SuperHero {
    name: String,
    age: u32,
    is_tsuyoi: bool,
}

fn main() {

    let hero1 = SuperHero {
        name: "hero1".to_string(),
        age: 25,
        is_tsuyoi: true,
    };

    let hero1_json_str = serde_json::to_string(&hero1).unwrap();
    println!("Serialized Json = {}", hero1_json_str);


    let hero2_json_str = r#"
        {
            "name": "hero2",
            "age": 40,
            "is_tsuyoi": false
        }
    "#;

    let hero2: SuperHero = serde_json::from_str(hero2_json_str).unwrap();
    println!("Deserialized Json = {:?}", hero2);

    let hero3_yaml_str = r#"
        name: "hero3"  
        age: 30
        is_tsuyoi: true
    "#;

    let hero3: SuperHero = serde_yaml::from_str(hero3_yaml_str).unwrap();
    println!("Deserialized Yaml = {:?}", hero3);

    let hero1_yaml_str = serde_yaml::to_string(&hero1).unwrap();
    println!("Serialized Yaml = {}", hero1_yaml_str);

}
