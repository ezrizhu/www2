use crate::SiteState;
use maud::{html, Markup};
pub mod home;

pub fn base(content: Markup, state: SiteState) -> Markup {
    let last_updated = state.last_updated.clone();
    let build_info = format!(
        "Built on: {} • Ref: {} • Commit: {}",
        std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
        std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
        std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")),
    );
    let description = "ezri's website";
    let title = "ezri";

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
                    meta name="author" content="ezri";

                    link rel="apple-touch-icon" sizes="180x180" href="/assets/favicon/apple-touch-icon.png";
                    link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
                    link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";
                    link rel="manifest" href="/assets/favicon/site.webmanifest";

                    meta name="theme-color" content="#1c1624";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(description);
                    meta property="og:theme-color" content="#1c1624";
                }

                body {
                    div class="main" {
                        a id="prideflag" href="https://www.hrw.org/topic/lgbt-rights" target="_blank" { img src="/assets/img/pride.svg" alt="progressive pride flag"; }
                        (content);
                        div class="footer" {
                            p {
                                a href="https://fediring.net/previous?host=ezri.pet" { "<- Prev" }
                                " |  " a href="https://fediring.net/" { "fediring" } "  | "
                                a href="https://fediring.net/next?host=ezri.pet" { "Next ->" }
                            }
                            p {
                                a href="https://skylarhill.me/" { "<- Skylar" }
                                " |  Hacker Girls  | "
                                a href="https://nora.codes/" { "Nora ->" }
                            }
                            p {
                                a href="https://stellophiliac.github.io/roboring/0x657a7269/previous" { "<- Prev" }
                                " |  " a href="https://stellophiliac.github.io/roboring" { "roboring" } "  | "
                                a href="https://stellophiliac.github.io/roboring/0x657a7269/next" { "Next ->" }
                            }
                            br;
                            div class="badges" {
                                a target="_blank" href="https://ezri.pet" {
                                    img src="/assets/img/badges/ezri.png" alt="ezri";
                                }
                                a target="_blank" href="https://ezri.cloud" {
                                    img src="/assets/img/badges/ezricloud.png" alt="EzriCloud";
                                }
                                a target="_blank" href="https://wolfgirl.systems" {
                                    img src="/assets/img/badges/wolfgirl.gif" alt="wolfgirl.systems";
                                }
                                a target="_blank" href="https://kate.pet" {
                                    img src="/assets/img/badges/kate.gif" alt="kate.pet";
                                }
                                a target="_blank" href="https://haylinmoore.com" {
                                    img src="/assets/img/badges/haylinmoore.com.png" alt="haylin";
                                }
                                a target="_blank" href="https://easrng.net" {
                                    img src="/assets/img/badges/easrng.gif" alt="easrng";
                                }
                                a target="_blank" href="https://girlboss.ceo/" {
                                    img src="/assets/img/badges/june.svg" alt="june";
                                }
                                a target="_blank" href="https://itzzen.net" {
                                    img src="/assets/img/badges/itzzennet.png" alt="itzzennet";
                                }
                                a target="_blank" href="https://constellatory.net" {
                                    img src="/assets/img/badges/constellatory.png" alt="constellatory";
                                }
                                a target="_blank" href="https://s-mith.github.io/awfulwebsite" {
                                    img src="/assets/img/badges/lily.gif" alt="lily";
                                }
                                a target="_blank" href="https://melody.codes" {
                                    img src="/assets/img/badges/melody.png" alt="melody";
                                }
                                a target="_blank" href="https://wilnil.gay" {
                                    img src="/assets/img/badges/wilnil_takeone.gif" alt="wilnil";
                                }
                                a target="_blank" href="https://graydenn.wtf" {
                                    img src="/assets/img/badges/graydenn.png" alt="graydenn";
                                }
                                a target="_blank" href="https://adryd.com" {
                                    img src="/assets/img/badges/adryd.png" alt="adryd";
                                }
                                a target="_blank" href="https://marisakirisame.net" {
                                    img src="/assets/img/badges/marisakirisame.net.png" alt="Marisa";
                                }
                                img src="https://marisakirisame.net/battle-of-wits.gif" alt="battle of wits";
                                a target="_blank" href="https://joscomputing.space" {
                                    img src="/assets/img/badges/spotlight.gif" alt="spotlight";
                                }
                                a target="_blank" href="https://pixilic.com" {
                                    img src="/assets/img/badges/hunter.png" alt="hunter";
                                }
                                a target="_blank" href="https://sapphic.moe" {
                                    img src="/assets/img/badges/sapphic.png" alt="Sapphic Angels";
                                }
                                a target="_blank" href="https://maia.lgbt" {
                                    img src="/assets/img/badges/maia.gif" alt="maia";
                                }
                                a target="_blank" href="https://seirdy.one" {
                                    img src="/assets/img/badges/seirdy.one.png" alt="seirdy";
                                }
                                a target="_blank" href="https://zvava.org" {
                                    img src="/assets/img/badges/zvava.org.png" alt="zvava.org";
                                }
                                a target="_blank" href="https://999eagle.moe" {
                                    img src="/assets/img/badges/440729.png" alt="⛧-440729";
                                }
                                a target="_blank" href="https://noe.sh" {
                                    img src="/assets/img/badges/noe.sh.png" alt="41666 noe";
                                }
                                a target="_blank" href="https://moondvsted.neocities.org" {
                                    img src="/assets/img/badges/d-17840-at-moondvsted.gif" alt="moondvsted";
                                }
                                a target="_blank" href="https://softkittypa.ws" {
                                    img src="/assets/img/badges/softkittypaws.png" alt="softkittypaws";
                                }
                                a target="_blank" href="https://libdb.so" {
                                    img src="/assets/img/badges/libdb.so.gif" alt="libdb.so";
                                }
                                a target="_blank" href="https://xaselgio.net/" {
                                    img src="/assets/img/badges/88x31_xaselgio.gif" alt="xaselgio";
                                }
                                a target="_blank" href="https://lily.pet/" {
                                    img src="/assets/img/badges/lily_pet.gif" alt="lily.pet";
                                }
                                a target="_blank" href="https://notnite.com/" {
                                    img src="/assets/img/badges/notnite.png" alt="notnite";
                                }
                                a target="_blank" href="https://slonk.ing/" {
                                    img src="/assets/img/badges/slonk.ing.gif" alt="slonksite";
                                }
                                a target="_blank" href="https://matdoes.dev/" {
                                    img src="/assets/img/badges/mat.png" alt="matdoesdev";
                                }
                                a target="_blank" href="https://goldenstack.net/" {
                                    img src="/assets/img/badges/golden_stack.png" alt="goldenstack";
                                }
                                a target="_blank" href="https://kibty.town/" {
                                    img src="/assets/img/badges/kibty.town-badge.gif" alt="kibty.town";
                                }
                                a target="_blank" href="https://nullablevo.id.au/" {
                                    img src="/assets/img/badges/avery.png" alt="nullable bytes";
                                }
                                a target="_blank" href="https://vmfunc.re/" {
                                    img src="/assets/img/badges/vmfunc.png" alt="vmfunc";
                                }
                                a target="_blank" href="https://zptr.cc/" {
                                    img src="/assets/img/badges/zeroptr.gif" alt="zeroptr";
                                }
                                a target="_blank" href="https://damcraft.de/" {
                                    img src="/assets/img/badges/dam.gif" alt="damcraft";
                                }
                                a target="_blank" href="https://aubrey.rs/" {
                                    img src="/assets/img/badges/aubrey.png" alt="aubrey";
                                }
                                a target="_blank" href="https://bun-e.fyi/" {
                                    img src="/assets/img/badges/3bunbutton.png" alt="ecool";
                                }
                                a target="_blank" href="https://giannirosato.com/" {
                                    img src="/assets/img/badges/gb82_88x31.webp" alt="gianni";
                                }
                                a target="_blank" href="https://tilde.town" {
                                    img src="/assets/img/badges/tildetownpink.gif" alt="tilde.town";
                                }
                                a target="_blank" href="https://envs.net/" {
                                    img src="/assets/img/badges/envs.net.png" alt="envs.net";
                                }
                                img src="/assets/img/badges/xenia-now.gif" alt="xenia-now";
                                a target="_blank" href="https://neovim.io/" {
                                    img src="/assets/img/badges/neovim.png" alt="neovim";
                                }
                                a target="_blank" href="https://xkcd.com" {
                                    img src="/assets/img/badges/xkcd.png" alt="xkcd";
                                }
                                a target="_blank" href="https://infernocomms.com" {
                                    img src="/assets/img/badges/infernocomms.png" alt="Inferno Communications";
                                }
                                a target="_blank" href="https://matrix.org/" {
                                    img src="/assets/img/badges/matrix.png" alt="Matrix";
                                }
                                a target="_blank" href="https://nixos.org/" {
                                    img src="/assets/img/badges/nixos.gif" alt="Nixos";
                                }
                                img src="/assets/img/badges/aperture_labs.jpg" alt="aperture_labs";
                                img src="/assets/img/badges/nb_noproblem.jpg" alt="nonbinary_noproblem";
                                a target="_blank" href="https://hrc.org/transgender" {
                                    img src="/assets/img/badges/transrightsnow.webp" alt="transrightsnow";
                                }
                                a target="_blank" href="https://www.mabsland.com/Adoption.html" {
                                    img src="/assets/img/badges/Censor_PGc.gif" alt="Censorship Panda: PG";
                                }
                                a target="_blank" href="https://eightyeightthirty.one/#ezri.pet" {
                                    img src="/assets/img/badges/eightyeightthirtyone.png" alt="eightyeightthirtyone";
                                }
                                a target="_blank" href="https://whats-th.is" {
                                    img src="/assets/img/badges/owo.gif" alt="owo";
                                }
                                a target="_blank" href="https://nightfall.city" {
                                    img src="/assets/img/badges/nightfall.city.png" alt="nightfall.city";
                                }
                                img src="/assets/img/badges/nap.png" alt="not a person";
                                a target="_blank" href="https://fediring.net" {
                                    img src="/assets/img/badges/fediring.gif" alt="fediring";
                                }
                                img src="/assets/img/badges/hackergirls.gif" alt="hackergirls";
                                img src="/assets/img/badges/hrt-e2.gif" alt="HRT 01/24/24";
                                iframe src="https://incr.easrng.net/badge?key=ezripet" style="background: url(https://incr.easrng.net/bg.gif)" title="increment badge" width="88" height="31" frameborder="0" {};
                            }

                            p {
                                "unit ⎈-657a7269, \"ezri\""
                                br;
                                " ▌▌▌▖▖▘▘▖▖▌▘▌▌▌▖▖▘▖▖"
                                br;
                                "Auto refreshed: " (last_updated)
                                br;
                                "Source code "
                                a target="_blank" href="https://github.com/ezrizhu/www2" { "available here" }
                                ", released under the "
                                a target="_blank" href="https://github.com/ezrizhu/www2/blob/main/COPYING" { "GNU AGPLv3 license." }
                                br;
                                "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                                br;
                                (build_info);
                            }
                        }
                    }

                }
            }
    }
}
