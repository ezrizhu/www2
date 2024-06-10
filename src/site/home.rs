use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;
//use rand::Rng;

pub async fn home(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;
    let workstation = state.workstation.clone();
    let val = state.val.clone();
    let steam = state.steam.clone();
    let discord = state.discord.clone();
    let cloud = state.cloud.clone();

    /*
       let mut rng = rand::thread_rng();
       let (img, img_link, artist) = match rng.gen_range(0..3) {
       0 => ("ezri.webp", "https://v3ss33l.crd.co/", "V3SS33L"),
       1 => ("pixel.webp", "https://toyhou.se/StandbySnail", "StandbySnail"),
       2 => ("blueberry.webp", "https://koiwypher.uwu.ai/#/", "Wypher"),
       _ => unreachable!(),
       };
       */

    let (img, img_link, artist) = ("ezri.webp", "https://v3ss33l.crd.co/", "V3SS33L");

    let img = format!("/assets/img/{}", img);

    let content = html! {
        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-2-3 hero-text" {
                h1 { "Ezri" }
                a target="_blank" href="https://en.pronouns.page/terminology#nonbinary" {
                    img class="flag" src="/assets/img/Nonbinary.webp" alt="Nonbinary flag";
                }
                a target="_blank" href="https://en.pronouns.page/terminology#pansexual" {
                    img class="flag" src="/assets/img/Pansexual.webp" alt="Pansexual flag";
                }
                p { "I am a 20 y/o computer science student from NYC that runs a small internet hosting service with its own ASN. I currently work in academia as a research assistant." }
                p { "Feel free to have a look around this website, and I hope you have a nice rest of your day." }
                p { "Please don't hesitate to reach out if you'd like to chat or have any questions." }
            }
            div class="pure-u-1 pure-u-md-1-3 hero-img" {
                a target="_blank" href="https://toyhou.se/finnekit" {
                    img class="pure-img" src=(img) alt="Ezri's avatar";
                }
                p { "Art by " a target="_blank" href=(img_link) { (artist) } "." }
            }
        }

        h3 { "Socials" }

        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-1-2" {
                p {
                    b { "Fediverse: " }
                    a rel="me" target="_blank" href="https://sleepless.cafe/ezri" {
                        "@ezri@sleepless.cafe"
                    }
                    br;
                    b { "Matrix: " }
                    a rel="me" target="_blank" href="https://matrix.to/#/@ezri:envs.net" {
                        "@ezri:envs.net"
                    }
                    br;
                    b { "Twitter: " }
                    a rel="me" target="_blank" href="https://twitter.com/0xEzri" {
                        "@0xEzri"
                    }
                    br;
                    b { "BSky: " }
                    a rel="me" target="_blank" href="https://bsky.app/profile/ezrizhu.com" {
                        "@ezrizhu.com"
                    }
                    br;
                    b { "GitHub: "}
                    a rel="me" target="_blank" href="https://github.com/ezrizhu" {
                        "@ezrizhu"
                    }
                }
            }
            div class="pure-u-1 pure-u-md-1-2" {
                p {
                    b { "Signal: " }
                    a rel="me" target="_blank" href="https://signal.me/#eu/ZhPPlw2hqcjo2BO1QEmD-XxMfVCtCG5n8gOLmV4yxpPcsBuJZFJBqblyOvo7XrOM" {
                        "ezri.01"
                    }
                    br;
                    b { "Email: " }
                    a target="_blank" href="mailto:me@ezri.pet" {
                        "me@ezri.pet"
                    }
                    br;
                    b { "pronouns.page: " }
                    a target="_blank" href="https://en.pronouns.page/@ezrieh" {
                        "@ezrieh"
                    }
                    br;
                    b { "Telegram: " }
                    a target="_blank" href="https://t.me/ezrizhu" {
                        "@ezrizhu"
                    }
                    br;
                    b { "Irc: " }
                    "ezri on libera, hackint" 
                }
            }
        }

        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Discord" }
                p {
                    "Username: ezrieh"
                        br;
                    "Custom status: " (discord.custom_status)
                        br;
                    "Web client: " 
                        @if discord.status_web == "" {
                            "offline"
                        } @else {
                            i { (discord.status_web) }
                        }
                    br;

                    "Mobile client: " 
                        @if discord.status_mobile == "" {
                            "offline"
                        } @else {
                            i { (discord.status_mobile) }
                        }
                    br;

                    "Desktop client: " 
                        @if discord.status_desk == "" {
                            "offline"
                        } @else {
                            i { (discord.status_desk) }
                        }
                }
            }

            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Valorant" }
                p {
                    pre { (val) };
                }
            }

            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Steam" }
                p {
                    "Profile: " a target="_blank" href=(steam.profile_url) { (steam.persona_name) }
                    br;
                    "Currently: " (steam.persona_state)
                        @if steam.is_gaming {
                            br;
                            "Playing: " a target="_blank" href=(steam.game_url) { (steam.game_extra_info) }
                        };
                    br;
                    "Last log off: " (steam.last_logoff)
                }
            }

            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "EzriCloud" }
                p {
                    "AS: 206628"
                        br;
                    "Status: "
                        @if cloud.is_down {
                            "Down since" (cloud.down_since)
                        } @else {
                            "All systems operational"
                        }
                    br;
                    "nic-hdl: EZRI-RIPE, ZHUEZ-ARIN"
                }
            }

        }


        h3 { "Workstation status" }
        pre { (workstation) };

    };

    base(content, state.clone())
}
