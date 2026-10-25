use axum::{extract::{Path, Query}, http::{header, StatusCode}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Clone)]
struct Video { id: u32, title: &'static str, creator: &'static str, category: &'static str, duration: &'static str, views: &'static str, emoji: &'static str, colors: &'static str, description: &'static str }
const VIDEOS: &[Video] = &[
    Video{id:1,title:"Building My Dream Rust Workspace",creator:"PixelForge",category:"Technology",duration:"18:24",views:"28K",emoji:"💻",colors:"#173850,#669bb5",description:"A complete tour of a developer workspace built for creating."},
    Video{id:2,title:"The Ultimate Mountain Adventure",creator:"Trailhead",category:"Travel",duration:"21:35",views:"38K",emoji:"🏔️",colors:"#244657,#72aab6",description:"A scenic hike across dramatic mountains."},
    Video{id:3,title:"Late Night Lo-Fi Beats",creator:"Soundwave",category:"Music",duration:"1:24:08",views:"128K",emoji:"🎧",colors:"#40326a,#b45294",description:"A peaceful collection of sounds for late-night coding."},
    Video{id:4,title:"100 Days in a Survival Game",creator:"GameZone",category:"Gaming",duration:"32:15",views:"94K",emoji:"🎮",colors:"#26463e,#89a85a",description:"An epic gaming challenge from start to finish."},
    Video{id:5,title:"Exploring Tokyo After Dark",creator:"Wanderer",category:"Travel",duration:"16:38",views:"17K",emoji:"🌃",colors:"#283c60,#b95c85",description:"A stroll through glowing streets and hidden spots."},
    Video{id:6,title:"The Future of AI and Open Source",creator:"Future Lab",category:"Technology",duration:"24:03",views:"56K",emoji:"🤖",colors:"#47385e,#a27bd5",description:"How community-built software is shaping the future."},
    Video{id:7,title:"Perfect Morning Coffee",creator:"Daylight",category:"Lifestyle",duration:"08:42",views:"13K",emoji:"☕",colors:"#54402d,#d29a5a",description:"From grinding beans to the perfect pour."},
    Video{id:8,title:"Make Electronic Music from Scratch",creator:"Soundwave",category:"Music",duration:"13:07",views:"31K",emoji:"🎹",colors:"#512c4f,#cd6da5",description:"An approachable beginner electronic music lesson."},
    Video{id:9,title:"Creative Filmmaking Techniques",creator:"Nova Films",category:"Film",duration:"12:48",views:"43K",emoji:"🎬",colors:"#633858,#bd7051",description:"Simple techniques for beautiful storytelling."},
];

#[derive(Deserialize,Default)]
struct Filters { q: Option<String>, category: Option<String> }
fn escape(s:&str)->String { s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;").replace('\'', "&#39;") }
fn base(title:&str, body:String)->Html<String> {
    Html(format!(r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{} · StreamVerse</title><meta name="description" content="StreamVerse, the open-source Rust video platform"><link rel="stylesheet" href="/styles.css"></head><body><header><a class="brand" href="/"><span class="brandmark">▶</span> Stream<span>Verse</span></a><form class="search" action="/" method="get"><input name="q" type="search" placeholder="Search videos and creators..." aria-label="Search videos"><button type="submit">Search</button></form><a class="button hot" href="/studio">＋ Upload</a></header><div class="layout"><nav><a href="/">⌂ &nbsp; Home</a><a href="/?category=Technology">⌘ &nbsp; Technology</a><a href="/?category=Music">♫ &nbsp; Music</a><a href="/?category=Gaming">▣ &nbsp; Gaming</a><a href="/channel">◉ &nbsp; Channels</a><a href="/studio">▤ &nbsp; Creator Studio</a><div class="side-note">🦀 Rust-powered<br>Open source • Self-hostable<br>Mock data mode</div></nav><main>{}</main></div></body></html>"#,escape(title),body))
}
fn card(v:&Video)->String {
    format!(r#"<a class="card" href="/watch/{}"><div class="thumb" style="background:linear-gradient(130deg,{})"><span>{}</span><small>{}</small></div><h3>{}</h3><p>{} · {} views · 2 days ago</p></a>"#,v.id,v.colors,v.emoji,v.duration,escape(v.title),escape(v.creator),v.views)
}
async fn home(Query(filters):Query<Filters>)->Html<String>{
    let current=filters.category.as_deref().unwrap_or("All");
    let search=filters.q.as_deref().unwrap_or("").to_lowercase();
    let mut body=String::from(r#"<section class="hero"><div><div class="eyebrow">✦ YOUR PLATFORM. YOUR RULES.</div><h1>Videos without<br>the limits.</h1><p>An independent, open-source video community. Powered by Rust, made for creators. Your server, your brand, your content.</p><a class="button hot" href="/studio">Open Creator Studio ↗</a></div><div class="hero-icon">▶</div></section><div class="heading"><h2>Explore videos</h2><span class="demo">LIVE UI · SEEDED DEMO DATA</span></div><div class="chips">"#);
    for cat in ["All","Technology","Music","Gaming","Travel","Lifestyle","Film"]{
       body.push_str(&format!(r#"<a class="chip {}" href="/{}">{}</a>"#,if current==cat{"active"}else{""},if cat=="All"{String::new()}else{format!("?category={cat}")},cat));
    }
    body.push_str("</div><div class=\"grid\">");
    let mut count=0;
    for v in VIDEOS {
       if (current=="All" || current==v.category) && (search.is_empty() || v.title.to_lowercase().contains(&search) || v.creator.to_lowercase().contains(&search)){
           body.push_str(&card(v)); count+=1;
       }
    }
    body.push_str("</div>");
    if count==0{body.push_str("<p class=\"panel\">No matching videos found.</p>")}
    body.push_str("<p class=\"footnote\">All video entries are clearly labeled sample data. No Google Drive, 9Drive, or paid storage service is connected.</p>");
    base("Explore",body)
}
async fn watch(Path(id):Path<u32>)->impl IntoResponse {
  match VIDEOS.iter().find(|v|v.id==id){
    Some(v)=>{
      let mut related=String::new();for r in VIDEOS.iter().filter(|x|x.id!=id).take(4){related.push_str(&card(r))}
      let body=format!(r#"<a href="/" class="back">← Back to explore</a><div class="watch"><div class="playicon">{}</div><div class="watch-message">Sample entry — no video file or storage connected</div></div><h1>{}</h1><div class="info"><span>{} · {} views</span><span class="tag">{}</span></div><div class="panel"><strong>{}</strong><p>{}</p><div class="actions"><button disabled>♡ Like (connect database)</button><button disabled>＋ Subscribe (connect database)</button></div></div><section class="panel"><h2>Comments</h2><p>Comments will become available after the database is configured.</p></section><h2>More to explore</h2><div class="grid">{}</div>"#,v.emoji,escape(v.title),escape(v.creator),v.views,escape(v.category),escape(v.creator),escape(v.description),related);
      (StatusCode::OK,base(v.title,body)).into_response()
    },
    None=>(StatusCode::NOT_FOUND,base("Not found","<h1>Video not found</h1><a href=\"/\">Return home</a>".into())).into_response()
  }
}
async fn studio()->Html<String>{
  base("Creator Studio",r#"<div class="eyebrow">CREATOR DASHBOARD</div><h1>Creator Studio</h1><p class="subtitle">Manage your own video community with a Rust-powered backend.</p><div class="stats"><section class="panel"><strong>9</strong><span>Sample videos</span></section><section class="panel"><strong>0</strong><span>Real uploads</span></section><section class="panel"><strong>Not connected</strong><span>Storage status</span></section></div><section class="panel"><h2>Upload a video</h2><p>Real uploads are intentionally disabled until an administrator configures a supported storage adapter (for example, 9Drive). We won't pretend a file has been stored.</p><button disabled>Upload unavailable — storage disconnected</button></section><section class="panel"><h2>Project setup</h2><p>Rust + Axum server rendering, mock content fixtures, responsive UI, and planned pluggable storage adapters.</p><p>The source code is publicly available under the MIT license.</p></section>"#.into())
}
async fn channel()->Html<String>{
   let mut body=String::from("<div class=\"eyebrow\">COMMUNITY</div><h1>Explore channels</h1><p class=\"subtitle\">Creators listed below are sample data, not registered accounts.</p><div class=\"grid\">");
   let mut creators=std::collections::BTreeSet::new();
   for v in VIDEOS{creators.insert(v.creator);}
   for name in creators{body.push_str(&format!("<div class=\"panel\"><div class=\"channel-icon\">◉</div><h3>{}</h3><p>Demo channel</p><a href=\"/?q={}\">View videos →</a></div>",escape(name),escape(name)));}
   body.push_str("</div>");base("Channels",body)
}
async fn css()->impl IntoResponse { ([(header::CONTENT_TYPE,"text/css; charset=utf-8")],CSS) }
const CSS:&str=r#"
:root{color-scheme:dark;--bg:#0b0b10;--panel:#17171f;--line:#34343d;--fg:#f5f4fa;--muted:#aaa7b7;--hot:#f43d68}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--fg);font:15px system-ui,-apple-system,Segoe UI,sans-serif}a{text-decoration:none;color:inherit}button,input{font:inherit}button{cursor:pointer}header{height:72px;display:flex;align-items:center;gap:18px;padding:0 27px;background:var(--panel);border-bottom:1px solid var(--line);position:sticky;top:0;z-index:5}.brand{font-size:23px;font-weight:850;letter-spacing:-1px;white-space:nowrap}.brand span:last-child{color:var(--hot)}.brandmark{background:var(--hot);border-radius:10px;padding:7px;color:white}.search{display:flex;margin:auto;max-width:620px;width:100%;background:#252530;border:1px solid var(--line);border-radius:11px}.search input{flex:1;min-width:0;background:transparent;color:white;border:0;outline:0;padding:13px}.search button{background:#34343f;border:0;color:white;padding:0 16px;border-radius:0 10px 10px 0}.button{display:inline-block;padding:12px 17px;border-radius:11px}.hot{background:var(--hot);color:white;font-weight:750}.layout{display:flex}nav{width:232px;min-height:calc(100vh - 72px);flex-shrink:0;border-right:1px solid var(--line);padding:22px 12px}nav a{display:block;padding:15px;border-radius:11px;color:var(--muted)}nav a:hover{background:#29232d;color:white}.side-note{margin:35px 13px;line-height:1.8;color:var(--muted);font-size:12px}main{padding:36px clamp(18px,4vw,55px);width:100%;min-width:0;max-width:1700px}.hero{background:radial-gradient(ellipse at 85% 0%,#77314f,#322039 58%,#1c1928);border:1px solid #744054;border-radius:23px;padding:42px;display:flex;justify-content:space-between;align-items:center}.eyebrow{font-size:12px;color:#f8a5ba;font-weight:800;letter-spacing:2px}.hero h1{font-size:clamp(35px,5vw,56px);line-height:1.05;letter-spacing:-2px;margin:12px 0}.hero p{color:#e1c0d0;line-height:1.65;max-width:500px}.hero-icon{font-size:112px;padding-right:10%}.heading{display:flex;align-items:center;justify-content:space-between;margin-top:26px}.heading h2{font-size:25px}.demo,.footnote{color:var(--muted);font-size:12px}.chips{display:flex;gap:9px;overflow:auto;margin:8px 0 25px}.chip{background:#22232b;border:1px solid var(--line);padding:9px 15px;border-radius:10px;color:#c4c2ce;white-space:nowrap}.chip.active{background:var(--hot);border-color:var(--hot);color:white}.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(225px,1fr));gap:24px 18px}.card h3{font-size:15px;margin:12px 0 6px;line-height:1.4}.card p,.panel p,.subtitle,.info{color:var(--muted);line-height:1.65}.card p{font-size:12px;margin:0}.thumb{aspect-ratio:16/9;border-radius:16px;display:grid;place-items:center;position:relative;transition:transform .15s}.card:hover .thumb{transform:translateY(-4px)}.thumb span{font-size:56px}.thumb small{position:absolute;right:9px;bottom:9px;padding:5px 7px;background:#07070ac9;border-radius:5px}.panel{border:1px solid var(--line);background:var(--panel);border-radius:18px;padding:24px;margin:18px 0}.watch{aspect-ratio:16/9;background:radial-gradient(circle,#523255,#15131d);border-radius:19px;display:flex;justify-content:center;align-items:center;flex-direction:column}.playicon{font-size:100px}.watch-message{color:#dbc4d7}.back{display:inline-block;margin-bottom:18px;color:#cdb1c0}.info,.actions{display:flex;gap:12px;flex-wrap:wrap}.tag{border:1px solid var(--line);border-radius:8px;padding:3px 11px}.actions button,.panel button{padding:12px;background:#33313c;border:1px solid var(--line);border-radius:10px;color:#aaa}.stats{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:13px}.stats .panel{display:grid;gap:6px}.stats strong{font-size:26px}.stats span{color:var(--muted)}.channel-icon{font-size:35px;color:var(--hot)}.footnote{margin-top:36px}@media(max-width:900px){nav{display:none}.hero-icon{display:none}}@media(max-width:600px){header{padding:0 12px;gap:9px}.brand{font-size:0}.brandmark{font-size:18px}.search button{display:none}.hero{padding:26px}.grid{grid-template-columns:repeat(2,minmax(0,1fr));gap:18px 10px}.stats{grid-template-columns:1fr}.demo{display:none}.watch-message{font-size:12px;text-align:center}}
"#;
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
 let port=std::env::var("PORT").ok().and_then(|x|x.parse::<u16>().ok()).unwrap_or(3000);
 let app=Router::new().route("/",get(home)).route("/watch/{id}",get(watch)).route("/studio",get(studio)).route("/channel",get(channel)).route("/styles.css",get(css));
 let listener=tokio::net::TcpListener::bind(SocketAddr::from(([0,0,0,0],port))).await?;
 println!("StreamVerse listening at http://localhost:{port}");
 axum::serve(listener,app).await?;
 Ok(())
}
