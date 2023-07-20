use leptos::*;

#[component]
pub fn Calculator(cx: Scope,) -> impl IntoView {
    let (res, set_res) = create_signal(cx, "0".to_string());
    let (first, set_first) = create_signal(cx, String::new());
    let (first_number, set_first_number) = create_signal(cx, String::new());
    let (second, set_second) = create_signal(cx, String::new());
    let (sign, set_sign) = create_signal(cx, String::new());

    fn calculate(first: String, second: String, sign: String) -> String {
        let result;

        let first: u32 = first.trim().parse().unwrap();
        let second: u32 = second.trim().parse().unwrap();

        match sign.trim() {
            "+" => result = first + second,
            "-" => result = first - second,
            "*" => result = first * second,
            "/" => result = first / second,
            _ => result = 0
        };

        result.to_string()
    }

    view! { cx,
      <section class="calculator">
            <div class="res">
                <div class="lastOperand">
                    <p class="first">{first_number}</p>
                    <p class="sign">{sign}</p>
                </div>
                <div class="res__inner">
                    <p class="result">{res}</p>
                </div>
            </div>
            <div class="navs">
                <div class="left">
                    <div class="buttons clearbtns">
                        <button on:click=move |_| {
                            set_res.update(|res| *res = "0".to_string());
                            set_first.update(|first| *first = String::new());
                            set_first_number.update(|first_number| *first_number = String::new());
                            set_second.update(|second| *second = String::new());
                            set_sign.update(|sign| *sign = String::new());
                        }>"AD"</button>
                        <button on:click=move |_| {
                            set_res.update(|res| {
                                if *res != "0" {
                                    if res.len() > 1 {
                                        *res = res[0..res.len() - 1].to_string();
                                    } else {
                                        *res = "0".to_string();
                                    }
                                }
                            });
                            if sign.get().len() == 0 {
                                set_first.update(|first| *first = first[0..first.len() - 1].to_string());
                            } else {
                                set_second.update(|second| *second = second[0..second.len() - 1].to_string());
                            };
                        }>"DE"</button>
                    </div>
                    <div class="buttons numbers">
                        <div class="rows row1">
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "1".to_string();
                                    } else {
                                        *res += "1";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "1".to_string();
                                        } else {
                                            *first += "1";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "1".to_string();
                                        } else {
                                            *first += "1";
                                        }

                                    })
                                };

                            }>1</button>
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "2".to_string();
                                    } else {
                                        *res += "2";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "2".to_string();
                                        } else {
                                            *first += "2";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "2".to_string();
                                        } else {
                                            *first += "2";
                                        }

                                    })
                                };

                            }>2</button>
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "3".to_string();
                                    } else {
                                        *res += "3";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "3".to_string();
                                        } else {
                                            *first += "3";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "3".to_string();
                                        } else {
                                            *first += "3";
                                        }

                                    })
                                };

                            }>3</button>
                        </div>
                        <div class="rows row2">
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "4".to_string();
                                    } else {
                                        *res += "4";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "4".to_string();
                                        } else {
                                            *first += "4";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "4".to_string();
                                        } else {
                                            *first += "4";
                                        }

                                    })
                                };

                            }>4</button>
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "5".to_string();
                                    } else {
                                        *res += "5";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "5".to_string();
                                        } else {
                                            *first += "5";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "5".to_string();
                                        } else {
                                            *first += "5";
                                        }

                                    })
                                };

                            }>5</button>
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "6".to_string();
                                    } else {
                                        *res += "6";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "6".to_string();
                                        } else {
                                            *first += "6";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "6".to_string();
                                        } else {
                                            *first += "6";
                                        }

                                    })
                                };

                            }>6</button>                       
                        </div>
                        <div class="rows row3">
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "7".to_string();
                                    } else {
                                        *res += "7";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "7".to_string();
                                        } else {
                                            *first += "7";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "7".to_string();
                                        } else {
                                            *first += "7";
                                        }

                                    })
                                };

                            }>7</button>
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "8".to_string();
                                    } else {
                                        *res += "8";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "8".to_string();
                                        } else {
                                            *first += "8";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "8".to_string();
                                        } else {
                                            *first += "8";
                                        }

                                    })
                                };

                            }>8</button>
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "9".to_string();
                                    } else {
                                        *res += "9";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "9".to_string();
                                        } else {
                                            *first += "9";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "9".to_string();
                                        } else {
                                            *first += "9";
                                        }

                                    })
                                };

                            }>9</button>                       
                        </div>
                    </div>
                    <div class="buttons lr">
                            <button on:click=move |_| {
                                set_res.update(|res| {

                                    if *res == "0" {
                                        *res = "0".to_string();
                                    } else {
                                        *res += "0";
                                    }

                                });

                                if sign.get().len() == 0 {
                                    set_first.update(|first| {
                                        if *first == "0" {
                                            *first = "0".to_string();
                                        } else {
                                            *first += "0";
                                        }

                                    })
                                } else {
                                    set_second.update(|first| {
                                        if *first == "0" {
                                            *first = "0".to_string();
                                        } else {
                                            *first += "0";
                                        }

                                    })
                                };

                            }>0</button>                       
                        <button class="equals" on:click=move |_| {
                            set_res.update(|res| {
                                *res = calculate(first.get(), second.get(), sign.get());
                            });
                            set_sign.update(|sign| *sign = String::new());
                            set_first.update(|first| *first = String::new());
                            set_second.update(|second| *second = String::new());
                            set_first_number.update(|first_number| *first_number = String::new());
                        }>"="</button>
                    </div>
                </div>
                <div class="right">
                    <button on:click=move |_| {
                        set_sign.update(|sign| *sign = "+".to_string());
                        set_first_number.update(|first_number| *first_number = first.get());
                        set_res.update(|res| *res = String::new());
                    }>"+"</button>
                    <button on:click=move |_| {
                        set_sign.update(|sign| *sign = "-".to_string());
                        set_first_number.update(|first_number| *first_number = first.get());
                        set_res.update(|res| *res = String::new());
                    }>"-"</button>
                    <button on:click=move |_| {
                        set_sign.update(|sign| *sign = "*".to_string());
                        set_first_number.update(|first_number| *first_number = first.get());
                        set_res.update(|res| *res = String::new());
                    }>"*"</button>
                    <button on:click=move |_| {
                        set_sign.update(|sign| *sign = "/".to_string());
                        set_first_number.update(|first_number| *first_number = first.get());
                        set_res.update(|res| *res = String::new());
                    }>"/"</button>
                </div>
            </div>
        </section>
    }
}
