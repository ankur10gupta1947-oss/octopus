use std::{fs,path::Path};
const DATA:[(&str,&str,&str,&str);9]=[
("Building My Dream Rust Workspace","Technology","PixelForge","💻"),
("The Ultimate Mountain Adventure","Travel","Trailhead","🏔️"),
("Late Night Lo-Fi Beats","Music","Soundwave","🎧"),
("100 Days in a Survival Game","Gaming","GameZone","🎮"),
("Exploring Tokyo After Dark","Travel","Wanderer","🌃"),
("The Future of AI and Open Source","Technology","Future Lab","🤖"),
("Perfect Morning Coffee","Lifestyle","Daylight","☕"),
("Make Electronic Music from Scratch","Music","Soundwave","🎹"),
("Creative Filmmaking Techniques","Film","Nova Films","🎬"),
];
fn write(p:&str,s:String){let p=Path::new(p);fs::create_dir_all(p.parent().unwrap()).unwrap();fs::write(p,s).unwrap();}
fn page(title:&str,body:&str)->String{
format!(r##"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title} · StreamVerse</title><link rel="stylesheet" href="/octopus/styles.css"></head><body><header><a class="logo" href="/octopus/">▶ Stream<span>Verse</span></a><a class="upload" href="/octopus/studio/">＋ Upload</a></header><div class="layout"><nav><a href="/octopus/">⌂ Home</a><a href="/octopus/category/Technology/">⌘ Technology</a><a href="/octopus/category/Music/">♫ Music</a><a href="/octopus/category/Gaming/">▣ Gaming</a><a href="/octopus/channels/">◉ Channels</a><a href="/octopus/studio/">▤ Creator Studio</a><p>🦀 Rust-generated open-source website</p></nav><main>{body}</main></div></body></html>"##)
}
fn cards(cat:&str)->String{
let mut out=String::from("<div class=\"grid\">");
for (i,(title,category,creator,emoji)) in DATA.iter().enumerate(){
if cat!="All" && cat!=*category {continue}
out.push_str(&format!(r##"<a class="card" href="/octopus/watch/{}/"><div class="thumb" style="background:linear-gradient(120deg,#3b2b57,#9f486c)">{emoji}<small>DEMO</small></div><h3>{title}</h3><p>{creator} · {category} · Sample video</p></a>"##,i+1));
}
out.push_str("</div>");out
}
fn main(){
let src=include_str!("../main.rs");let css=src.split("const CSS:&str=r#\"").nth(1).unwrap().split("\"#;").next().unwrap();
write("site/styles.css",format!("{css}\n.logo{{font-size:24px;font-weight:800}}.logo span{{color:#f43d68}}.upload{{margin-left:auto;background:#f43d68;padding:11px;border-radius:10px}}.thumb{{font-size:60px}}.thumb small{{position:absolute;bottom:8px;right:8px;background:#111;padding:5px;font-size:11px}}"));
for cat in ["All","Technology","Music","Gaming","Travel","Lifestyle","Film"]{
let mut body=String::new();
if cat=="All"{body.push_str("<section class=\"hero\"><div><div class=\"eyebrow\">✦ YOUR PLATFORM. YOUR RULES.</div><h1>Videos without<br>the limits.</h1><p>An open-source video platform built with Rust. Browse seeded videos before connecting storage.</p><a class=\"button hot\" href=\"/octopus/studio/\">Creator Studio ↗</a></div><div class=\"hero-icon\">▶</div></section>");}
body.push_str(&format!("<h2>{cat} Videos <span class=\"demo\">• MOCK DATA</span></h2><div class=\"chips\">"));
for c in ["All","Technology","Music","Gaming","Travel","Lifestyle","Film"]{
let path=if c=="All"{"/octopus/".to_string()}else{format!("/octopus/category/{c}/")};
body.push_str(&format!("<a class=\"chip\" href=\"{path}\">{c}</a>"));
}
body.push_str("</div>");body.push_str(&cards(cat));
let path=if cat=="All"{"site/index.html".to_string()}else{format!("site/category/{cat}/index.html")};
write(&path,page(cat,&body));
}
for (i,(title,category,creator,emoji)) in DATA.iter().enumerate(){
let body=format!("<a href=\"/octopus/\">← Back</a><div class=\"watch\"><div class=\"playicon\">{emoji}</div><p>Mock video: no media file connected</p></div><h1>{title}</h1><section class=\"panel\"><h3>{creator}</h3><p>{category} • Sample data only</p><p>Video playback, comments, likes and subscriptions will be enabled after backend services are connected.</p></section><h2>Other videos</h2>{}",cards("All"));
write(&format!("site/watch/{}/index.html",i+1),page(title,&body));
}
write("site/channels/index.html",page("Channels","<h1>Channels</h1><p>Channel list is seeded mock data. Real creator accounts will be added later.</p>"));
write("site/studio/index.html",page("Creator Studio","<h1>Creator Studio</h1><section class=\"panel\"><h2>9 sample videos • 0 uploads</h2><p>Video storage is not connected. Upload is disabled.</p><button disabled>Upload unavailable</button></section>"));
write("site/.nojekyll",String::new());
println!("Generated {} Rust-powered mock video pages",DATA.len());
}
