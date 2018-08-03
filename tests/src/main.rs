struct Test {
    boop: Vec<String>
}

impl<'a> IntoIterator for &'a Test {
    type Item = String;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.boop.clone().into_iter()
    }
}

impl IntoIterator for Test {
    type Item = String;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.boop.clone().into_iter()
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropoo");
        for name in (&self.boop).into_iter() {
            println!("hep {}", name);
        }
    }
}
fn main() {
    let b = Test { boop: vec!["oo".to_string(), "dd".to_string()] };
    for gig in Result::Ok::<Option<Option<u8>>, std::io::Error>(None) {
        println!("fij {:?}", gig);
    }
    for gog in b {
        println!("fij {}", gog);
    }
    {
        println!("1");
        let mut a = Test { boop: vec!["aa".to_string(), "bb".to_string()] };
        println!("2");
        if true {
            a = Test { boop: vec!["cc".to_string(), "dd".to_string()] };
        }
        println!("3");
        for gag in a {
            println!("faj {}", gag);
        }
    }
    println!("end");
}
// use std::collections::HashMap;


// type Table = HashMap<String, Vec<String>>;

// fn show(table : &Table) {
//     for (artist, works) in table {
//         println!("works by {}:", artist);
//         for work in works {
//             println!("  - {}", work);
//         }
//     }
// }

// fn sort_works(table: &mut Table) {
//     for (_artist, works) in table {
//         works.sort();
//     }
// }

// fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
//     for elt in slice {
//         vec.push(*elt);
//     }
// }

// use std::error::Error;
// use std::io::{Write, stderr};

// static B : u32 = 1;

// fn print_error(err: & Error) {
//     let _ = writeln!(stderr(), "error: {}", err);
//     while let Some(cause) = err.cause() {
//         let _ = writeln!(stderr(), "caused by: {}", cause);
//         print_error(cause);
//     }
// }

// fn return_result() -> Result<u8, u8> {
//     return Err(5);
// }

// fn main() {
//     // let mut v = (136, 139);
//     // let m = &mut v;
//     // let m0 = &mut m.0;
//     match return_result() {
//         Err(x) => print_error(&Err(x)),
//         Ok(x) => println!("aa"),
//     }
//     println!("{}", match 54u32.checked_div(B) {
//         Some(x) => x,
//         None => 5,
//     });

//     // let mut wave = Vec::new();
//     // let head = vec![0.0, 1.0];
//     // let tail = [0.0, -1.0];

//     // extend(&mut wave, &head);
//     // extend(&mut wave, &tail);

//     // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

//     // let clone = wave.clone();
//     // extend(&mut wave, &clone);
//     // assert_eq!(wave, vec![
//     //            0.0, 1.0, 0.0, -1.0,
//     //            0.0, 1.0, 0.0, -1.0
//     // ]);
//     // let mut table = Table::new();
//     // table.insert("Gesualdo".to_string(),
//     //              vec![
//     //                 "many madrigals".to_string(),
//     //                 "Tenebrae Responsoria".to_string(),
//     //              ]);
//     // table.insert("Caravaggio".to_string(),
//     //              vec![
//     //                 "The Musicians".to_string(),
//     //                 "The Calling of St. Natthew".to_string(),
//     //              ]);
//     // table.insert("Cellini".to_string(),
//     //              vec![
//     //                 "Perseus with the head of Medusa".to_string(),
//     //                 "a salt cellar".to_string(),
//     //              ]);
//     // sort_works(&mut table);
//     // show(&table);
//     // assert_eq!(table["Gesualdo"][0], "many madrigals");

//     // let languages : Vec<String> = std::env::args().skip(1).collect();
//     // print_first_element_of_slice(&languages);
//     // let b = &languages[1..languages.len()];
//     // for l in b {
//     //     println!("{} is {} language", l,
//     //              if l.len() % 2 == 0 {
//     //                 "a functional"
//     //              } else {
//     //                  "an imperative"
//     //              });
//     // }
//     // println!(r"It was a bright, cold day in April, and \
//     //          there were four of us—\
//     //          more or less."
//     //         );

//     // let v = vec![
//     //              "liberté".to_string(),
//     //              "égalité".to_string(),
//     //              "fraternité".to_string(),
//     //             ];

//     // for mut s in v {
//     //     s.push('!');
//     //     println!("{}", s);
//     // }
// }

// // fn print_first_element_of_slice(slice : &[String]) {
// //     println!("first element: {}", slice[0]);
// // }
