use std::{fmt::DebugStruct, vec};
use xmltree::Element;
use std::fs::File;
use std::fs;
use dict::{ Dict, DictIface };
use std::fs::OpenOptions;
use std::io::prelude::*;

struct Aeons_Blessings {
    name: String,
    DD: String,
    SubDD: String,
    ED: String,
    Support: String,
}

struct Boss{
    name: String,
    weakness1: String,
    weakness2: String,
    weakness3: String,
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
        let mut char_name = "name";
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

            // if head == "5. Main DD потенциал" {
            //     // let s_slice: &str = &cur_id[..];
            //     DD_pot_id = cur_id
            // }
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
                "Ранг"                      => Rank_id = cur_id,
                "1. Имя"                    => char_name = rec,
                "3. Стихия"                 => Element_id = cur_id,
                "2. В коллекции"            => in_collection_id=cur_id,
                "4. Путь"                   => My_Aeon = rec.to_string(),
                _ => (),
            }
        });

        let classes1 = ["DD", "SubDD", "ED", "Support"];
        let classes = [DD_id.clone(),Sub_id.clone(),ED_id.clone(),Support_id.clone()];
        let subclasses=[DD_pot_id.clone(), Sub_pot_id.clone(), ED_pot_id.clone(), Support_pot_id.clone()];
        // println!("{:#?}", My_Aeon);
        let mut wtf = [&aeons_blessings.get(&My_Aeon).unwrap().DD, &aeons_blessings.get(&My_Aeon).unwrap().SubDD,
        &aeons_blessings.get(&My_Aeon).unwrap().ED, &aeons_blessings.get(&My_Aeon).unwrap().Support];

        for i in 0..4{

            // print_type_of(&aeons_blessings.get(&My_Aeon).unwrap().DD);

            

            // println!("{:#?}", &aeons_blessings.get(&My_Aeon).unwrap().DD);
            
            path_to_class_rules.push_str(&format!(
                "<rule id=\"{}\" shortName=\"{}\" relation=\"c55e959e-d94b-4121-802f-08a1f17ba6dc\" resultId=\"c:{}\" initId=\"a:{};b:{}\"/>\n",
                uuid::Uuid::new_v4().to_string(),
                &format!("{}: {} в {}", char_name, &My_Aeon, classes1[i]),
                classes[i],
                wtf[i],
                // match i {
                //     0 => aeons_blessings.get(&My_Aeon).unwrap().DD,
                //     1 => aeons_blessings.get(&My_Aeon).unwrap().SubDD,
                //     2 => aeons_blessings.get(&My_Aeon).unwrap().ED,
                //     3 => aeons_blessings.get(&My_Aeon).unwrap().Support,
                //     _ => (),
                // },
                subclasses[i],
                
                
            ));
        }

        for b in &bosses{
            bosses_weaknesses_rules.push_str(&format!(
                "<rule id=\"{}\" shortName=\"{}\" relation=\"f46e9ddd-e40f-4109-9dc8-86153aa396f5\" resultId=\"d:{}\" initId=\"b3:{};c:{};b2:{};b1:{}\"/>\n",
                uuid::Uuid::new_v4().to_string(),
                &format!("{}-{}", b.name, char_name),
                Element_bonus_id,
                b.weakness3,
                Element_id,
                b.weakness2,
                b.weakness1,
                
                
            ));
        }

        xml.push_str("</parameters>\n<rules>\n");
        xml.push_str(&format!(
            "<rule id=\"{}\" shortName=\"{}\" relation=\"08a58fba-7a38-4eef-ad89-0bf6934288ef\" resultId=\"e:{}\" initId=\"a:{};d:{};b:{};c:{}\"/>\n",
            uuid::Uuid::new_v4().to_string(),
            &format!("Ранг {}", char_name),
            Rank_id,
            DD_id,
            Sub_id,
            ED_id,
            Support_id
        ));
        xml.push_str("</rules>\n<constraints/>\n<classes/>\n</class>");
        // xml.push_str("</parameters>\n<rules/>\n<constraints/>\n<classes/>\n</class>");
        if let Err(e) = writeln!(output_characters, "{}", xml) {
            eprintln!("Couldn't write to file: {}", e);
        }
        println!("{}", xml);
        
        xml.clear();


        
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
            };
            for w in &b.children{
                match w.as_element().expect("No.").attributes["shortName"].as_str()  {
                    "Уязвимость1" => (temp_boss.weakness1=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "Уязвимость2" => (temp_boss.weakness2=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
                    "Уязвимость3" => (temp_boss.weakness3=w.as_element().expect("No.").attributes["id"].parse().unwrap()),
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