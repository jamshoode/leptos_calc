use leptos::*;

#[component]
pub fn Calculator(cx: Scope) -> impl IntoView {
    let (res, set_res) = create_signal(cx, "0".to_string());
    let (result, set_result) = create_signal(cx, String::new());
    let (first, set_first) = create_signal(cx, String::new());
    let (first_number, set_first_number) = create_signal(cx, String::new());
    let (second, set_second) = create_signal(cx, String::new());
    let (sign, set_sign) = create_signal(cx, String::new());
    let (_, set_state) = create_signal(cx, false);

    fn calculate(first: String, second: String, sign: String) -> String {
        let result;

        let first: u32 = first.trim().parse().unwrap();
        let second: u32 = second.trim().parse().unwrap();

        match sign.trim() {
            "+" => result = first + second,
            "-" => result = first - second,
            "*" => result = first * second,
            "/" => result = first / second,
            _ => result = 0,
        };

        result.to_string()
    }

    let button_click = move |name: &str| {
        set_result.update(|result| *result = String::new());
        set_state.update(|state| *state = false);
        set_res.update(|res| {

            if *res == "0" {
                *res = name.clone().to_string();
            } else {
                *res += name.clone();
            }

        });

        if sign.get().len() == 0 {
            set_first.update(|first| {
                if *first == "0" {
                    *first = name.clone().to_string();
                } else {
                    *first += name.clone();
                }

            })
        } else {
            set_second.update(|first| {
                if *first == "0" {
                    *first = name.clone().to_string();
                } else {
                    *first += name;
                }

            })
        };
    };

    view! { cx,
      <section class="calculator">
            <div class="res">
                <div class="lastOperand">
                    <p class="first">{first_number}</p>
                    <p class="sign">{sign}</p>
                </div>
                <div class="res__inner">
                    <p class="result">{
                        move || if res.get().len() == 0 {result} else {res}
                    }</p>
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
                                "0".to_string();
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
                            <button on:click=move |_| button_click("1")>1</button>
                            <button on:click=move |_| button_click("2")>2</button>
                            <button on:click=move |_| button_click("3")>3</button>
                        </div>
                        <div class="rows row2">
                            <button on:click=move |_| button_click("4")>4</button>
                            <button on:click=move |_| button_click("5")>5</button>
                            <button on:click=move |_| button_click("6")>6</button>
                        </div>
                        <div class="rows row3">
                            <button on:click=move |_| button_click("7")>7</button>
                            <button on:click=move |_| button_click("8")>8</button>
                            <button on:click=move |_| button_click("9")>9</button>
                        </div>
                    </div>
                    <div class="buttons lr">
                            <button on:click=move |_| {
                                set_result.update(|result| *result = String::new());
                                set_state.update(|state| *state = false);
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
                            set_state.update(|state| *state = true);
                            set_res.update(|res| *res = String::new());
                            set_result.update(|result| {
                                *result = calculate(first.get(), second.get(), sign.get());
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
