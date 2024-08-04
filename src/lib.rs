use rand::Rng;


#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    slint::slint! {
        import { Button, GroupBox, TabWidget, AboutSlint } from "std-widgets.slint";
        import "res/fonts/cherolina/Cherolina.ttf";

        struct Die {
            image: image,
        }

        export component MainWindow inherits Window {

            in property <[Die]> dice: [
                { image: @image-url("res/images/dice/dice_1.png") },
                { image: @image-url("res/images/dice/dice_2.png") },
                { image: @image-url("res/images/dice/dice_3.png") },
                { image: @image-url("res/images/dice/dice_4.png") },
                { image: @image-url("res/images/dice/dice_5.png") },
                { image: @image-url("res/images/dice/dice_6.png") },
            ];

            in property <int> index: 0;

            callback rand-num(int);

            background: purple;

            GroupBox {
                width: parent.width;
                height: parent.height;
                VerticalLayout {
                    TabWidget {
                        Tab {
                            title: "Tab 1";
                            VerticalLayout {
                                spacing: 10px;
                                padding: 10px;
                                Text {
                                    text: "Ai's Dice";
                                    color: white;
                                    font-family: "Cherolina";
                                    font-size: 32px;
                                    horizontal-alignment: center;
                                }
                                Image {
                                    source: dice[index].image;
                                }
                                Button {
                                    text: "Roll";
                                    height: 50px;
                                    clicked => {
                                        root.rand-num(dice.length);
                                    }
                                }
                            }
                        }
                        Tab {
                            title: "Tab 2";
                            Text {
                                text: "Coming soon...";
                                color: white;
                                font-family: "Cherolina";
                                font-size: 32px;
                            }
                        }
                        Tab {
                            title: "About";
                            AboutSlint {}
                        }
                    }
                }
            }
        }

    }

    let window = MainWindow::new().unwrap();

    let weak = window.as_weak();

    window.on_rand_num(move |len| {
        let rand_num = rand::thread_rng().gen_range(0..len);
        let window = weak.unwrap();
        window.set_index(rand_num);
    });

    window.run().unwrap();
}
