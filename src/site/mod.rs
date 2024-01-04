use maud::{html, Markup};
use crate::SiteState;
pub mod home;

pub fn base(content: Markup, state: SiteState) -> Markup {
    let last_updated = state.last_updated.clone();
    let build_info = format!("Built on: {} • Ref: {} • Commit: {}",
                             std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")),
                             );
    let description = "Ezri's website";
    let title = "Ezri";

    html! {
        (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    link rel="stylesheet" href="/assets/css/pure-min.css";
                    link rel="stylesheet" href="/assets/css/main.css";
                    link rel="stylesheet" href="/assets/css/grids-responsive-min.css";

                    title { (title) };
                    meta name="description" content=(description);
                    meta name="author" content="Ezri";

                    link rel="apple-touch-icon" sizes="180x180" href="/assets/favicon/apple-touch-icon.png";
                    link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
                    link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";
                    link rel="manifest" href="/assets/favicon/site.webmanifest";

                    meta name="theme-color" content="#f7b8c6";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(description);
                    meta property="og:theme-color" content="#f7b8c6";
                }

                body {
                    div class="main" {
                        (content);
                        div class="footer" {
                            div class="badges" {
                                img src="/assets/img/badges/ezri.png" alt="Ezri";
                                img src="/assets/img/badges/ezricloud.png" alt="EzriCloud";
                                a href="https://kate.pet" { 
                                    img src="/assets/img/badges/kate.gif" alt="kate.pet";
                                }
                                a href="https://easrng.net" {
                                    img src="/assets/img/badges/easrng.gif" alt="easrng";
                                }
                                a href="https://s-mith.github.io/awfulwebsite/" {
                                    img src="/assets/img/badges/lily.gif" alt="lily";
                                }
                                a href="https://tilde.town/" {
                                    img src="/assets/img/badges/tildetownpink.gif" alt="tilde.town";
                                }
                                img src="/assets/img/badges/xenia-now.gif" alt="xenia-now";
                                img src="/assets/img/badges/vimlove.gif" alt="vim";
                                a href="https://xenyth.net/" {
                                    img src="/assets/img/badges/xenyth.png" alt="xenyth cloud";
                                }
                                img src="/assets/img/badges/aperture_labs.jpg" alt="aperture_labs";
                                img src="/assets/img/badges/nb_noproblem.jpg" alt="nonbinary_noproblem";
                                iframe src="//incr.easrng.net/badge?key=ezripet" style="background: url(//incr.easrng.net/bg.gif)" title="increment badge" width="88" height="31" frameborder="0" {};
                            }

                            p {
                                "Auto refreshed: " (last_updated)
                                    br;
                                    "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                                    br;
                                "Source code "
                                    a target="_blank" href="https://github.com/ezrizhu/www2" { "available here" }
                                ", released under the "
                                    a target="_blank" href="https://github.com/ezrizhu/www2/blob/main/COPYING" { "GNU AGPLv3 license" }
                                "."
                                    br;
                                (build_info);
                            }
                        }
                    }

                }
            }
    }
}
