use super::base;
use crate::SiteState;
use axum::extract::State;
use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
//use rand::Rng;

pub async fn home(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;
    let workstation = state.workstation.clone();
    let _val = state.val.clone();
    let _steam = state.steam.clone();
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

    let (img, img_link, artist) = ("ezri2.webp", "https://fatalterror3.crd.co/", "V3SS33L");

    let img = format!("/assets/img/{}", img);

    let content = html! {
        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-2-3 hero-text" {
                h1 {
                    "ezri " 
                }
                p { "I am a 21 y/o computer science student from NYC that runs a small internet hosting service with its own ASN." }
                p { "I currently work in academia as research assistants for two groups, one in computer systems, and one in computer networking." }
                p { "Feel free to have a look around this website, and I hope you have a nice rest of your day." }
                p { "Please don't hesitate to reach out if you'd like to chat or have any questions." }
            }
            div class="pure-u-1 pure-u-md-1-3 hero-img" {
                a target="_blank" href="https://toyhou.se/24606420.ezri" {
                    img class="pure-img" src=(img) alt="ezri's avatar";
                }
                p { "Art by " a target="_blank" href=(img_link) { (artist) } "." }
            }
        }

        h3 { "Socials" }

        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-1-2" {
                p {
                    b { "Website: " }
                    a rel="me" target="_blank" href="https://ezrizhu.com" {
                        "ezrizhu.com"
                    }
                    br;
                    b { "Email: " }
                    a target="_blank" href="mailto:me@ezri.pet" {
                        "me@ezri.pet"
                    }
                    br;
                    b { "Matrix: " }
                    a rel="me" target="_blank" href="https://matrix.to/#/@ezri:envs.net" {
                        "@ezri:envs.net"
                    }
                    br;
                    b { "Fediverse: " }
                    a rel="me" target="_blank" href="https://starry.cafe/@ezri" {
                        "@ezri@starry.cafe"
                    }
                    br;
                    b { "Signal: " }
                    a rel="me" target="_blank" href="https://signal.me/#eu/ZhPPlw2hqcjo2BO1QEmD-XxMfVCtCG5n8gOLmV4yxpPcsBuJZFJBqblyOvo7XrOM" {
                        "ezri.01"
                    }
                }
            }
            div class="pure-u-1 pure-u-md-1-2" {
                p {
                    b { "PGP & ID Proofs: " }
                    a rel="me" target="_blank" href="https://keyoxide.org/me%40ezri.pet" {
                        "wkd:me@ezri.pet"
                    }
                    br;
                    b { "Email (alt): " }
                    a target="_blank" href="mailto:ezrieh@riseup.net" {
                        "ezrieh@riseup.net"
                    }
                    br;
                    b { "XMPP: " }
                    a target="_blank" href="xmpp:ezri@disroot.org" {
                        "ezri@disroot.org"
                    }
                    br;
                    b { "Twitter: " }
                    a rel="me" target="_blank" href="https://twitter.com/0xEzri" {
                        "@0xEzri"
                    }
                    br;
                    b { "Telegram: " }
                    a target="_blank" href="https://t.me/ezrizhu" {
                        "@ezrizhu"
                    }
                }
            }
        }

        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Discord" }
                p {
                    "Username: "
                    a target="_blank" href="https://discord.com/users/691734266458931341" {
                        "ezrieh"
                    }
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
                h3 { "EzriCloud" }
                p {
                    "Website: "
                    a target="_blank" href="https://ezri.cloud" {
                        "ezri.cloud"
                    }
                    br;
                    "AS: 206628"
                        br;
                    "Status: "
                        @if cloud.is_down {
                            "Outage since" (cloud.down_since)
                        } @else {
                            "All systems operational"
                        }
                    br;
                    "nic-hdl: EZRI-RIPE"
                    br;
                    "nic-hdl: ZHUEZ-ARIN"
                }
            }

        }

        h3 { "Workstation status" }
        pre { (workstation) };

    };

    base(content, state.clone())
}
