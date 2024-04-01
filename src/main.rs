use std::{fmt::DebugStruct, vec};
use xmltree::Element;
use std::fs::File;
use std::fs;
use dict::{ Dict, DictIface };
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Clone)]
struct Aeons_Blessings {
    name: String,
    DD: String,
    SubDD: String,
    ED: String,
    Support: String,
}

#[derive(Clone)]
struct Boss{
    name: String,
    weakness1: String,
    weakness2: String,
    weakness3: String,
    chosen: String,
}

#[derive(Clone)]
struct Character{
    name: String,
    name_id: String,
    DD_id: String,
    Sub_id: String,
    ED_id: String,
    Support_id: String,
    
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
   simulators("test.csv");
    // bless_me_aeons("hsr_Sparkle.xml");
    // bosses_research("hsr_Sparkle.xml");

}

fn selectors(path: &str) {
    let mut xml = String::new();
    let mut reader = csv::Reader::from_path(path).unwrap();
    for head in reader.headers().unwrap().iter().skip(1) {
        xml.push_str(&format!("<parameter id=\"{}\" shortName=\"{}\" type=\"{}\"/>\n",
        uuid::Uuid::new_v4().to_string(),
        head,
        "string"
    ));
        print!("{}", xml);
        xml.clear();
    }
}

fn simulators(path: &str) {
    let aeons_blessings = bless_me_aeons("hsr_Sparkle.xml");
    let bosses = bosses_research("hsr_Sparkle.xml");
    let mut characters: Vec<Character> = Vec::new();
    let mut path_to_class_rules = String::new();
    let mut bosses_weaknesses_rules = String::new();
    let log_file1 = File::create("result_characters.txt").unwrap();
    let log_file2 = File::create("result_rules.txt").unwrap();
    let mut output_characters = OpenOptions::new()
        .write(true)
        .append(true)
        .open("result_characters.txt")
        .unwrap();
    let mut output_rules = OpenOptions::new()
        .write(true)
        .append(true)
        .open("result_rules.txt")
        .unwrap();



    let mut xml = String::new();
    let mut reader = csv::Reader::from_path(path).unwrap();
    let headers = reader.headers().unwrap().clone();
    for result in reader.records() {
        let mut DD_pot_id = String::from("DD_pot_id");
        let mut Sub_pot_id = String::from("Sub_pot_id");
        let mut ED_pot_id = String::from("ED_pot_id");
        let mut Support_pot_id = String::from("Support_pot_id");
        let mut DD_id = String::from("DD_id");
        let mut Sub_id = String::from("Sub_id");
        let mut ED_id = String::from("ED_id");
        let mut Support_id = String::from("Support_id");
        let mut Element_bonus_id = String::from("Element_bonus_id");
        let mut Rank_id = String::from("Rank_id");
        let mut char_name = "name".to_string();
        let mut Name_id = String::from("Name_id");
        let mut SP_id = String::from("SP_id");
        
        let mut in_collection_id = String::from("in_collection_id");
        let record = result.unwrap();
        let mut My_Aeon = String::from("My_Aeon");
        let mut Element_id = String::from("Element_id");
        xml.push_str(&format!(
            "\n<class id=\"{}\" shortName=\"{}\">\n<parameters>\n",
            uuid::Uuid::new_v4().to_string(),
            record.get(0).unwrap()
        ));
        record.iter().zip(headers.iter()).skip(1).for_each(|(rec, head)| {
            let cur_id = uuid::Uuid::new_v4().to_string();
            xml.push_str(&format!(
                "<parameter id=\"{}\" shortName=\"{}\" defaultValue=\"{}\" type=\"{}\"/>\n",
                cur_id,
                head,
                rec,
                if rec.parse::<f64>().is_ok() || rec==String::new(){
                    "double"
                } else {
                    "string"
                }
            ));

            match head {
                "6. Main DD потенциал"      => DD_pot_id = cur_id,
                "7. Sub DD потенциал"       => Sub_pot_id = cur_id,
                "8. ED потенциал"           => ED_pot_id = cur_id,
                "9. Support потенциал"      => Support_pot_id = cur_id,
                "91. Main DD ранг"          => DD_id = cur_id,
                "92. Sub DD ранг"           => Sub_id = cur_id,
                "93. Effect Dealer ранг"    => ED_id = cur_id,
                "94. Support ранг"          => Support_id = cur_id,
                "95. Бонус стихии"          => Element_bonus_id=cur_id,
                "5. SP голод"               => SP_id=cur_id,
                "Ранг"                      => Rank_id = cur_id,
                "1. Имя"                    => {
                    char_name = rec.to_string();
                    Name_id=cur_id;
                },
                "3. Стихия"                 => Element_id = cur_id,
                "2. В коллекции"            => in_collection_id=cur_id,
                "4. Путь"                   => My_Aeon = rec.to_string(),
                _ => (),
            }
        });

        let mut tmp_character=Character{
            name: char_name.clone(),
            name_id: Name_id.clone(),
            DD_id: DD_id.clone(),
            Sub_id: Sub_id.clone(),
            ED_id: ED_id.clone(),
            Support_id: Support_id.clone(),
          

        };
        characters.push(tmp_character.clone());


        let classes1 = ["DD", "SubDD", "ED", "Support"];
        let classes = [DD_id.clone(),Sub_id.clone(),ED_id.clone(),Support_id.clone()];
        let subclasses=[DD_pot_id.clone(), Sub_pot_id.clone(), ED_pot_id.clone(), Support_pot_id.clone()];
        let mut wtf = [&aeons_blessings.get(&My_Aeon).unwrap().DD, &aeons_blessings.get(&My_Aeon).unwrap().SubDD,
        &aeons_blessings.get(&My_Aeon).unwrap().ED, &aeons_blessings.get(&My_Aeon).unwrap().Support];

        for i in 0..4{
            path_to_class_rules.push_str(&format!(
                "<rule id=\"{}\" shortName=\"{}\" relation=\"b6028006-e460-496e-a93e-5f6b9be0e36a\" resultId=\"rang:{}\" \
                initId=\"path:{};element:{};obtained:{};SP:{};potential:{}\"/>\n",
                uuid::Uuid::new_v4().to_string(),
                &format!("{} в {}", &char_name, classes1[i]),
                classes[i],
                wtf[i],
                Element_bonus_id,
                in_collection_id,
                SP_id,
                subclasses[i],
                
                
            ));
        }

        for b in &bosses{
            bosses_weaknesses_rules.push_str(&format!(
                // <rule id="{}" shortName="test" relation="f46e9ddd-e40f-4109-9dc8-86153aa396f5" resultId="d:{}" initId="b3:{};c:{};chosen:{};b2:{};b1:{}"/>

                "<rule id=\"{}\" shortName=\"{}\" relation=\"f46e9ddd-e40f-4109-9dc8-86153aa396f5\" resultId=\"d:{}\" initId=\"b3:{};c:{};chosen:{};b2:{};b1:{}\"/>\n",
                uuid::Uuid::new_v4().to_string(),
                &format!("{}-{}", b.name, char_name),
                Element_bonus_id,
                b.weakness3,
                Element_id,
                b.chosen,
                b.weakness2,
                b.weakness1,
                
                
            ));
        }

        xml.push_str("</parameters>\n<rules>\n");
        


        // xml.push_str(&format!(
        //     "<rule id=\"{}\" shortName=\"{}\" relation=\"08a58fba-7a38-4eef-ad89-0bf6934288ef\" resultId=\"e:{}\" initId=\"a:{};d:{};b:{};c:{}\"/>\n",
        //     uuid::Uuid::new_v4().to_string(),
        //     &format!("Ранг {}", char_name),
        //     Rank_id,
        //     DD_id,
        //     Sub_id,
        //     ED_id,
        //     Support_id
        // ));
        xml.push_str("</rules>\n<constraints/>\n<classes/>\n</class>");
        if let Err(e) = writeln!(output_characters, "{}", xml) {
            eprintln!("Couldn't write to file: {}", e);
        }
        println!("{}", xml);
        xml.clear();

        super_iterator(characters.clone());
        
    }
    

    if let Err(e) = writeln!(output_rules, "{}", path_to_class_rules) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("{}", path_to_class_rules);

    if let Err(e) = writeln!(output_rules, "{}", bosses_weaknesses_rules) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("{}", bosses_weaknesses_rules);

}

trait RemoveLast {
    fn remove_last(&self) -> &Self;
}

impl RemoveLast for str {
    fn remove_last(&self) -> &Self {
        self.strip_suffix(|_: char| true).unwrap_or(self)
    }
}


fn super_iterator(characters: Vec<Character>){
    let log_file3 = File::create("result_rules1.txt").unwrap();
    let mut output_rules = OpenOptions::new()
    .write(true)
    .append(true)
    .open("result_rules1.txt")
    .unwrap();

    let mut chars_declaration = String::from("var ");
    let mut chars_list = String::from("var list=[\n");
    let mut chars_input = String::new();
    let mut chars_init_DD=String::new();
    let mut chars_init_SubDD=String::new();
    let mut chars_init_ED=String::new();
    let mut chars_init_Support=String::new();
    let mut counter: u32 = 0;
    let relation_id=uuid::Uuid::new_v4().to_string();

    



    for c in characters{
        counter+=1;

        chars_input.push_str(&format!("{}:string;{}:double;", &format!("name{}",counter), &format!("rang{}",counter)));
        chars_init_DD.push_str(&format!("{}:{};{}:{};", &format!("name{}",counter), c.name_id, &format!("rang{}",counter),c.DD_id));
        chars_init_SubDD.push_str(&format!("{}:{};{}:{};", &format!("name{}",counter), c.name_id, &format!("rang{}",counter),c.Sub_id));
        chars_init_ED.push_str(&format!("{}:{};{}:{};", &format!("name{}",counter), c.name_id, &format!("rang{}",counter),c.ED_id));
        chars_init_Support.push_str(&format!("{}:{};{}:{};", &format!("name{}",counter), c.name_id, &format!("rang{}",counter),c.Support_id));

        chars_declaration.push_str(&format!("{},{},", &format!("name{}",counter), &format!("rang{}",counter)));
        chars_list.push_str(
            &format!(
                "{{ name: {}, rang: {}, id: {} }},\n",
                &format!("name{}",counter),
                &format!("rang{}",counter),
                counter,
            )
        )
    }

    chars_declaration=(&chars_declaration.remove_last()).to_string();

    chars_declaration.push_str(";\n");

    chars_list=(&chars_list.remove_last()).to_string();
    chars_list=(&chars_list.remove_last()).to_string();

    chars_list.push_str("];\n");


    let relation = &format!(
        "<relation id=\"{}\" shortName=\"RANG_ITERATOR\" inObj=\"{}\" relationType=\"prog\" outObj=\"rang:double;name:string\">\n",
        relation_id,(&chars_input.remove_last()).to_string());


    let DDrule=&format!(
        "<rule id=\"{}\" shortName=\"{}\" relation=\"{}\" resultId=\"rang:a37e36b9-f334-4bd0-8bd6-1bd8b7b30d64;name:50145a28-17ad-424b-9dfc-446f9e46a9fc\" initId=\"{}\"/>\n",
            uuid::Uuid::new_v4().to_string(), "DD_Iterator".to_string(), relation_id,(&chars_init_DD.remove_last()).to_string());
    let SubDDrule=&format!(
        "<rule id=\"{}\" shortName=\"{}\" relation=\"{}\" resultId=\"rang:b79cde9e-a5fe-45ce-b219-f99d9a723860;name:db8d7e84-529b-40b5-b511-2441aa181e37\" initId=\"{}\"/>\n",
            uuid::Uuid::new_v4().to_string(), "SubDD_Iterator".to_string(), relation_id,(&chars_init_SubDD.remove_last()).to_string());
    let EDrule=&format!(
        "<rule id=\"{}\" shortName=\"{}\" relation=\"{}\" resultId=\"rang:4b17ade7-fd99-4f52-ae63-fcaeee5e59a7;name:55e9fb79-9614-4b65-bb77-bf70824a3dd5\" initId=\"{}\"/>\n",
            uuid::Uuid::new_v4().to_string(), "ED_Iterator".to_string(), relation_id,(&chars_init_ED.remove_last()).to_string());
    let Supportrule=&format!(
        "<rule id=\"{}\" shortName=\"{}\" relation=\"{}\" resultId=\"rang:cdf8b4e7-b1d3-4b59-ae1c-5f2952cbe2a9;name:861739f8-b139-4cc9-a26f-69ba989961bd\" initId=\"{}\"/>\n",
            uuid::Uuid::new_v4().to_string(), "Support_Iterator".to_string(), relation_id,(&chars_init_Support.remove_last()).to_string());
    
    let mut rangRules=String::new();
    rangRules.push_str(&DDrule);
    rangRules.push_str(&SubDDrule);
    rangRules.push_str(&EDrule);
    rangRules.push_str(&Supportrule);

    let mut res = &format!(
    "{} \n {} \n {} \n
var tmp_name = \"Noone\"; 
var tmp_rang=0; 
var size = Object.keys(list).length; 
for (var i = 0; i &lt; size; i+=1) {{ 
    if (tmp_rang&lt;list[i].rang){{ 
        tmp_rang=list[i].rang; 
        tmp_name=list[i].name; 
    }} 
}}; 
var name = tmp_name; 
var rang = tmp_rang;

</relation>",
        relation,
        chars_declaration,
        chars_list

    );

    

    if let Err(e) = writeln!(output_rules, "{}", rangRules) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("{}", rangRules);
    if let Err(e) = writeln!(output_rules, "{}", res) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("{}", res);
    



// <relation id="7269adf8-7c90-4b5f-92d2-15903648c9ae" shortName="RANG_ITERATOR" inObj="bla_name:string;spa_name:string;arg_rang:double;spa_rang:double;bla_rang:double;arg_name:string" relationType="prog" outObj="rang:double;name:string">var tmp_name = "Noone";&#xd;

    /*&format!(
        "<rule id=\"{}\" shortName=\"{}\" relation=\"f46e9ddd-e40f-4109-9dc8-86153aa396f5\" resultId=\"d:{}\" initId=\"b3:{};c:{};b2:{};b1:{}\"/>\n",
        uuid::Uuid::new_v4().to_string(),
        &format!("{}-{}", b.name, char_name),
        Element_bonus_id,
        b.weakness3,
        Element_id,
        b.weakness2,
        b.weakness1,
        
        
    )    */



}

fn bosses_research(path: &str) -> Vec<Boss>{
    let data = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut xml_elements = Element::parse(data.as_bytes()).unwrap();
    let classes = &xml_elements.get_mut_child("class").expect("No.").get_mut_child("classes").expect("No.").children;
    let mut bosses_info = Element::new("");
    for c in classes.iter(){
        if c.as_element().expect("No.").attributes["shortName"]=="Босс"{
            bosses_info=c.as_element().expect("No.").clone();
            break;
        }
    }
    let mut bosses = Vec::<Boss>::new();
    // println!("{:#?}", bosses_info);
    for e in &bosses_info.get_child("classes").expect("No.").children{
        for b in &e.as_element().expect("No.").get_child("parameters"){
            let mut temp_boss=Boss{
                name: e.as_element().expect("No.").attributes["shortName"].clone(),
                weakness1: String::new(),
                weakness2: String::new(),
                weakness3: String::new(),
                chosen: String::new(),
            };
            for w in &b.children{
                match w.as_element().expect("No.").attributes["shortName"].as_str()  {
                    "Уязвимость1" => (temp_boss.weakness1=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "Уязвимость2" => (temp_boss.weakness2=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "Уязвимость3" => (temp_boss.weakness3=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "Выбран" => (temp_boss.chosen=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    _ => (),
                }
            }
            bosses.push(temp_boss);
            
        }
    }
    // for b in &bosses{
    //     println!("{:#?}", b.name);

    // }


    return bosses
    
}

fn bless_me_aeons(path: &str) -> Dict<Aeons_Blessings> {

    let data = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut xml_elements = Element::parse(data.as_bytes()).unwrap();
    let classes = &xml_elements.get_mut_child("class").expect("No.").get_mut_child("classes").expect("No.").children;
    let mut aeons_paths = Element::new("");
    for c in classes.iter(){
        if c.as_element().expect("No.").attributes["shortName"]=="Путь"{
            aeons_paths=c.as_element().expect("No.").clone();
            break;
        }
    }
    
    let mut aeons_blessings = Dict::<Aeons_Blessings>::new();
    for e in &aeons_paths.get_child("classes").expect("No.").children{
        for b in &e.as_element().expect("No.").get_child("parameters"){
            // println!("{}", e.as_element().expect("No.").attributes["shortName"]);
            let mut temp_blessing=Aeons_Blessings{
                name: e.as_element().expect("No.").attributes["shortName"].clone(),
                DD: String::new(),
                SubDD: String::new(),
                ED: String::new(),
                Support: String::new(),
            };
            for w in &b.children{
                match w.as_element().expect("No.").attributes["shortName"].as_str()  {
                    "1. DD+" => (temp_blessing.DD=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "2. Sub DD+" => (temp_blessing.SubDD=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "3. ED+" => (temp_blessing.ED=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "4. Support+" => (temp_blessing.Support=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    _ => (),
                }
            }
            aeons_blessings.add(e.as_element().expect("No.").attributes["shortName"].clone(), temp_blessing);
        }
    }
    
    // for o in &aeons_blessings {
    //     println!( "{} - {}, {}, {}, {}", o.key, o.val.DD, o.val.SubDD, o.val.ED, o.val.Support );
    // }

    return aeons_blessings

    

}