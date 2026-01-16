int cons2fpga = 0;  // serial byte from Console to FPGA
int fpga2cons = 0;  // serial byte from FPGA    to Console
int cnt = 0;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);//19200);
  Serial1.begin(115200);//19200);

  // initial tests: send: 0..f, 10x(a,5)
  // only output on FPGA, echo response not read!
  for (cnt=0; cnt<16; cnt++)
  { Serial1.write((cnt<10)?cnt+0x30:cnt-10+0x61);
    delay(500);
  }
  for (cnt=0; cnt<10; cnt++)
  { Serial1.write(0x61);
    delay(300);
    Serial1.write(0x35);
    delay(300);
  }
}

void loop() {
  // put your main code here, to run repeatedly:
  // read input from console, send to FPGA
  if (Serial.available() > 0)
  { cons2fpga = Serial.read();
    Serial1.write(cons2fpga);
  }

  // read input from FPGA, send to console
  if (Serial1.available() > 0)
  { fpga2cons = Serial1.read();
    Serial.write(fpga2cons);
  }
}
