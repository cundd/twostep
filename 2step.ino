#include <Arduino.h>

int trigger = 12;
int step_port1 = 2;
int step_port2 = 3;
int step_port3 = 4;
int step_port4 = 5;
int step_port5 = 6;
int step_port6 = 7;
int step_port7 = 8;
int step_port8 = 9;

int step_ports[8] = {
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
};

int step_out = 10;

int sequence = 0b11001010;
int step_counter = 0;
bool last_trigger_state = false;

void setup() {
    Serial.begin(9600);
    pinMode(LED_BUILTIN, OUTPUT);
    pinMode(trigger, INPUT);

    pinMode(step_port1, OUTPUT);
    pinMode(step_port2, OUTPUT);
    pinMode(step_port3, OUTPUT);
    pinMode(step_port4, OUTPUT);
    pinMode(step_port5, OUTPUT);
    pinMode(step_port6, OUTPUT);
    pinMode(step_port7, OUTPUT);
    pinMode(step_port8, OUTPUT);
    pinMode(step_out, OUTPUT);

}

//int step_pointer = 0b10000000;

void trigger_step(int step) {
    set_all_off();
    digitalWrite(step, HIGH);
}

void set_all_off() {
    for (const auto port : step_ports) {
        digitalWrite(port, LOW);
    }
}

void loop() {
    bool trigger_state = digitalRead(trigger);
    Serial.println(trigger_state);
    if (trigger_state && last_trigger_state == false) {
        update();
    }
    last_trigger_state = trigger_state;

}

void update() {
    step_counter += 1;
    if (step_counter > 7) {
        step_counter = 0;
    }

    int step_pointer = 0b10000000 >> step_counter;
//    step_pointer = step_pointer >> 1;
//    if (step_pointer == 0) {
//        // print!("Roll ");
//        step_pointer = 0b10000000;
//    } else {
//        // print!("     ");
//    }
    prntBits(step_pointer);
    // print!("{:#010b}", step_pointer);
    // print!(" {:#020b}", step_pointer & sequence);
    set_all_off();

    digitalWrite(step_ports[step_counter], (step_pointer & sequence) == step_pointer);

    if ((step_pointer & sequence) == step_pointer) {
        digitalWrite(step_out, HIGH);
        digitalWrite(LED_BUILTIN, HIGH);
        Serial.print("!");
    } else {
        digitalWrite(step_out, LOW);
        digitalWrite(LED_BUILTIN, LOW);
    }
    Serial.println();
    delay(100);


//    delay(500);                       // wait for half a second
//    digitalWrite(LED_BUILTIN, trigger_state);    // turn the LED off
//    delay(500);
}

void prntBits(byte b) {
    for (int i = 7; i >= 0; i--) {
        Serial.print(bitRead(b, i));
    }
}
//int get_step_for_(byte b) {
//    for (int i = 7; i >= 0; i--) {
//        Serial.print(bitRead(b, i));
//    }
//}
