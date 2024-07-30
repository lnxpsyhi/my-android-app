
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    slint::slint! {
        import { GroupBox, Button, CheckBox, ComboBox, Slider, Switch } from "std-widgets.slint";
        export component MainWindow inherits Window {
            GroupBox {
                VerticalLayout {
                    txt := Text {
                        text: "Hello, World!";
                        color: purple; 
                        font-size: 32px;
                        font-family: "Verdana";
                    }
                    slider-val := Text { text: "0"; }
                    txt-input := TextInput {
                        font-size: 32px;
                        font-family: "Verdana";
                        wrap: TextWrap.word-wrap;
                    }
                    ComboBox { model: [ "Male", "Female" ];}
                    Switch { text: "Switch"; toggled => {} }
                    slider := Slider { released => { slider-val.text = self.value }}
                    CheckBox { text: "Agree with the terms and angrements."; }
                    Button {
                        text: "Click Me!";
                        clicked => {
                            txt.text = txt-input.text;
                        }
                    }
                }
            }
        }
    }
    MainWindow::new().unwrap().run().unwrap();
}