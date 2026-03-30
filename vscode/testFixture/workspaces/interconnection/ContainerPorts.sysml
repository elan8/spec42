package IT {
    port def HdmiPort;
    port def PowerPort;

    part def Motherboard;

    part def Laptop {
        port hdmi : HdmiPort;
        port power : PowerPort;
        part motherboard : Motherboard;
    }

    part def Monitor {
        port hdmi : HdmiPort;
        port power : PowerPort;
    }

    part def Socket {
        port power : PowerPort;
    }

    part office {
        part laptop : Laptop;
        part monitor : Monitor;
        part socket : Socket;

        connect laptop.hdmi to monitor.hdmi;
        connect socket.power to laptop.power;
        connect socket.power to monitor.power;
    }
}
