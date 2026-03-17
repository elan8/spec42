package ConnectedBlocks {
    port def SignalPort;
    port def ControlPort;
    port def TelemetryPort;
    port def PowerPort;

    part def FlightController {
        port navIn : SignalPort;
        port imuIn : SignalPort;
        port telemetryOut : TelemetryPort;
        port gimbalCmd : ControlPort;
        port radioCmdIn : ControlPort;
        port pwr : PowerPort;
    }

    part def NavigationSensor {
        port gpsOut : SignalPort;
        port imuOut : SignalPort;
        port pwr : PowerPort;
    }

    part def RadioModule {
        port uplinkOut : ControlPort;
        port telemetryIn : TelemetryPort;
        port pwr : PowerPort;
    }

    part def Gimbal {
        port cmdIn : ControlPort;
        port statusOut : TelemetryPort;
        port pwr : PowerPort;
    }

    part def PayloadCamera {
        port statusOut : TelemetryPort;
        port pwr : PowerPort;
    }

    part def PowerDistribution {
        port fcPower : PowerPort;
        port sensorPower : PowerPort;
        port radioPower : PowerPort;
        port gimbalPower : PowerPort;
        port cameraPower : PowerPort;
    }

    part def System {
        part flightController : FlightController;
        part navigation : NavigationSensor;
        part radio : RadioModule;
        part gimbal : Gimbal;
        part camera : PayloadCamera;
        part power : PowerDistribution;

        connect navigation.gpsOut to flightController.navIn;
        connect navigation.imuOut to flightController.imuIn;
        connect radio.uplinkOut to flightController.radioCmdIn;
        connect flightController.gimbalCmd to gimbal.cmdIn;
        connect flightController.telemetryOut to radio.telemetryIn;
        connect gimbal.statusOut to radio.telemetryIn;
        connect camera.statusOut to radio.telemetryIn;

        connect power.fcPower to flightController.pwr;
        connect power.sensorPower to navigation.pwr;
        connect power.radioPower to radio.pwr;
        connect power.gimbalPower to gimbal.pwr;
        connect power.cameraPower to camera.pwr;
    }
}
