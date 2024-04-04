


PImage bg;
PImage desktop;

PGraphics result;

// 57 2217
float h, v1, v2;

void setup() {
  size(0, 0, P2D);
  
  bg = loadImage("/home/io/uni/ingegneria del software/BeeLive/Deliverable-1/Images/dell-xps-15-341985051.png");
  desktop = loadImage("/home/io/uni/ingegneria del software/BeeLive/Deliverable-1/Images/Desktop.png");
  
  h = desktop.width / (5193.0 / 677 - 2);
  v1 = desktop.height / (2217.0 - 57) * 57;
  v2 = desktop.height / (2217.0 - 57) * (2643 - 2217);
  
  
  println(h);
  
  windowResize(1920 + int(h + h), 1012 + int(v1 + v2));
  
  result = createGraphics(width, height, P2D);
  result.beginDraw();
  
  result.image(desktop, h, v1, width - h - h, height - v1 - v2);
  result.image(bg, 0, 0, width, height);
  
  result.endDraw();
  
  result.save("/home/io/uni/ingegneria del software/BeeLive/Deliverable-1/Images/DesktopMockup.png");
}

void draw() {
  image(result, 0, 0, width, height);
  
  delay(1000);
  
  
}
