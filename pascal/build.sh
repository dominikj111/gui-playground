#!/bin/bash

echo "Building fpGUI Example..."
echo

# Check if fpGUI exists
if [ ! -d "third_party/fpGUI" ]; then
    echo "❌ fpGUI directory not found!"
    echo ""
    echo "Please download fpGUI first:"
    echo "  mkdir -p third_party"
    echo "  cd third_party"
    echo "  git clone https://github.com/graemeg/fpGUI.git"
    echo ""
    exit 1
fi

# Compile
fpc -Fu./third_party/fpGUI/src \
    -Fu./third_party/fpGUI/src/corelib \
    -Fu./third_party/fpGUI/src/corelib/x11 \
    -Fu./third_party/fpGUI/src/gui \
    -Fu./third_party/fpGUI/src/3rdparty/regex \
    -Fi./third_party/fpGUI/src/corelib \
    -Fi./third_party/fpGUI/src/corelib/x11 \
    -Fi./third_party/fpGUI/src \
    -dX11 \
    fpgui_example.pas

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "Run with: ./fpgui_example"
    echo ""
    echo "Features:"
    echo "  - Real GUI window with buttons and text input"
    echo "  - 2D drawing canvas with shapes"
    echo "  - Pure Pascal (no C dependencies)"
    echo ""
else
    echo ""
    echo "❌ Build failed"
    echo ""
    echo "Make sure you have installed:"
    echo "  sudo apt install libx11-dev libxft-dev"
    echo ""
    exit 1
fi
